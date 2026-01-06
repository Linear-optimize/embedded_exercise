#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;

use nrf52833_hal::{gpio, pac, twim::*};
use rtt_target::{rprint, rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let cp = pac::Peripherals::take().unwrap();

    let p0 = gpio::p0::Parts::new(cp.P0);

    let scl = p0.p0_08.into_floating_input().degrade();
    let sda = p0.p0_16.into_floating_input().degrade();

    let mut twim = Twim::new(cp.TWIM0, Pins { scl, sda }, Frequency::K100);

    rprintln!("Start i2c scanning...");
    rprintln!();

    for addr in 0x00_u8..0x80 {
        // Write the empty array and check the slave response.
        let byte: [u8; 1] = [0; 1];
        if twim.write(addr, &byte).is_ok() {
            rprint!("{:02x}", addr);
        } else {
            rprint!("..");
        }
        if addr % 0x10 == 0x0F {
            rprintln!();
        } else {
            rprint!(" ");
        }
    }

    rprintln!();
    rprintln!("Done!");
    #[allow(clippy::empty_loop)]
    loop {}
}
