use argh::FromArgs;
use eyre::{eyre, Report};
use linux_embedded_hal::{i2cdev::linux::LinuxI2CError, I2cdev};
use si5351;
use si5351::{Si5351, Si5351Device};
use std::error::Error;

#[derive(FromArgs)]
#[argh(description = "set Adafruit Si5351 module frequency")]
struct InputArgs {
    #[argh(positional)]
    bus: String,
    #[argh(positional, from_str_fn(from_base_16))]
    addr: u8,
    #[argh(positional, from_str_fn(from_base_10))]
    freq: u32,
}

fn from_base_16(val: &str) -> Result<u8, String> {
    match u8::from_str_radix(val, 16) {
        Ok(v) => Ok(v),
        Err(_) => Err("Unable to convert address from base 16".into()),
    }
}

fn from_base_10(val: &str) -> Result<u32, String> {
    match u32::from_str_radix(val, 10) {
        Ok(v) => Ok(v),
        Err(_) => Err("Unable to convert address from base 10".into()),
    }
}

fn wrap_si_error(si_error: si5351::Error) -> Report {
    let wrap_err = match si_error {
        si5351::Error::CommunicationError => eyre!("Communication Error"),
        si5351::Error::InvalidParameter => eyre!("Invalid Parameter"),
    };
    return From::from(wrap_err);
}

fn main() -> eyre::Result<()> {
    let args: InputArgs = argh::from_env();

    let i2c: I2cdev = I2cdev::new(args.bus)?;

    let mut clock = Si5351Device::<I2cdev>::new_adafruit_module(i2c);
    clock.init_adafruit_module().map_err(wrap_si_error)?;

    clock
        .set_frequency(si5351::PLL::A, si5351::ClockOutput::Clk0, args.freq)
        .map_err(wrap_si_error)?;

    println!("PLL frequency of clk 0 set to {} Hz.", args.freq);
    Ok(())
}
