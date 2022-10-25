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
// enable and disable outputs
use crate::hal::gpio::OutputEnableOverride::Disable;
use crate::hal::gpio::OutputEnableOverride::Enable;

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
    let mut col0 = pins.gpio13.into_push_pull_output();
    let mut col1 = pins.gpio14.into_push_pull_output();
    let mut col2 = pins.gpio15.into_push_pull_output();
    let mut col3 = pins.gpio12.into_push_pull_output();
    let mut col4 = pins.gpio11.into_push_pull_output();
    let mut col5 = pins.gpio10.into_push_pull_output();
    let mut col6 = pins.gpio9.into_push_pull_output();
    let mut col7 = pins.gpio8.into_push_pull_output();
    let mut col8 = pins.gpio2.into_push_pull_output();
    let mut col9 = pins.gpio3.into_push_pull_output();
    let mut col10 = pins.gpio4.into_push_pull_output();
    let mut col11 = pins.gpio5.into_push_pull_output();
    let mut col12 = pins.gpio6.into_push_pull_output();
    let mut col13 = pins.gpio7.into_push_pull_output();

    // key state - 1 is pressed, 0 is released
    // recording the key state should be separate from usb polling so that they can work independently
    let mut pressed_keys: [[i32; 14]; 5] = [
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    ];

    // set default state of col pins to disable
    // so we can cycle through each column to check rows
    col0.set_output_enable_override(Disable);
    col1.set_output_enable_override(Disable);
    col2.set_output_enable_override(Disable);
    col3.set_output_enable_override(Disable);
    col4.set_output_enable_override(Disable);
    col5.set_output_enable_override(Disable);
    col6.set_output_enable_override(Disable);
    col7.set_output_enable_override(Disable);
    col8.set_output_enable_override(Disable);
    col9.set_output_enable_override(Disable);
    col10.set_output_enable_override(Disable);
    col11.set_output_enable_override(Disable);
    col12.set_output_enable_override(Disable);
    col13.set_output_enable_override(Disable);

    // polling rate countdown
    let mut input_count_down = timer.count_down();
    input_count_down.start(1.millis()); // todo good polling time?

    let mut tick_count_down = timer.count_down();
    tick_count_down.start(1.millis());

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
        // send signal for this col
        col0.set_output_enable_override(Enable);
        col0.set_low().ok();
        // read the value and set the pressed_keys value if read
        for i in 0..5 {
            if row_pins[i].is_low().unwrap() {
                pressed_keys[i][0] = 1;
            } else {
                pressed_keys[i][0] = 0;
            }
        }
        // then disable
        col0.set_output_enable_override(Disable);

        // send signal for this col
        col1.set_output_enable_override(Enable);
        col1.set_low().ok();
        // read the value and set the pressed_keys value if read
        for i in 0..5 {
            if row_pins[i].is_low().unwrap() {
                pressed_keys[i][1] = 1;
            } else {
                pressed_keys[i][1] = 0;
            }
        }
        // then disable
        col1.set_output_enable_override(Disable);

        // etc etc ....
        col2.set_output_enable_override(Enable);
        col2.set_low().ok();
        for i in 0..5 {
            if row_pins[i].is_low().unwrap() {
                pressed_keys[i][2] = 1;
            } else {
                pressed_keys[i][2] = 0;
            }
        }
        col2.set_output_enable_override(Disable);

        // etc etc ....
        col3.set_output_enable_override(Enable);
        col3.set_low().ok();
        for i in 0..5 {
            if row_pins[i].is_low().unwrap() {
                pressed_keys[i][3] = 1;
            } else {
                pressed_keys[i][3] = 0;
            }
        }
        col3.set_output_enable_override(Disable);

        // etc etc ....
        col4.set_output_enable_override(Enable);
        col4.set_low().ok();
        for i in 0..5 {
            if row_pins[i].is_low().unwrap() {
                pressed_keys[i][4] = 1;
            } else {
                pressed_keys[i][4] = 0;
            }
        }
        col4.set_output_enable_override(Disable);

        // etc etc ....
        col5.set_output_enable_override(Enable);
        col5.set_low().ok();
        for i in 0..5 {
            if row_pins[i].is_low().unwrap() {
                pressed_keys[i][5] = 1;
            } else {
                pressed_keys[i][5] = 0;
            }
        }
        col5.set_output_enable_override(Disable);

        // etc etc ....
        col6.set_output_enable_override(Enable);
        col6.set_low().ok();
        for i in 0..5 {
            if row_pins[i].is_low().unwrap() {
                pressed_keys[i][6] = 1;
            } else {
                pressed_keys[i][6] = 0;
            }
        }
        col6.set_output_enable_override(Disable);

        // etc etc ....
        col7.set_output_enable_override(Enable);
        col7.set_low().ok();
        for i in 0..5 {
            if row_pins[i].is_low().unwrap() {
                pressed_keys[i][7] = 1;
            } else {
                pressed_keys[i][7] = 0;
            }
        }
        col7.set_output_enable_override(Disable);

        // etc etc ....
        col8.set_output_enable_override(Enable);
        col8.set_low().ok();
        for i in 0..5 {
            if row_pins[i].is_low().unwrap() {
                pressed_keys[i][8] = 1;
            } else {
                pressed_keys[i][8] = 0;
            }
        }
        col8.set_output_enable_override(Disable);

        // etc etc ....
        col9.set_output_enable_override(Enable);
        col9.set_low().ok();
        for i in 0..5 {
            if row_pins[i].is_low().unwrap() {
                pressed_keys[i][9] = 1;
            } else {
                pressed_keys[i][9] = 0;
            }
        }
        col9.set_output_enable_override(Disable);

        // etc etc ....
        col10.set_output_enable_override(Enable);
        col10.set_low().ok();
        for i in 0..5 {
            if row_pins[i].is_low().unwrap() {
                pressed_keys[i][10] = 1;
            } else {
                pressed_keys[i][10] = 0;
            }
        }
        col10.set_output_enable_override(Disable);

        // etc etc ....
        col11.set_output_enable_override(Enable);
        col11.set_low().ok();
        for i in 0..5 {
            if row_pins[i].is_low().unwrap() {
                pressed_keys[i][11] = 1;
            } else {
                pressed_keys[i][11] = 0;
            }
        }
        col11.set_output_enable_override(Disable);

        // etc etc ....
        col12.set_output_enable_override(Enable);
        col12.set_low().ok();
        for i in 0..5 {
            if row_pins[i].is_low().unwrap() {
                pressed_keys[i][12] = 1;
            } else {
                pressed_keys[i][12] = 0;
            }
        }
        col12.set_output_enable_override(Disable);

        // etc etc ....
        col13.set_output_enable_override(Enable);
        col13.set_low().ok();
        for i in 0..5 {
            if row_pins[i].is_low().unwrap() {
                pressed_keys[i][13] = 1;
            } else {
                pressed_keys[i][13] = 0;
            }
        }
        col13.set_output_enable_override(Disable);
    }
}

fn get_keys(keys: [[i32; 14]; 5]) -> [Keyboard; 2] {
    [
        if keys[0][0] == 1 {
            Keyboard::A
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[3][13] == 1 {
            Keyboard::B
        } else {
            Keyboard::NoEventIndicated
        },
        // todo list out rest of keys
        // todo make layers for fn key
        // todo implement rotary encoder logic and usb output
    ]
}
