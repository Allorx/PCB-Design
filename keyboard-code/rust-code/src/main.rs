// Aleksas Girenas 23/10/2022
// For controlling OrionsHands (a fully custom keyboard)

#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use rp2040_hal as hal;

use hal::pac;

use embedded_hal::digital::v2::OutputPin;
use embedded_time::fixed_point::FixedPoint;
use rp2040_hal::clocks::Clock;

// where to place the boot block
#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

// crystal freq
const XTAL_FREQ_HZ: u32 = 12_000_000u32;

// entry point after global variable initialisation
#[entry]
fn main() -> ! {
    // grab our peripherals
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    // create a watchdog
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);
    // Clock configuration
    let clocks = hal::clocks::init_clocks_and_plls(
        XTAL_FREQ_HZ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();
    // setting up delay
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().raw());
    // gpio control with single-cycle io block
    let sio = hal::Sio::new(pac.SIO);

    // set pins to default state
    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );
    // led pin (GPIO25) as output
    let mut led_pin = pins.gpio25.into_push_pull_output();
    // main loop
    loop {
        led_pin.set_high().unwrap();
        delay.delay_ms(500);
        led_pin.set_low().unwrap();
        delay.delay_ms(500);
    }
}
