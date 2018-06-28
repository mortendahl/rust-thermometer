use error::Error;
use futures::{Future, Stream};
use tokio_codec::{FramedRead, LinesCodec};
use tokio_fs::OpenOptions;

/// W1 devices folder.
pub const DEVICE_PATH_FOLDER: &str = "/sys/bus/w1/devices";

/// W1 slave devices suffix.
pub const SLAVE_DEVICE_PATH_SUFFIX: &str = "w1_slave";

/// Trait that must be implemented by all W1 devices.
pub trait Device: Send {
    /// Full device path.
    fn device_path(&self) -> &str;
}

/// Trait that must be implemented by all W1 slave devices.
pub trait SlaveDevice: Device {
    /// Stream of lines read from w1_slave.
    fn lines(&self) -> Box<dyn Stream<Item = String, Error = Error> + Send> {
        let path = self.device_path().to_string();

        Box::new(
            OpenOptions::new()
                .read(true)
                .open(path)
                .and_then(|f| Ok(FramedRead::new(f, LinesCodec::new())))
                .flatten_stream()
                .map_err(Error::from),
        )
    }
}
