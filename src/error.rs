use std;
use std::fmt::{Display, Formatter};
use std::io::Error as IOError;

#[derive(Debug, Clone)]
pub struct Error {
    msg: String,
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        &self.msg
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        f.write_str(&self.msg)
    }
}

impl From<String> for Error {
    fn from(msg: String) -> Error {
        Error { msg }
    }
}

impl<'a> From<&'a str> for Error {
    fn from(msg: &'a str) -> Error {
        Error::from(msg.to_string())
    }
}

impl From<IOError> for Error {
    fn from(e: IOError) -> Error {
        Error::from(format!("I/O error: {}", e))
    }
}
