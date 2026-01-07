#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;

use core::fmt::Write;
use embedded_hal::delay::DelayNs;
use hal::{gpio, uarte, uarte::Uarte};
use nrf52833_hal as hal;
use nrf52833_hal::delay::Delay;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let p = hal::pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();
    let mut delay = Delay::new(cp.SYST);

    let (uart0, cdc_pins) = {
        let p0 = gpio::p0::Parts::new(p.P0);
        let p1 = gpio::p1::Parts::new(p.P1);
        (
            p.UARTE0,
            uarte::Pins {
                txd: p0.p0_06.into_push_pull_output(gpio::Level::High).degrade(),
                rxd: p1.p1_08.into_floating_input().degrade(),
                cts: None,
                rts: None,
            },
        )
    };

    let mut uarte = Uarte::new(
        uart0,
        cdc_pins,
        uarte::Parity::EXCLUDED,
        uarte::Baudrate::BAUD115200,
    );

    loop {
        rprintln!("Serial begin!");
        write!(uarte, "Hello world\r\n").unwrap();
        delay.delay_ms(1000_u32);
    }
}
