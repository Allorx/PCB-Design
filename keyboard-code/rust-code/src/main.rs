// Aleksas Girenas 23/10/2022
// For controlling OrionsHands (a fully custom keyboard)
// core code taken from https://github.com/dlkj/usbd-human-interface-device/blob/main/examples/src/bin/keyboard_nkro.rs

#![no_std]
#![no_main]

use bsp::entry;
use bsp::hal;
use core::convert::Infallible;
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
        .max_packet_size_0(8)
        .build();

    //GPIO pins
    // rows
    let keys: &[&dyn InputPin<Error = core::convert::Infallible>] = &[
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

    // todo set all pins to disable to start
    // send signal for this col
    col0.set_output_enable_override(Enable);
    col0.set_low().ok();
    // then disable
    col0.set_output_enable_override(Disable);
    // send signal for this col
    col1.set_output_enable_override(Enable);
    col1.set_low().ok();
    // then disable
    col1.set_output_enable_override(Disable);
    // todo etc etc .... and put in the loop getting the keys

    let mut input_count_down = timer.count_down();
    input_count_down.start(2.millis());

    let mut tick_count_down = timer.count_down();
    tick_count_down.start(1.millis());

    loop {
        //Poll the keys
        if input_count_down.wait().is_ok() {
            let keys = get_keys(keys);

            match keyboard.interface().write_report(&keys) {
                Err(UsbHidError::WouldBlock) => {}
                Err(UsbHidError::Duplicate) => {}
                Ok(_) => {}
                Err(e) => {
                    core::panic!("Failed to write keyboard report: {:?}", e)
                }
            };
        }

        //Tick once per ms
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
                Ok(_) => {}
            }
        }
    }
}

fn get_keys(keys: &[&dyn InputPin<Error = Infallible>]) -> [Keyboard; 12] {
    [
        if keys[0].is_low().unwrap() {
            Keyboard::A
        } else {
            Keyboard::NoEventIndicated
        }, //Numlock
        if keys[1].is_low().unwrap() {
            Keyboard::B
        } else {
            Keyboard::NoEventIndicated
        }, //Up
        if keys[2].is_low().unwrap() {
            Keyboard::C
        } else {
            Keyboard::NoEventIndicated
        }, //F12
        if keys[3].is_low().unwrap() {
            Keyboard::D
        } else {
            Keyboard::NoEventIndicated
        }, //Left
        if keys[4].is_low().unwrap() {
            Keyboard::E
        } else {
            Keyboard::NoEventIndicated
        }, //Down
        if keys[5].is_low().unwrap() {
            Keyboard::F
        } else {
            Keyboard::NoEventIndicated
        }, //Right
        if keys[6].is_low().unwrap() {
            Keyboard::G
        } else {
            Keyboard::NoEventIndicated
        }, //A
        if keys[7].is_low().unwrap() {
            Keyboard::H
        } else {
            Keyboard::NoEventIndicated
        }, //B
        if keys[8].is_low().unwrap() {
            Keyboard::I
        } else {
            Keyboard::NoEventIndicated
        }, //C
        if keys[9].is_low().unwrap() {
            Keyboard::J
        } else {
            Keyboard::NoEventIndicated
        }, //LCtrl
        if keys[10].is_low().unwrap() {
            Keyboard::K
        } else {
            Keyboard::NoEventIndicated
        }, //LShift
        if keys[11].is_low().unwrap() {
            Keyboard::L
        } else {
            Keyboard::NoEventIndicated
        }, //Enter
    ]
}
