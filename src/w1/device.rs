use error::Error;
use std::fs;

pub trait Device {
    fn device_path(&self) -> &str;
    fn name(&self) -> &str;
}

pub trait SlaveDevice: Device {
    fn read_bytes(&self) -> Result<Vec<u8>, Error> {
        fs::read(self.device_path()).map_err(Error::from)
    }

    fn read_string(&self) -> Result<String, Error> {
        Ok(String::from_utf8_lossy(&self.read_bytes()?).to_string())
    }
}
