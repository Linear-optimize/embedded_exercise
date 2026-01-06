#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;

use embedded_hal::i2c::I2c;
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

    for header in 0x0..0x10 {
        rprint!("{:02x}", header);
    }
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

    const ACCELEROMETER_ADDR: u8 = 0b0011001;
    const MAGNETOMETER_ADDR: u8 = 0b0011110;

    const ACCELEROMETER_ID_REG: u8 = 0x0f;
    const MAGNETOMETER_ID_REG: u8 = 0x4f;

    let mut acc = [0_u8];
    let mut mag = [0_u8];

    twim.write_read(ACCELEROMETER_ADDR, &[ACCELEROMETER_ID_REG], &mut acc)
        .unwrap();
    twim.write_read(MAGNETOMETER_ADDR, &[MAGNETOMETER_ID_REG], &mut mag)
        .unwrap();
    rprintln!("The accelerometer chip's id is: {:#b}", acc[0]);
    rprintln!("The MaGNETOMETER chip's id is: {:#b}", mag[0]);
    #[allow(clippy::empty_loop)]
    loop {}
}
