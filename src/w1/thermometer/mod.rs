pub mod ds18b20;

use error::Error;
use std::str::FromStr;
use w1::device::SlaveDevice;

/// Trait that must be implemented by all temperature sensors.
pub trait Thermometer: SlaveDevice {
    fn read_temperature(&self) -> Result<Temperature, Error>;
}

/// Temperature unit.
#[derive(PartialEq, Debug)]
pub enum Units {
    Celsius,
    Fahrenheit,
}

impl AsRef<str> for Units {
    fn as_ref(&self) -> &str {
        match self {
            Units::Celsius => "celsius",
            Units::Fahrenheit => "fahrenheit",
        }
    }
}

impl FromStr for Units {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "celsius" => Ok(Units::Celsius),
            "fahrenheit" => Ok(Units::Fahrenheit),
            _ => Err(Error::from(format!("Invalid temperature units: {}", s))),
        }
    }
}

/// Temperature.
#[derive(Clone)]
pub struct Temperature {
    /// Value in celsius degrees multiplied by 1_000.0
    value: i64,
}

impl Temperature {
    /// Create new `Temperature`.
    ///
    /// # Arguments
    ///
    /// * `value` - degrees celsius multiplied by 1_000.0
    pub fn new(value: i64) -> Temperature {
        Temperature { value }
    }

    /// Temperature in celsius degrees.
    pub fn celsius(&self) -> f64 {
        (self.value as f64) / 1_000.0
    }

    /// Temperature in fahrenheit degrees.
    pub fn fahrenheit(&self) -> f64 {
        self.celsius() * 1.8 + 32.0
    }

    /// Temperature formatted as `String`.
    ///
    /// # Arguments
    ///
    /// * `units` - temperature units
    pub fn to_string(&self, units: &Units) -> String {
        match units {
            Units::Celsius => format!("{:.1} °C", self.celsius()),
            Units::Fahrenheit => format!("{:.1} °F", self.fahrenheit()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Temperature, Units};

    #[test]
    fn test_temperature_celsius_value() {
        assert_eq!(Temperature::new(10_123).celsius(), 10.123);
    }

    #[test]
    fn test_temperature_fahrenheit_value() {
        assert_eq!(Temperature::new(-50_000).fahrenheit(), -58.0);
        assert_eq!(Temperature::new(-5_000).fahrenheit(), 23.0);
    }

    #[test]
    fn test_units_conversion() {
        assert_eq!(Units::Celsius.as_ref().parse::<Units>().unwrap(), Units::Celsius);
        assert_eq!(Units::Fahrenheit.as_ref().parse::<Units>().unwrap(), Units::Fahrenheit);
    }
}
