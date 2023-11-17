use std::error::Error;
use std::fmt;


#[derive(Debug)]
pub struct ClientError(String);

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for ClientError {}
