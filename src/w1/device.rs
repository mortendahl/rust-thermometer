use error::Error;
use std::fs;

/// W1 devices folder.
pub const DEVICE_PATH_FOLDER: &str = "/sys/bus/w1/devices";

/// W1 slave devices suffix.
pub const DEVICE_PATH_SUFFIX: &str = "w1_slave";

/// Trait that must be implemented by all W1 devices.
pub trait Device: Send {
    /// Full device path.
    fn device_path(&self) -> &str;
}

/// Trait that must be implemented by all W1 slave devices.
pub trait SlaveDevice: Device {
    /// Read bytes value.
    fn read_bytes(&self) -> Result<Vec<u8>, Error> {
        fs::read(self.device_path()).map_err(Error::from)
    }

    /// Read bytes value and try to convert it to `String`.
    fn read_string(&self) -> Result<String, Error> {
        Ok(String::from_utf8_lossy(&self.read_bytes()?).to_string())
    }
}
