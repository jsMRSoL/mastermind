use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ParsePinError(pub String);

impl fmt::Display for ParsePinError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ParsePinError: {} is not one of the options.", self.0)
    }
}

impl Error for ParsePinError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
    fn description(&self) -> &str {
        "The colour-code letter submitted was not one of the options."
    }
    fn cause(&self) -> Option<&dyn Error> {
        None
    }
}
