use std::error::Error;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub struct InvalidAlgorithmError(pub String);

impl Display for InvalidAlgorithmError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{} is not a valid algorithm.", self.0)
    }
}

impl Error for InvalidAlgorithmError {}
