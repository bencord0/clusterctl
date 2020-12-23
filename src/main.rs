use clap::Clap;
use rppal::i2c::I2c;
use std::error::Error;

#[derive(Clap, Debug)]
#[clap(version, author)]
pub struct Opts {
    #[clap(subcommand)]
    pub commands: Commands,
}

#[derive(Clap, Debug)]
pub enum Commands {
    #[clap(name = "status")]
    Status,

    #[clap(name = "on")]
    On(OnOffOpts),

    #[clap(name = "off")]
    Off(OnOffOpts),
}

#[derive(Clap, Debug)]
pub struct OnOffOpts {
    pub pi: u8,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts = Opts::parse();
    println!("Opts: {:?}", opts);

    // The RaspberryPi I2C bus assigned to HATs is set by convention
    let mut i2c = I2c::with_bus(1)?;

    // The ClusterPI base address
    i2c.set_slave_address(0x20)?;

    // The control byte to turn on/off specific RPi Zeros on the HAT
    let state = i2c.smbus_read_byte(0x01)?;
    println!("State: 0b{:b}", state);

    match opts.commands {
        Commands::Status => {
            for pi in 0..4 {
                let power_state = (state & 1<<pi) != 0;
                println!("p{}: {}", pi + 1, power_state);
            }
        },

        Commands::On(OnOffOpts{pi}) => {
            let pi = pi - 1;
            let newstate = state | (1 << pi);
            println!("Newstate: 0b{:b}", newstate);

            i2c.smbus_write_byte(0x01, newstate)?;
        },

        Commands::Off(OnOffOpts{pi}) => {
            let pi = pi - 1;
            let newstate = state & (255 - (1 << pi));
            println!("Newstate: 0b{:b}", newstate);

            i2c.smbus_write_byte(0x01, newstate)?;
        },
    }

    Ok(())
}
