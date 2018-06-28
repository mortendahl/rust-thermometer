use clap::{App, Arg};
use std::fmt::Display;
use std::str::FromStr;
use w1::thermometer::Units;

/// Package version (set at compile time).
pub const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Package name (set at compile time).
pub const PKG_NAME: &str = env!("CARGO_PKG_NAME");

/// Return `()` in case provided argument is valid otherwise error message is returned.
///
/// # Arguments
///
/// * `value` - number in a `String`
/// * `min` - if provided, `value` must be greater or equal to `min`
/// * `max` - if provided, `value` must be lower or equal to `max`
#[cfg_attr(feature = "cargo-clippy", allow(trivially_copy_pass_by_ref, needless_pass_by_value))]
fn validate<T>(value: String, min: Option<T>, max: Option<T>) -> Result<(), String>
where
    T: FromStr + PartialOrd + Display,
{
    match value.parse::<T>() {
        Ok(value) => {
            if let Some(min) = min {
                if value < min {
                    return Err(format!("Value must not be lower than {}", min));
                }
            }
            if let Some(max) = max {
                if value > max {
                    return Err(format!("Value must not be greater than {}", max));
                }
            }

            Ok(())
        }
        Err(_) => Err("Invalid port number".to_string()),
    }
}

fn validate_max_fps(value: String) -> Result<(), String> {
    validate::<u64>(value, Some(2), Some(60))
}

fn validate_temperature_interval(value: String) -> Result<(), String> {
    validate::<u64>(value, Some(500), Some(60_000))
}

/// Application configuration.
pub struct Config {
    inside_thermometer_device: String,
    outside_thermometer_device: String,
    temperature_units: Units,
    max_fps: u64,
    temperature_interval: u64,
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
            .arg(
                Arg::with_name("MAX_FPS")
                    .long("max-fps")
                    .env("MAX_FPS")
                    .help("Max frames per second")
                    .takes_value(true)
                    .required(true)
                    .default_value("2")
                    .validator(validate_max_fps),
            )
            .arg(
                Arg::with_name("TEMPERATURE_INTERVAL")
                    .long("temperature-interval")
                    .env("TEMPERATURE_INTERVAL")
                    .help("Interval in which temperatures are read from sensors (ms)")
                    .takes_value(true)
                    .required(true)
                    .default_value("500")
                    .validator(validate_temperature_interval),
            )
            .get_matches();

        // It's ok to unwrap all values. If it crashes, it's programmer error in argument definition.
        let inside_thermometer_device = matches.value_of("INSIDE_THERMOMETER").unwrap().to_string();
        let outside_thermometer_device = matches.value_of("OUTSIDE_THERMOMETER").unwrap().to_string();
        let temperature_units = matches.value_of("TEMPERATURE_UNITS").unwrap().parse::<Units>().unwrap();
        let max_fps = matches.value_of("MAX_FPS").unwrap().parse::<u64>().unwrap();
        let temperature_interval = matches
            .value_of("TEMPERATURE_INTERVAL")
            .unwrap()
            .parse::<u64>()
            .unwrap();

        Config {
            inside_thermometer_device,
            outside_thermometer_device,
            temperature_units,
            max_fps,
            temperature_interval,
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
    pub fn temperature_units(&self) -> Units {
        self.temperature_units
    }

    /// Max frames per second.
    pub fn max_fps(&self) -> u64 {
        self.max_fps
    }

    /// Interval in which temperatures are read from sensors (ms)
    pub fn temperature_interval(&self) -> u64 {
        self.temperature_interval
    }
}

lazy_static! {
    /// Shared configuration.
    pub static ref CONFIG: Config = Config::new();
}
