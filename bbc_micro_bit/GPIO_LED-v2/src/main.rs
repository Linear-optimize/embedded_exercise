#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;

// rtt
use rtt_target::{rprint, rtt_init_print};

//digital & delay

use embedded_hal::delay::DelayNs;

use embedded_hal::digital::OutputPin;

use nrf52833_hal as hal;
use nrf52833_hal::Delay;
use nrf52833_hal::gpio::Level::{High, Low};
#[entry]
fn main() -> ! {
    rtt_init_print!();
    let p = hal::pac::Peripherals::take().unwrap();
    let p0 = hal::gpio::p0::Parts::new(p.P0);

    let _row1 = p0.p0_21.into_push_pull_output(High);

    let mut col1 = p0.p0_28.into_push_pull_output(Low);

    let cp = cortex_m::Peripherals::take().unwrap();
    let mut delay = Delay::new(cp.SYST);

    loop {
        col1.set_low().unwrap();
        rprint!("LED row1 cols 1 ON\n");
        delay.delay_ms(1000_u32);
        col1.set_high().unwrap();
        rprint!("LED row1 cols 1 ON\n");
        delay.delay_ms(1000_u32);
    }
}
