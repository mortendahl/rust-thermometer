use clap::{App, Arg};
use w1::thermometer::Units;

/// Package version (set at compile time).
pub const PKG_VERSION: &'static str = env!("CARGO_PKG_VERSION");

/// Package name (set at compile time).
pub const PKG_NAME: &'static str = env!("CARGO_PKG_NAME");

/// Application configuration.
pub struct Config {
    inside_thermometer_device: String,
    outside_thermometer_device: String,
    temperature_units: Units,
}

impl Config {
    /// Create new application `Config`.
    fn new() -> Config {
        let matches = App::new(PKG_NAME)
            .version(PKG_VERSION)
            .about("Raspberry Pi Thermometer")
            .arg(
                Arg::with_name("INSIDE_THERMOMETER")
                    .long("inside-thermometer")
                    .env("INSIDE_THERMOMETER")
                    .help("Inside W1 thermometer device ID")
                    .takes_value(true)
                    .required(true)
                    .default_value("28-000009e8f6e7"),
            )
            .arg(
                Arg::with_name("OUTSIDE_THERMOMETER")
                    .long("outside-thermometer")
                    .env("OUTSIDE_THERMOMETER")
                    .help("Outside W1 thermometer device ID")
                    .takes_value(true)
                    .required(true)
                    .default_value("28-000009d4dffc"),
            )
            .arg(
                Arg::with_name("TEMPERATURE_UNITS")
                    .long("temperature-units")
                    .env("TEMPERATURE_UNITS")
                    .help("Temperature units")
                    .takes_value(true)
                    .required(true)
                    .possible_value(Units::Celsius.as_ref())
                    .possible_value(Units::Fahrenheit.as_ref())
                    .default_value(Units::Celsius.as_ref()),
            )
            .get_matches();

        // It's ok to unwrap all values. If it crashes, it's programmer error in argument definition.
        let inside_thermometer_device = matches.value_of("INSIDE_THERMOMETER").unwrap().to_string();
        let outside_thermometer_device = matches.value_of("OUTSIDE_THERMOMETER").unwrap().to_string();
        let temperature_units = matches.value_of("TEMPERATURE_UNITS").unwrap().parse::<Units>().unwrap();

        Config {
            inside_thermometer_device,
            outside_thermometer_device,
            temperature_units,
        }
    }

    /// Inside thermometer device identifier.
    pub fn inside_thermometer_device(&self) -> &str {
        &self.inside_thermometer_device
    }

    /// Outside thermometer device identifier.
    pub fn outside_thermometer_device(&self) -> &str {
        &self.outside_thermometer_device
    }

    /// Temperature units.
    pub fn temperature_units(&self) -> &Units {
        &self.temperature_units
    }
}

lazy_static! {
    /// Shared configuration.
    pub static ref CONFIG: Config = Config::new();
}
