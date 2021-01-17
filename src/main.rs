use argh::FromArgs;
use eyre::Report;
use linux_embedded_hal::I2cdev;
use si5351::{Si5351, Si5351Device};
use std::fmt;

#[derive(FromArgs)]
#[argh(description = "set Adafruit Si5351 module frequency")]
struct InputArgs {
    /// suppress message indicating action taken
    #[argh(switch, short = 'q')]
    quiet: bool,
    /// output clock to set
    #[argh(option, short = 'c', default = "default_clk()", from_str_fn(arg_clk))]
    clk: si5351::ClockOutput,
    /// PLL to use to set output clock
    #[argh(option, short = 'p', default = "default_pll()", from_str_fn(arg_pll))]
    pll: si5351::PLL,
    #[argh(positional)]
    bus: String,
    #[argh(positional, from_str_fn(from_base_10))]
    freq: u32,
}

fn default_clk() -> si5351::ClockOutput {
    si5351::ClockOutput::Clk0
}

fn arg_clk(val: &str) -> Result<si5351::ClockOutput, String> {
    match val {
        "0" => Ok(si5351::ClockOutput::Clk0),
        "1" => Ok(si5351::ClockOutput::Clk1),
        "2" => Ok(si5351::ClockOutput::Clk2),
        _ => Err("PLL must be \"0\", \"1\", or \"2\"".into()),
    }
}

fn default_pll() -> si5351::PLL {
    si5351::PLL::A
}

fn arg_pll(val: &str) -> Result<si5351::PLL, String> {
    match val {
        "A" => Ok(si5351::PLL::A),
        "B" => Ok(si5351::PLL::B),
        _ => Err("PLL must be \"A\" or \"B\"".into()),
    }
}

fn from_base_10(val: &str) -> Result<u32, String> {
    match u32::from_str_radix(val, 10) {
        Ok(v) => Ok(v),
        Err(_) => Err("Unable to convert address from base 10".into()),
    }
}

#[derive(Debug)]
struct SisetClk(si5351::ClockOutput);

impl From<si5351::ClockOutput> for SisetClk {
    fn from(clk: si5351::ClockOutput) -> Self {
        SisetClk(clk)
    }
}

impl fmt::Display for SisetClk {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            si5351::ClockOutput::Clk0 => write!(f, "0"),
            si5351::ClockOutput::Clk1 => write!(f, "1"),
            si5351::ClockOutput::Clk2 => write!(f, "2"),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct SisetPLL(si5351::PLL);

impl From<si5351::PLL> for SisetPLL {
    fn from(pll: si5351::PLL) -> Self {
        SisetPLL(pll)
    }
}

impl fmt::Display for SisetPLL {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            si5351::PLL::A => write!(f, "A"),
            si5351::PLL::B => write!(f, "B"),
        }
    }
}

fn main() -> eyre::Result<()> {
    let args: InputArgs = argh::from_env();

    let i2c: I2cdev = I2cdev::new(args.bus)?;

    let mut clock = Si5351Device::<I2cdev>::new_adafruit_module(i2c);
    clock.init_adafruit_module().map_err(Report::msg)?;

    clock
        .set_frequency(args.pll, args.clk, args.freq)
        .map_err(Report::msg)?;

    if !args.quiet {
        println!(
            "Clk {} set to {} Hz using PLL {}.",
            SisetClk::from(args.clk),
            args.freq,
            SisetPLL::from(args.pll)
        );
    }
    Ok(())
}
