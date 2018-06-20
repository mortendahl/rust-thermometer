use w1::device::{Device, SlaveDevice};
use w1::thermometer::Thermometer;

const DEVICE_PATH_FOLDER: &str = "/sys/bus/w1/devices";
const DEVICE_PATH_SUFFIX: &str = "w1_slave";

pub struct DS18B20 {
    name: String,
    path: String,
}

impl DS18B20 {
    pub fn new<S>(name: S, device: S) -> DS18B20
    where
        S: Into<String>,
    {
        let path = format!("{}/{}/{}", DEVICE_PATH_FOLDER, device.into(), DEVICE_PATH_SUFFIX);

        DS18B20 {
            name: name.into(),
            path,
        }
    }
}

impl Device for DS18B20 {
    fn device_path(&self) -> &str {
        &self.path
    }

    fn name(&self) -> &str {
        &self.name
    }
}

impl SlaveDevice for DS18B20 {}

impl Thermometer for DS18B20 {}
