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
};
// display
use display_interface_i2c::I2CInterface;
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, PrimitiveStyle, Rectangle, Triangle},
    text::{Alignment, Text},
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
const DISPLAY_ON: u32 = 0xDE;
const DISPLAY_OFF: u32 = 0xDD;
const CAPS_ON: u32 = 0xCC;
const CAPS_OFF: u32 = 0xCD;

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

    let mut sio = hal::Sio::new(pac.SIO);
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
        .with_rotation(DisplayRotation::Rotate90)
        .connect(i2c_interface)
        .into();
    disp.reset(&mut reset, &mut delay).unwrap();
    disp.init().unwrap();

    // drawing variables
    let mut caps_on = false;

    let disp_dim = disp.get_dimensions();
    let circle_rad: i32 = 5;
    let circle_dim = 10;
    let circle_start = Point::new((disp_dim.0 / 2).into(), (disp_dim.1 / 2).into());
    let mut circle_velocity = Point::new(1, -1);
    let mut circle = Circle::with_center(circle_start, circle_dim);

    let caps_block_start = Point::new((disp_dim.0 / 2 - 5).into(), (disp_dim.1 / 2 + 1).into());
    let mut caps_velocity = Point::new(0, -1);
    let caps_max_pos = 9;
    let mut caps_y_pos = 0;
    let mut caps_arrow = Triangle::new(
        Point::new((disp_dim.0 / 2 - 9).into(), (disp_dim.1 / 2).into()),
        Point::new((disp_dim.0 / 2).into(), (disp_dim.1 / 2 - 9).into()),
        Point::new((disp_dim.0 / 2 + 9).into(), (disp_dim.1 / 2).into()),
    )
    .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 2));
    let mut caps_block = Rectangle::new(caps_block_start, Size::new(11, 10))
        .into_styled(PrimitiveStyle::with_fill(BinaryColor::On));
    let text_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);
    let caps_text =
        Text::with_alignment("Locked", Point::new(31, 100), text_style, Alignment::Center);

    loop {
        // todo - add more circles/shapes different sizes with some binarycolor::on and some off
        // ? draw to display
        disp.clear();
        if !caps_on {
            // animation
            circle
                .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
                .draw(&mut disp)
                .unwrap();
            if (circle.center().x - circle_rad) < 1
                || (circle.center().x + circle_rad) + 1 >= disp_dim.0.into()
            {
                circle_velocity.x *= -1;
            }
            if (circle.center().y - circle_rad) < 1
                || (circle.center().y + circle_rad) + 1 >= disp_dim.1.into()
            {
                circle_velocity.y *= -1;
            }
            circle.translate_mut(circle_velocity);
        } else {
            //? draw caps on
            caps_block.draw(&mut disp).unwrap();
            caps_arrow.draw(&mut disp).unwrap();
            caps_text.draw(&mut disp).unwrap();
            caps_y_pos += caps_velocity.y;
            if caps_y_pos > 0 {
                caps_velocity.y *= -1;
            } else if caps_y_pos < -caps_max_pos {
                caps_velocity.y *= -1;
            }
            caps_arrow.translate_mut(caps_velocity);
            caps_block.translate_mut(caps_velocity);
        }
        disp.flush().unwrap();

        // ? read fifo
        if sio.fifo.is_read_ready() {
            let fifo_read = sio.fifo.read();
            if fifo_read == Some(DISPLAY_OFF) {
                disp.display_on(false).unwrap();
            } else if fifo_read == Some(DISPLAY_ON) {
                disp.display_on(true).unwrap();
            } else if fifo_read == Some(CAPS_ON) {
                caps_on = true;
            } else if fifo_read == Some(CAPS_OFF) {
                caps_on = false;
            }
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
        .serial_number("260120231843") // using date + time (ddmmyyyyhhmm)
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
    let mut display_turn_on = true;
    let mut display_toggled = false;
    let display_on_time = 5.minutes();
    let mut display_off_timer = timer.count_down();
    display_off_timer.start(display_on_time);

    // capslock
    let mut caps_on = false;
    let mut caps_toggled = false;

    loop {
        // ? toggle on/off display if keyboard inactive for some time
        // checking keyboard activity
        let mut keyboard_activity = 0;
        for i in 0..14 {
            for j in 0..5 {
                keyboard_activity += pressed_keys[j][i];
            }
        }
        keyboard_activity += rot_rotation_dir.pow(2);
        // reset
        if keyboard_activity > 0 {
            display_off_timer.start(display_on_time);
            display_turn_on = true;
        }
        // send message
        if !display_toggled
            && keyboard_activity == 0
            && display_off_timer.wait().is_ok()
            && sio.fifo.is_write_ready()
        {
            // timer ran out
            display_off_timer.cancel().unwrap();
            display_turn_on = false;
            display_toggled = true;
            sio.fifo.write(DISPLAY_OFF);
        } else if display_turn_on && display_toggled && sio.fifo.is_write_ready() {
            // reset
            display_toggled = false;
            sio.fifo.write(DISPLAY_ON);
        }

        // ? toggle when caps_on
        // send message
        if !caps_toggled && caps_on && sio.fifo.is_write_ready() {
            caps_toggled = true;
            sio.fifo.write(CAPS_ON);
        } else if !caps_on && caps_toggled && sio.fifo.is_write_ready() {
            // reset
            caps_toggled = false;
            sio.fifo.write(CAPS_OFF);
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
                Ok(caps_lock) => {
                    caps_on = caps_lock.caps_lock;
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
