#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;

use embedded_hal::digital::{InputPin, OutputPin};
use nrf52833_hal::{gpio, pac};

use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let p = pac::Peripherals::take().unwrap();
    let p0 = gpio::p0::Parts::new(p.P0);

    let _row_1 = p0.p0_21.into_push_pull_output(gpio::Level::High);
    let mut col_1 = p0.p0_28.into_push_pull_output(gpio::Level::Low);

    let mut button = p0.p0_14.into_pullup_input();

    loop {
        if button.is_low().unwrap() {
            rprintln!("Button On");
            col_1.set_low().unwrap();
        } else {
            col_1.set_high().unwrap();
            rprintln!("Button Off");
        }
    }
}
