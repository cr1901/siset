use argh::FromArgs;
use eyre::Report;
use linux_embedded_hal::I2cdev;
use si5351::{Si5351, Si5351Device};

#[derive(FromArgs)]
#[argh(description = "set Adafruit Si5351 module frequency")]
struct InputArgs {
    #[argh(positional)]
    bus: String,
    #[argh(positional, from_str_fn(from_base_10))]
    freq: u32,
}

fn from_base_10(val: &str) -> Result<u32, String> {
    match u32::from_str_radix(val, 10) {
        Ok(v) => Ok(v),
        Err(_) => Err("Unable to convert address from base 10".into()),
    }
}

fn main() -> eyre::Result<()> {
    let args: InputArgs = argh::from_env();

    let i2c: I2cdev = I2cdev::new(args.bus)?;

    let mut clock = Si5351Device::<I2cdev>::new_adafruit_module(i2c);
    clock.init_adafruit_module().map_err(Report::msg)?;

    clock
        .set_frequency(si5351::PLL::A, si5351::ClockOutput::Clk0, args.freq)
        .map_err(Report::msg)?;

    println!("PLL frequency of clk 0 set to {} Hz.", args.freq);
    Ok(())
}
