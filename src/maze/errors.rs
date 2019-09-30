use std::error::Error;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub enum MazeError {
    NoStartError,
    NoExitError,
}

impl Display for MazeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use MazeError::*;
        let text = match self {
            NoStartError => "image does not contain a starting point",
            NoExitError => "image does not contain a exit ponit",
        };
        write!(f, "{}", text)
    }
}

impl Error for MazeError {}
