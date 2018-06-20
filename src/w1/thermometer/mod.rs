pub mod ds18b20;

use error::Error;
use std::str::FromStr;
use w1::device::SlaveDevice;

pub trait Thermometer: SlaveDevice {
    fn temperature(&self) -> Result<Temperature, Error> {
        self.read_string()?.parse::<Temperature>()
    }
}

pub struct Temperature {
    value: i64,
}

impl Temperature {
    fn new(value: i64) -> Temperature {
        Temperature { value }
    }

    pub fn celsius(&self) -> f64 {
        (self.value as f64) / 1_000.0
    }
}

impl FromStr for Temperature {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let first_line = lines
            .next()
            .ok_or_else(|| Error::from("Unable to get first temperature line"))?;

        if !first_line.ends_with("YES") {
            return Err(Error::from("Invalid temperature CRC"));
        }

        let value = lines
            .next()
            .ok_or_else(|| Error::from("Unable to get second temperature line"))
            .and_then(|l| l.split("t=").nth(1).ok_or_else(|| Error::from("Missing t= separator")))
            .and_then(|v| {
                v.parse::<i64>()
                    .map_err(|e| Error::from(format!("Unable to parse temperature: {}", e)))
            })
            .and_then(|v| match v {
                -1 => Err(Error::from("Invalid temperature value (-1)")),
                85_000 => Err(Error::from("Sensor error (t=85000)")),
                _ => Ok(v),
            })?;

        Ok(Temperature::new(value))
    }
}

#[cfg(test)]
mod tests {
    use super::Temperature;

    #[test]
    fn test_parser_valid_temperature() {
        let temp = "b2 01 4b 46 7f ff 0e 10 8c : crc=8c YES\n\
                    b2 01 4b 46 7f ff 0e 10 8c t=27125"
            .parse::<Temperature>()
            .unwrap();

        assert_eq!(temp.value, 27125);
        assert_eq!(temp.celsius(), 27.125);
    }

    #[test]
    fn test_parser_invalid_temperature_minus_one() {
        let temp = "b2 01 4b 46 7f ff 0e 10 8c : crc=8c YES\n\
                    b2 01 4b 46 7f ff 0e 10 8c t=-1"
            .parse::<Temperature>();

        assert!(temp.is_err());
    }

    #[test]
    fn test_parser_invalid_temperature_crc() {
        let temp = "b2 01 4b 46 7f ff 0e 10 8c : crc=8c NO\n\
                    b2 01 4b 46 7f ff 0e 10 8c t=27125"
            .parse::<Temperature>();

        assert!(temp.is_err());
    }

    #[test]
    fn test_parser_sensor_error() {
        let temp = "b2 01 4b 46 7f ff 0e 10 8c : crc=8c NO\n\
                    b2 01 4b 46 7f ff 0e 10 8c t=85000"
            .parse::<Temperature>();

        assert!(temp.is_err());
    }

    #[test]
    fn test_parser_invalid_temperature_value() {
        let temp = "b2 01 4b 46 7f ff 0e 10 8c : crc=8c YES\n\
                    b2 01 4b 46 7f ff 0e 10 8c t=hallo"
            .parse::<Temperature>();

        assert!(temp.is_err());
    }

    #[test]
    fn test_parser_invalid_temperature_format() {
        let temp = "YES".parse::<Temperature>();
        assert!(temp.is_err());
    }
}
