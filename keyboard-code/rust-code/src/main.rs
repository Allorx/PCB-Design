// Aleksas Girenas 23/10/2022
// For controlling OrionsHands (a fully custom keyboard)
// inspired by https://github.com/dlkj/usbd-human-interface-device/blob/main/examples/src/bin/keyboard_nkro.rs
// somewhat poorly written as it is my first time working with Rust, microcontrollers and hastily written (jumping straight into the deep end - some might say a lil rusty) :D

#![no_std]
#![no_main]

use bsp::entry;
use bsp::hal;
//use bsp::hal::gpio::dynpin;
use defmt::*;
use defmt_rtt as _;
use embedded_hal::digital::v2::*;
use embedded_hal::prelude::*;
use fugit::ExtU32;
use hal::pac;
use panic_probe as _;
use usb_device::class_prelude::*;
use usb_device::prelude::*;
use usbd_human_interface_device::page::Keyboard;
use usbd_human_interface_device::prelude::*;

use rp2040_hal::gpio::DynPin;
use rp_pico as bsp;

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();

    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);
    let clocks = hal::clocks::init_clocks_and_plls(
        bsp::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let timer = hal::Timer::new(pac.TIMER, &mut pac.RESETS);

    let sio = hal::Sio::new(pac.SIO);
    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    info!("Starting");

    //USB
    let usb_bus = UsbBusAllocator::new(hal::usb::UsbBus::new(
        pac.USBCTRL_REGS,
        pac.USBCTRL_DPRAM,
        clocks.usb_clock,
        true,
        &mut pac.RESETS,
    ));

    let mut keyboard = UsbHidClassBuilder::new()
        .add_interface(
            usbd_human_interface_device::device::keyboard::NKROBootKeyboardInterface::default_config(),
        )
        .build(&usb_bus);

    //https://pid.codes
    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x1209, 0x0001))
        .manufacturer("Orions Hands")
        .product("Orions Hands")
        .serial_number("291020221639") // using date + time
        .max_packet_size_0(32) // todo check if works and needed?
        .build();

    //GPIO pins
    // rows
    let row_pins: &[&dyn InputPin<Error = core::convert::Infallible>] = &[
        &pins.gpio20.into_pull_up_input(),
        &pins.gpio19.into_pull_up_input(),
        &pins.gpio18.into_pull_up_input(),
        &pins.gpio17.into_pull_up_input(),
        &pins.gpio16.into_pull_up_input(),
    ];

    // cols
    // so we can cycle through each column to check rows first turn them into dynpins then put in array
    let col0: DynPin = pins.gpio13.into();
    let col1: DynPin = pins.gpio14.into();
    let col2: DynPin = pins.gpio15.into();
    let col3: DynPin = pins.gpio12.into();
    let col4: DynPin = pins.gpio11.into();
    let col5: DynPin = pins.gpio10.into();
    let col6: DynPin = pins.gpio9.into();
    let col7: DynPin = pins.gpio8.into();
    let col8: DynPin = pins.gpio2.into();
    let col9: DynPin = pins.gpio3.into();
    let col10: DynPin = pins.gpio4.into();
    let col11: DynPin = pins.gpio5.into();
    let col12: DynPin = pins.gpio6.into();
    let col13: DynPin = pins.gpio7.into();

    let mut col_pins = [
        col0, col1, col2, col3, col4, col5, col6, col7, col8, col9, col10, col11, col12, col13,
    ];
    // rotary encoder
    let rot_clk = &pins.gpio0.into_pull_up_input();
    let rot_dt = &pins.gpio1.into_pull_up_input();
    let mut rot_last_state = [0, 0];
    let mut rot_current_state = [0, 0];
    // set rot_last_state
    col_pins[13].into_push_pull_output();
    col_pins[13].set_low().ok();
    if rot_clk.is_low().unwrap() {
        rot_last_state[0] = 1;
    } else {
        rot_last_state[0] = 0;
    }
    if rot_dt.is_low().unwrap() {
        rot_last_state[1] = 1;
    } else {
        rot_last_state[1] = 0;
    }
    // set default state of col pins to input
    for i in 0..14 {
        col_pins[i].into_pull_up_input();
    }

    // key state - 1 is pressed, 0 is released
    // recording the key state should be separate from usb polling so that they can work independently
    let mut pressed_keys: [[i32; 14]; 5] = [
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    ];

    // key polling rate countdown
    //let mut key_input_count_down = timer.count_down();
    //key_input_count_down.start(500.micros());

    // usb polling rate countdown
    let mut input_count_down = timer.count_down();
    input_count_down.start(1.millis()); // todo good polling time?

    let mut tick_count_down = timer.count_down();
    tick_count_down.start(1.millis());

    loop {
        //write report every input_count_down
        if input_count_down.wait().is_ok() {
            if pressed_keys[4][11] == 1 {
                // fn key pressed
                let keys = get_fnkeys(pressed_keys);
                match keyboard.interface().write_report(&keys) {
                    Err(UsbHidError::WouldBlock) => {}
                    Err(UsbHidError::Duplicate) => {}
                    Ok(_) => {}
                    Err(e) => {
                        core::panic!("Failed to write keyboard report: {:?}", e)
                    }
                };
            } else {
                // fn key released
                let keys = get_keys(pressed_keys);
                match keyboard.interface().write_report(&keys) {
                    Err(UsbHidError::WouldBlock) => {}
                    Err(UsbHidError::Duplicate) => {}
                    Ok(_) => {}
                    Err(e) => {
                        core::panic!("Failed to write keyboard report: {:?}", e)
                    }
                };
            }
        }

        //tick every tick_count_down
        if tick_count_down.wait().is_ok() {
            match keyboard.interface().tick() {
                Err(UsbHidError::WouldBlock) => {}
                Ok(_) => {}
                Err(e) => {
                    core::panic!("Failed to process keyboard tick: {:?}", e)
                }
            };
        }

        if usb_dev.poll(&mut [&mut keyboard]) {
            match keyboard.interface().read_report() {
                Err(UsbError::WouldBlock) => {
                    //do nothing
                }
                Err(e) => {
                    core::panic!("Failed to read keyboard report: {:?}", e)
                }
                Ok(_) => {
                    // do nothing
                }
            }
        }

        //poll the keys
        // send signal for this col;
        for i in 0..14 {
            // ? set wait till next loop or pio
            col_pins[i].into_push_pull_output();
            col_pins[i].set_low().ok();
            // read the value and set the pressed_keys value if read
            for j in 0..5 {
                if row_pins[j].is_low().unwrap() {
                    pressed_keys[j][i] = 1;
                } else {
                    pressed_keys[j][i] = 0;
                }
            }
            // for rotary encoder check if at col13
            if i == 13 {
                //poll the rotary encoder
                // read values clk and dt and compare to last state
                if rot_clk.is_low().unwrap() {
                    rot_current_state[0] = 1;
                } else {
                    rot_current_state[0] = 0;
                }
                if rot_dt.is_low().unwrap() {
                    rot_current_state[1] = 1;
                } else {
                    rot_current_state[1] = 0;
                }
                // compare current to last state and assign to an unused pressed_keys
                if (rot_last_state[0] == 0
                    && rot_last_state[1] == 0
                    && rot_current_state[0] == 1
                    && rot_current_state[1] == 0)
                    || (rot_last_state[0] == 1
                        && rot_last_state[1] == 1
                        && rot_current_state[0] == 0
                        && rot_current_state[1] == 1)
                {
                    // clockwise
                    pressed_keys[4][4] = 1;
                } else if (rot_last_state[0] == 0
                    && rot_last_state[1] == 0
                    && rot_current_state[0] == 0
                    && rot_current_state[1] == 1)
                    || (rot_last_state[0] == 1
                        && rot_last_state[1] == 1
                        && rot_current_state[0] == 1
                        && rot_current_state[1] == 0)
                {
                    // anticlockwise
                    pressed_keys[4][4] = -1;
                } else {
                    // nothing
                    pressed_keys[4][4] = 0;
                }
                // setup for next
                rot_last_state = rot_current_state;
            }
            // then disable
            col_pins[i].into_pull_up_input();
        }
    }
}

// 64 keys excluding fn key - 65th key is volume
fn get_keys(keys: [[i32; 14]; 5]) -> [Keyboard; 65] {
    [
        if keys[0][0] == 1 {
            Keyboard::Escape
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[0][1] == 1 {
            Keyboard::Keyboard1
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[0][2] == 1 {
            Keyboard::Keyboard2
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[0][3] == 1 {
            Keyboard::Keyboard3
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[0][4] == 1 {
            Keyboard::Keyboard4
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[0][5] == 1 {
            Keyboard::Keyboard5
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[0][6] == 1 {
            Keyboard::Keyboard6
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[0][7] == 1 {
            Keyboard::Keyboard7
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[0][8] == 1 {
            Keyboard::Keyboard8
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[0][9] == 1 {
            Keyboard::Keyboard9
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[0][10] == 1 {
            Keyboard::Keyboard0
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[0][11] == 1 {
            Keyboard::Minus
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[0][12] == 1 {
            Keyboard::Equal
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[0][13] == 1 {
            Keyboard::DeleteBackspace
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[1][0] == 1 {
            Keyboard::Tab
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[1][1] == 1 {
            Keyboard::Q
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[1][2] == 1 {
            Keyboard::W
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[1][3] == 1 {
            Keyboard::E
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[1][4] == 1 {
            Keyboard::R
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[1][5] == 1 {
            Keyboard::T
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[1][6] == 1 {
            Keyboard::Y
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[1][7] == 1 {
            Keyboard::U
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[1][8] == 1 {
            Keyboard::I
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[1][9] == 1 {
            Keyboard::O
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[1][10] == 1 {
            Keyboard::P
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[1][11] == 1 {
            Keyboard::LeftBrace
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[1][12] == 1 {
            Keyboard::RightBrace
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[1][13] == 1 {
            Keyboard::Pause
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[2][0] == 1 {
            Keyboard::CapsLock
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[2][1] == 1 {
            Keyboard::A
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[2][2] == 1 {
            Keyboard::S
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[2][3] == 1 {
            Keyboard::D
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[2][4] == 1 {
            Keyboard::F
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[2][5] == 1 {
            Keyboard::G
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[2][6] == 1 {
            Keyboard::H
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[2][7] == 1 {
            Keyboard::J
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[2][8] == 1 {
            Keyboard::K
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[2][9] == 1 {
            Keyboard::L
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[2][10] == 1 {
            Keyboard::Semicolon
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[2][11] == 1 {
            Keyboard::Apostrophe
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[2][12] == 1 {
            Keyboard::ReturnEnter
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[2][13] == 1 {
            Keyboard::DeleteForward
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[3][0] == 1 {
            Keyboard::LeftShift
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[3][1] == 1 {
            Keyboard::NonUSBackslash
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[3][2] == 1 {
            Keyboard::Z
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[3][3] == 1 {
            Keyboard::X
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[3][4] == 1 {
            Keyboard::C
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[3][5] == 1 {
            Keyboard::V
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[3][6] == 1 {
            Keyboard::B
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[3][7] == 1 {
            Keyboard::N
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[3][8] == 1 {
            Keyboard::M
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[3][9] == 1 {
            Keyboard::Comma
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[3][10] == 1 {
            Keyboard::Dot
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[3][11] == 1 {
            Keyboard::ForwardSlash
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[3][12] == 1 {
            Keyboard::NonUSHash
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[3][13] == 1 {
            Keyboard::UpArrow
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[4][0] == 1 {
            Keyboard::LeftControl
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[4][1] == 1 {
            Keyboard::LeftGUI
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[4][2] == 1 {
            Keyboard::LeftAlt
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[4][6] == 1 {
            Keyboard::Space
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[4][9] == 1 {
            Keyboard::RightAlt
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[4][11] == 1 {
            Keyboard::LeftArrow
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[4][12] == 1 {
            Keyboard::DownArrow
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[4][13] == 1 {
            Keyboard::RightArrow
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[4][4] == 1 {
            Keyboard::VolumeUp
        } else if keys[4][4] == -1 {
            Keyboard::VolumeDown
        } else {
            Keyboard::NoEventIndicated
        },
    ]
}

// 64 keys excluding fn key - 65th key is volume
fn get_fnkeys(keys: [[i32; 14]; 5]) -> [Keyboard; 65] {
    [
        if keys[0][0] == 1 {
            Keyboard::Escape
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[0][1] == 1 {
            Keyboard::F1
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[0][2] == 1 {
            Keyboard::F2
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[0][3] == 1 {
            Keyboard::F3
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[0][4] == 1 {
            Keyboard::F4
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[0][5] == 1 {
            Keyboard::F5
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[0][6] == 1 {
            Keyboard::F6
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[0][7] == 1 {
            Keyboard::F7
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[0][8] == 1 {
            Keyboard::F8
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[0][9] == 1 {
            Keyboard::F9
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[0][10] == 1 {
            Keyboard::F10
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[0][11] == 1 {
            Keyboard::F11
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[0][12] == 1 {
            Keyboard::F12
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[0][13] == 1 {
            Keyboard::DeleteBackspace
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[1][0] == 1 {
            Keyboard::Tab
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[1][1] == 1 {
            Keyboard::Q
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[1][2] == 1 {
            Keyboard::W
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[1][3] == 1 {
            Keyboard::E
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[1][4] == 1 {
            Keyboard::R
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[1][5] == 1 {
            Keyboard::T
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[1][6] == 1 {
            Keyboard::Y
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[1][7] == 1 {
            Keyboard::U
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[1][8] == 1 {
            Keyboard::I
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[1][9] == 1 {
            Keyboard::O
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[1][10] == 1 {
            Keyboard::P
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[1][11] == 1 {
            Keyboard::LeftBrace
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[1][12] == 1 {
            Keyboard::RightBrace
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[1][13] == 1 {
            Keyboard::Pause
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[2][0] == 1 {
            Keyboard::CapsLock
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[2][1] == 1 {
            Keyboard::A
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[2][2] == 1 {
            Keyboard::S
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[2][3] == 1 {
            Keyboard::D
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[2][4] == 1 {
            Keyboard::F
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[2][5] == 1 {
            Keyboard::G
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[2][6] == 1 {
            Keyboard::H
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[2][7] == 1 {
            Keyboard::J
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[2][8] == 1 {
            Keyboard::K
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[2][9] == 1 {
            Keyboard::L
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[2][10] == 1 {
            Keyboard::Semicolon
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[2][11] == 1 {
            Keyboard::Apostrophe
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[2][12] == 1 {
            Keyboard::ReturnEnter
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[2][13] == 1 {
            Keyboard::DeleteForward
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[3][0] == 1 {
            Keyboard::LeftShift
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[3][1] == 1 {
            Keyboard::NonUSBackslash
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[3][2] == 1 {
            Keyboard::Z
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[3][3] == 1 {
            Keyboard::X
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[3][4] == 1 {
            Keyboard::C
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[3][5] == 1 {
            Keyboard::V
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[3][6] == 1 {
            Keyboard::B
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[3][7] == 1 {
            Keyboard::N
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[3][8] == 1 {
            Keyboard::M
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[3][9] == 1 {
            Keyboard::Comma
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[3][10] == 1 {
            Keyboard::Dot
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[3][11] == 1 {
            Keyboard::ForwardSlash
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[3][12] == 1 {
            Keyboard::NonUSHash
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[3][13] == 1 {
            Keyboard::UpArrow
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[4][0] == 1 {
            Keyboard::LeftControl
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[4][1] == 1 {
            Keyboard::LeftGUI
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[4][2] == 1 {
            Keyboard::LeftAlt
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[4][6] == 1 {
            Keyboard::Space
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[4][9] == 1 {
            Keyboard::RightAlt
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[4][11] == 1 {
            Keyboard::LeftArrow
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[4][12] == 1 {
            Keyboard::DownArrow
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[4][13] == 1 {
            Keyboard::RightArrow
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[4][4] == 1 {
            Keyboard::VolumeUp
        } else if keys[4][4] == -1 {
            Keyboard::VolumeDown
        } else {
            Keyboard::NoEventIndicated
        },
    ]
}

// todo usb over bluetooth?
// todo still need to check keycodes for certain keys or add to them - might need to fork and add the rest from usbd-human-interface-device
