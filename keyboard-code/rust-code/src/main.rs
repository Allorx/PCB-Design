// Aleksas Girenas 23/10/2022
// For controlling OrionsHands (a fully custom keyboard)
// inspired by https://github.com/dlkj/usbd-human-interface-device/blob/main/examples/src/bin/keyboard_nkro.rs
// somewhat poorly written as it is my first time working with Rust, microcontrollers and hastily written (jumping straight into the deep end - some might say a lil rusty) :D

#![no_std]
#![no_main]

use bsp::entry;
use bsp::hal;
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
        .serial_number("000001")
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
    // set default state of col pins to input
    // so we can cycle through each column to check rows first assign then put in array
    let mut col0 = pins.gpio13.into_pull_up_input();
    let mut col1 = pins.gpio14.into_pull_up_input();
    let mut col2 = pins.gpio15.into_pull_up_input();
    let mut col3 = pins.gpio12.into_pull_up_input();
    let mut col4 = pins.gpio11.into_pull_up_input();
    let mut col5 = pins.gpio10.into_pull_up_input();
    let mut col6 = pins.gpio9.into_pull_up_input();
    let mut col7 = pins.gpio8.into_pull_up_input();
    let mut col8 = pins.gpio2.into_pull_up_input();
    let mut col9 = pins.gpio3.into_pull_up_input();
    let mut col10 = pins.gpio4.into_pull_up_input();
    let mut col11 = pins.gpio5.into_pull_up_input();
    let mut col12 = pins.gpio6.into_pull_up_input();
    let mut col13 = pins.gpio7.into_pull_up_input();

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
    //key_input_count_down.start(500.micros()); // todo good polling time?

    // usb polling rate countdown
    let mut input_count_down = timer.count_down();
    input_count_down.start(1.millis()); // todo good polling time?

    let mut tick_count_down = timer.count_down();
    tick_count_down.start(500.micros());

    loop {
        //write report every input_count_down
        if input_count_down.wait().is_ok() {
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
                    // does nothing
                    // can put in logic for lighting up an led when capslock is pressed
                }
            }
        }

        //poll the keys
        // send signal for this col;
        let mut col0_out = col0.into_push_pull_output();
        col0_out.set_low().ok();
        // read the value and set the pressed_keys value if read
        for i in 0..5 {
            if row_pins[i].is_low().unwrap() {
                pressed_keys[i][0] = 1;
            } else {
                pressed_keys[i][0] = 0;
            }
        }
        // then disable
        col0 = col0_out.into_pull_up_input();
        // send signal for this col
        let mut col1_out = col1.into_push_pull_output();
        col1_out.set_low().ok();
        // read the value and set the pressed_keys value if read
        for i in 0..5 {
            if row_pins[i].is_low().unwrap() {
                pressed_keys[i][1] = 1;
            } else {
                pressed_keys[i][1] = 0;
            }
        }
        // then disable
        col1 = col1_out.into_pull_up_input();
        // etc etc ....
        let mut col2_out = col2.into_push_pull_output();
        col2_out.set_low().ok();
        for i in 0..5 {
            if row_pins[i].is_low().unwrap() {
                pressed_keys[i][2] = 1;
            } else {
                pressed_keys[i][2] = 0;
            }
        }
        col2 = col2_out.into_pull_up_input();
        // etc etc ....
        let mut col3_out = col3.into_push_pull_output();
        col3_out.set_low().ok();
        for i in 0..5 {
            if row_pins[i].is_low().unwrap() {
                pressed_keys[i][3] = 1;
            } else {
                pressed_keys[i][3] = 0;
            }
        }
        col3 = col3_out.into_pull_up_input();
        // etc etc ....
        let mut col4_out = col4.into_push_pull_output();
        col4_out.set_low().ok();
        for i in 0..5 {
            if row_pins[i].is_low().unwrap() {
                pressed_keys[i][4] = 1;
            } else {
                pressed_keys[i][4] = 0;
            }
        }
        col4 = col4_out.into_pull_up_input();
        // etc etc ....
        let mut col5_out = col5.into_push_pull_output();
        col5_out.set_low().ok();
        for i in 0..5 {
            if row_pins[i].is_low().unwrap() {
                pressed_keys[i][5] = 1;
            } else {
                pressed_keys[i][5] = 0;
            }
        }
        col5 = col5_out.into_pull_up_input();
        // etc etc ....
        let mut col6_out = col6.into_push_pull_output();
        col6_out.set_low().ok();
        for i in 0..5 {
            if row_pins[i].is_low().unwrap() {
                pressed_keys[i][6] = 1;
            } else {
                pressed_keys[i][6] = 0;
            }
        }
        col6 = col6_out.into_pull_up_input();
        // etc etc ....
        let mut col7_out = col7.into_push_pull_output();
        col7_out.set_low().ok();
        for i in 0..5 {
            if row_pins[i].is_low().unwrap() {
                pressed_keys[i][7] = 1;
            } else {
                pressed_keys[i][7] = 0;
            }
        }
        col7 = col7_out.into_pull_up_input();
        // etc etc ....
        let mut col8_out = col8.into_push_pull_output();
        col8_out.set_low().ok();
        for i in 0..5 {
            if row_pins[i].is_low().unwrap() {
                pressed_keys[i][8] = 1;
            } else {
                pressed_keys[i][8] = 0;
            }
        }
        col8 = col8_out.into_pull_up_input();
        // etc etc ....
        let mut col9_out = col9.into_push_pull_output();
        col9_out.set_low().ok();
        for i in 0..5 {
            if row_pins[i].is_low().unwrap() {
                pressed_keys[i][9] = 1;
            } else {
                pressed_keys[i][9] = 0;
            }
        }
        col9 = col9_out.into_pull_up_input();
        // etc etc ....
        let mut col10_out = col10.into_push_pull_output();
        col10_out.set_low().ok();
        for i in 0..5 {
            if row_pins[i].is_low().unwrap() {
                pressed_keys[i][10] = 1;
            } else {
                pressed_keys[i][10] = 0;
            }
        }
        col10 = col10_out.into_pull_up_input();
        // etc etc ....
        let mut col11_out = col11.into_push_pull_output();
        col11_out.set_low().ok();
        for i in 0..5 {
            if row_pins[i].is_low().unwrap() {
                pressed_keys[i][11] = 1;
            } else {
                pressed_keys[i][11] = 0;
            } 
        }
        col11 = col11_out.into_pull_up_input();
        // etc etc ....
        let mut col12_out = col12.into_push_pull_output();
        col12_out.set_low().ok();
        for i in 0..5 {
            if row_pins[i].is_low().unwrap() {
                pressed_keys[i][12] = 1;
            } else {
                pressed_keys[i][12] = 0;
            }
        }
        col12 = col12_out.into_pull_up_input();
        // etc etc ....
        let mut col13_out = col13.into_push_pull_output();
        col13_out.set_low().ok();
        for i in 0..5 {
            if row_pins[i].is_low().unwrap() {
                pressed_keys[i][13] = 1;
            } else {
                pressed_keys[i][13] = 0;
            }
        }
        col13 = col13_out.into_pull_up_input();
    }
}

fn get_keys(keys: [[i32; 14]; 5]) -> [Keyboard; 64] {
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
        //if keys[4][11] == 1 {
        //    Func pressed
        //} else {
        //    Func released
        //},
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
    ]
}

// todo make layers for fn key
// todo implement rotary encoder logic and usb output
// todo still need to check keycodes for certain keys or add to them - might need to fork and add the rest from usbd-human-interface-device
