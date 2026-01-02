#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;

//rtt
use rtt_target::rprintln;
use rtt_target::rtt_init_print;

// delay
use embedded_hal::delay::DelayNs;
use nrf52833_hal::Delay;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let cp = cortex_m::Peripherals::take().unwrap();
    let mut delay = Delay::new(cp.SYST);

    loop {
        rprintln!("Hello world");
        delay.delay_ms(1000_u32);
    }
}
