#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use panic_halt as _;

//delay
use nrf52833_hal::Timer;

use rtt_target::rprintln;
use rtt_target::rtt_init_print;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let p = nrf52833_hal::pac::Peripherals::take().unwrap();
    let mut timer = Timer::new(p.TIMER0);

    loop {
        rprintln!("Hello world");
        timer.delay_ms(1000_u32);
    }
}
