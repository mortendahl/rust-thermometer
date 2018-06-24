use error::Error;
use w1::device::{Device, SlaveDevice};
use w1::device::{DEVICE_PATH_FOLDER, DEVICE_PATH_SUFFIX};
use w1::thermometer::{Temperature, Thermometer};

/// W1 DS18B20 thermometer device.
pub struct DS18B20 {
    path: String,
}

impl DS18B20 {
    /// Create new `DS18B20` thermometer device.
    ///
    /// # Arguments
    ///
    /// * `device` - device name (folder name inside /sys/bus/w1/devices)
    pub fn new<S>(device: S) -> DS18B20
    where
        S: Into<String>,
    {
        let path = format!("{}/{}/{}", DEVICE_PATH_FOLDER, device.into(), DEVICE_PATH_SUFFIX);

        DS18B20 { path }
    }
}

/// `DS18B20` is W1 device.
impl Device for DS18B20 {
    fn device_path(&self) -> &str {
        &self.path
    }
}

/// `DS18B20` is W1 slave device.
///
/// Auto adds `read_bytes` & `read_string` functions.
impl SlaveDevice for DS18B20 {}

/// Parse DS18B20 sensor temperature value.
///
/// # Arguments
///
/// * `s` - string from w1_slave device
fn parse_temperature(s: &str) -> Result<Temperature, Error> {
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

/// `DS18B20` is W1 temperature sensor.
impl Thermometer for DS18B20 {
    fn read_temperature(&self) -> Result<Temperature, Error> {
        parse_temperature(&self.read_string()?)
    }
}

#[cfg(test)]
mod tests {
    use super::parse_temperature;

    #[test]
    fn test_parser_valid_temperature() {
        let temp = parse_temperature(
            "b2 01 4b 46 7f ff 0e 10 8c : crc=8c YES\n\
             b2 01 4b 46 7f ff 0e 10 8c t=27125",
        ).unwrap();

        assert_eq!(temp.value, 27125);
        assert_eq!(temp.celsius(), 27.125);
    }

    #[test]
    fn test_parser_invalid_temperature_minus_one() {
        let temp = parse_temperature(
            "b2 01 4b 46 7f ff 0e 10 8c : crc=8c YES\n\
             b2 01 4b 46 7f ff 0e 10 8c t=-1",
        );

        assert!(temp.is_err());
    }

    #[test]
    fn test_parser_invalid_temperature_crc() {
        let temp = parse_temperature(
            "b2 01 4b 46 7f ff 0e 10 8c : crc=8c NO\n\
             b2 01 4b 46 7f ff 0e 10 8c t=27125",
        );

        assert!(temp.is_err());
    }

    #[test]
    fn test_parser_sensor_error() {
        let temp = parse_temperature(
            "b2 01 4b 46 7f ff 0e 10 8c : crc=8c NO\n\
             b2 01 4b 46 7f ff 0e 10 8c t=85000",
        );

        assert!(temp.is_err());
    }

    #[test]
    fn test_parser_invalid_temperature_value() {
        let temp = parse_temperature(
            "b2 01 4b 46 7f ff 0e 10 8c : crc=8c YES\n\
             b2 01 4b 46 7f ff 0e 10 8c t=hallo",
        );

        assert!(temp.is_err());
    }

    #[test]
    fn test_parser_invalid_temperature_format() {
        let temp = parse_temperature("YES");
        assert!(temp.is_err());
    }
}
