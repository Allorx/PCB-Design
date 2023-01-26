// Aleksas Girenas 23/10/2022
// For controlling OrionsHands (a fully custom keyboard)
// inspired by https://github.com/dlkj/usbd-human-interface-device/blob/main/examples/src/bin/keyboard_nkro.rs
// somewhat poorly written as it is my first time working with Rust and microcontrollers (jumping straight into the deep end - some might say a lil rusty) :D

#![no_std]
#![no_main]

// core
use cortex_m::delay;
use cortex_m_rt::{entry, exception, ExceptionFrame};
use embedded_hal::digital::v2::*;
use embedded_hal::prelude::*;
use embedded_hal::timer::Cancel;
use fugit::{ExtU32, RateExtU32};
use panic_halt as _;
use rp2040_hal::gpio::DynPin;
use rp2040_hal::multicore::{Multicore, Stack};
use rp_pico::{
    hal,
    hal::clocks::{Clock, SystemClock},
    hal::pac,
    hal::sio::Spinlock0,
};
// display
use display_interface_i2c::I2CInterface;
use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, PrimitiveStyle},
};
use ssd1309::{prelude::*, Builder};
// usb hid
use usb_device::{class_prelude::*, prelude::*};
use usbd_human_interface_device::device::consumer::{
    ConsumerControlInterface, MultipleConsumerReport,
};
use usbd_human_interface_device::device::keyboard::NKROBootKeyboardInterface;
use usbd_human_interface_device::page::Consumer;
use usbd_human_interface_device::prelude::*;

// src
pub mod consumer;
pub mod keys;

// declarations
static mut CORE1_STACK: Stack<4096> = Stack::new();
static mut DISPLAY_ON: u32 = 1;

// ? implementing exception frame handling
#[exception]
unsafe fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}

// ? core1 - used for external display
fn core1_task(sys_clock: &SystemClock) -> ! {
    // initialisation
    let mut pac = unsafe { pac::Peripherals::steal() };
    let core = unsafe { pac::CorePeripherals::steal() };

    let sio = hal::Sio::new(pac.SIO);
    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // ? create i2c drive and display
    // configure two pins as being I2C, not GPIO
    let sda_pin = pins.gpio26.into_mode::<hal::gpio::FunctionI2C>(); // sda = din
    let scl_pin = pins.gpio27.into_mode::<hal::gpio::FunctionI2C>(); // scl = clk

    let mut reset = pins.gpio28.into_push_pull_output(); // reset pin
    let mut delay = delay::Delay::new(core.SYST, sys_clock.freq().to_Hz()); // delay for reset

    let i2c = hal::I2C::i2c1(
        pac.I2C1,
        sda_pin,
        scl_pin,
        400.kHz(),
        &mut pac.RESETS,
        sys_clock.freq(),
    );
    let i2c_interface = I2CInterface::new(i2c, 0x3D, 0x40);
    let mut disp: GraphicsMode<_> = Builder::new()
        .with_rotation(DisplayRotation::Rotate270)
        .connect(i2c_interface)
        .into();
    disp.reset(&mut reset, &mut delay).unwrap();
    disp.init().unwrap();

    // drawing variables
    let disp_dim = disp.get_dimensions();
    let circle_rad: i32 = 5;
    let circle_dim = 10;
    let circle_start = Point::new((disp_dim.0 / 2).into(), (disp_dim.1 / 2).into());
    let mut velocity = Point::new(1, 1);
    let mut circle = Circle::with_center(circle_start, circle_dim);

    let mut display_on = true;

    loop {
        // todo - show when caps lock on
        // ? draw to display
        disp.clear();
        circle
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
            .draw(&mut disp)
            .unwrap();
        if (circle.center().x - circle_rad) < 1
            || (circle.center().x + circle_rad) + 1 >= disp_dim.0.into()
        {
            velocity.x *= -1;
        }
        if (circle.center().y - circle_rad) < 1
            || (circle.center().y + circle_rad) + 1 >= disp_dim.1.into()
        {
            velocity.y *= -1;
        }
        circle.translate_mut(velocity);
        disp.flush().unwrap();

        // toggle on/off display timed by core0 through DISPLAY_ON variable
        let _lock = Spinlock0::claim();
        if unsafe { DISPLAY_ON == 0 } && display_on {
            disp.display_on(false).unwrap();
            display_on = false;
        } else if unsafe { DISPLAY_ON == 1 } && !display_on {
            disp.display_on(true).unwrap();
            display_on = true;
        }
    }
}

// ? core0 and entry point
#[entry]
fn main() -> ! {
    // initialisation
    let mut pac = pac::Peripherals::take().unwrap();
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);
    let clocks = hal::clocks::init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
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
    let mut sio = hal::Sio::new(pac.SIO);

    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // ? initialise other core
    let mut mc = Multicore::new(&mut pac.PSM, &mut pac.PPB, &mut sio.fifo);
    let cores = mc.cores();
    let core1 = &mut cores[1];
    let _test = core1.spawn(unsafe { &mut CORE1_STACK.mem }, move || {
        core1_task(&clocks.system_clock)
    });

    // ? USB set up
    let usb_bus = UsbBusAllocator::new(hal::usb::UsbBus::new(
        pac.USBCTRL_REGS,
        pac.USBCTRL_DPRAM,
        clocks.usb_clock,
        true,
        &mut pac.RESETS,
    ));

    let mut composite = UsbHidClassBuilder::new()
        .add_interface(
            usbd_human_interface_device::device::keyboard::NKROBootKeyboardInterface::default_config(),
        )
        .add_interface(usbd_human_interface_device::device::consumer::ConsumerControlInterface::default_config())
        .build(&usb_bus);

    // ? https://pid.codes
    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x1209, 0x6E6E)) //0x0001 - testing PID
        .manufacturer("Allorx")
        .product("Orions Hands")
        .serial_number("260120230337") // using date + time (ddmmyyyyhhmm)
        .max_packet_size_0(32)
        .build();

    // ? GPIO pin and variable set up
    // rows
    let row_pins: &[&dyn InputPin<Error = core::convert::Infallible>] = &[
        &pins.gpio20.into_pull_up_input(),
        &pins.gpio19.into_pull_up_input(),
        &pins.gpio18.into_pull_up_input(),
        &pins.gpio17.into_pull_up_input(),
        &pins.gpio16.into_pull_up_input(),
    ];

    // cols
    // so we can cycle through each column to check rows, first turn them into dynpins then put in array
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
    // set default state of col pins to input
    for i in 0..14 {
        col_pins[i].into_pull_up_input();
    }

    // rotary encoder
    let rot_a = &pins.gpio0.into_pull_up_input();
    let rot_b = &pins.gpio1.into_pull_up_input();
    let mut rot_a_last_state = rot_a.is_low().unwrap();
    let mut rot_was_pressed = false;
    let mut rot_can_push = true;
    let mut rot_rotation_dir: i32 = 0;

    // key state - 1 is pressed, 0 is released
    // recording the key state should be separate from usb polling so that they can work independently
    let mut pressed_keys: [[i32; 14]; 5] = [
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    ];
    // key debounce flip flop
    let mut debounce_keys: [[i32; 14]; 5] = [
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    ];
    // debounce iterations until press or release is confirmed
    let confirmed_press = 4;

    // usb polling rate countdown
    let mut input_count_down = timer.count_down();
    input_count_down.start(1.millis());

    let mut tick_count_down = timer.count_down();
    tick_count_down.start(1.millis());
    // consumer polling rate countdown
    let mut consumer_poll = timer.count_down();
    consumer_poll.start(1.millis());
    let mut last_consumer_report = MultipleConsumerReport::default();

    // display
    let mut display_on = true;
    let mut display_toggled = false;
    let display_on_time = 1.minutes();
    let mut display_off_timer = timer.count_down();
    display_off_timer.start(display_on_time);

    loop {
        // ? toggle on/off display with DISPLAY_ON
        let mut toggle_display = 0;
        for i in 0..14 {
            for j in 0..5 {
                toggle_display += pressed_keys[j][i];
            }
        }
        toggle_display += rot_rotation_dir.pow(2);

        if toggle_display > 0 {
            display_off_timer.start(display_on_time);
            display_on = true;
        }

        if !display_toggled && toggle_display == 0 && display_off_timer.wait().is_ok() {
            display_off_timer.cancel().unwrap();
            display_on = false;
            display_toggled = true;
            let _lock = Spinlock0::claim();
            unsafe { DISPLAY_ON = 0 };
        } else if display_on && display_toggled {
            display_toggled = false;
            let _lock = Spinlock0::claim();
            unsafe { DISPLAY_ON = 1 };
        }

        // ? keyboard reporting
        // write report every input_count_down
        if input_count_down.wait().is_ok() {
            let keyboard = composite.interface::<NKROBootKeyboardInterface<'_, _>, _>();
            // 2 separate functions for fn key and normal, more memory intensive but less cpu?
            if pressed_keys[4][10] == 1 {
                // fn key pressed
                let keys = keys::get_fnkeys(pressed_keys);
                match keyboard.write_report(&keys) {
                    Err(UsbHidError::WouldBlock) => {}
                    Err(UsbHidError::Duplicate) => {}
                    Ok(_) => {}
                    Err(e) => {
                        core::panic!("Failed to write keyboard report: {:?}", e)
                    }
                };
            } else {
                // fn key released
                let keys = keys::get_keys(pressed_keys);
                match keyboard.write_report(&keys) {
                    Err(UsbHidError::WouldBlock) => {}
                    Err(UsbHidError::Duplicate) => {}
                    Ok(_) => {}
                    Err(e) => {
                        core::panic!("Failed to write keyboard report: {:?}", e)
                    }
                };
            }
        }

        // tick every tick_count_down
        if tick_count_down.wait().is_ok() {
            match composite
                .interface::<NKROBootKeyboardInterface<'_, _>, _>()
                .tick()
            {
                Err(UsbHidError::WouldBlock) => {}
                Ok(_) => {}
                Err(e) => {
                    core::panic!("Failed to process keyboard tick: {:?}", e)
                }
            };
        }

        if usb_dev.poll(&mut [&mut composite]) {
            let keyboard = composite.interface::<NKROBootKeyboardInterface<'_, _>, _>();
            match keyboard.read_report() {
                Err(UsbError::WouldBlock) => {}
                Err(e) => {
                    core::panic!("Failed to read keyboard report: {:?}", e)
                }
                Ok(_) => {
                    // do nothing
                    // can read report from host to eg turn on caps lock led
                }
            }
        }

        // ? consumer reporting
        // write report every consumer_poll
        if consumer_poll.wait().is_ok() {
            let codes = consumer::get_consumer(
                pressed_keys,
                rot_rotation_dir,
                rot_was_pressed,
                rot_can_push,
            );
            let consumer_report = MultipleConsumerReport {
                codes: [
                    codes[0],
                    Consumer::Unassigned, // can send more consumer codes with codes[1], codes[2], codes[3]
                    Consumer::Unassigned,
                    Consumer::Unassigned,
                ],
            };

            if last_consumer_report != consumer_report {
                let consumer = composite.interface::<ConsumerControlInterface<'_, _>, _>();
                match consumer.write_report(&consumer_report) {
                    Err(UsbError::WouldBlock) => {}
                    Ok(_) => {
                        last_consumer_report = consumer_report;
                    }
                    Err(e) => {
                        core::panic!("Failed to write consumer report: {:?}", e)
                    }
                };
            };
            // reset rotary encoder states
            rot_was_pressed = false;
            rot_rotation_dir = 0;
        }

        // ? poll the keys
        // send signal for this col;
        for i in 0..14 {
            col_pins[i].into_push_pull_output();
            col_pins[i].set_low().ok();
            // read the value and set the pressed_keys value if read and confirmed_press
            for j in 0..5 {
                if row_pins[j].is_low().unwrap() {
                    if debounce_keys[j][i] > confirmed_press {
                        pressed_keys[j][i] = 1;
                        // reset debounce
                        debounce_keys[j][i] = 0;
                    } else {
                        // increment debounce
                        debounce_keys[j][i] += 1;
                    }
                } else {
                    if debounce_keys[j][i] < -confirmed_press {
                        pressed_keys[j][i] = 0;
                        // reset debounce
                        debounce_keys[j][i] = 0;
                    } else {
                        // decrement debounce
                        debounce_keys[j][i] -= 1;
                    }
                }
            }
            // then disable
            col_pins[i].into_pull_up_input();
        }

        // check if play/pause key has been pressed and set that it was pressed
        if pressed_keys[1][13] == 1 && !rot_was_pressed {
            rot_was_pressed = true;
        }

        // ? poll the rotary encoder
        // read values a and b and compare to last state and assign to rot_rotation_dir
        if rot_a.is_low().unwrap() != rot_a_last_state {
            if rot_a.is_low().unwrap() {
                if rot_b.is_low().unwrap() {
                    // clockwise
                    rot_rotation_dir = 1;
                    // disable push - play/pause will not activate if the encoder has also been rotated before its release
                    // so we can have alternate pushed and rotated functionality without also activating play/pause after release.
                    rot_can_push = false;
                } else {
                    // anticlockwise
                    rot_rotation_dir = -1;
                    // disable push - play/pause will not activate if the encoder has also been rotated before its release
                    // so we can have alternate pushed and rotated functionality without also activating play/pause after release.
                    rot_can_push = false;
                }
            }
            // setup for next
            rot_a_last_state = rot_a.is_low().unwrap();
            // reset rot can push
            if !rot_was_pressed && !rot_can_push {
                rot_can_push = true;
            }
        }
    }
}
