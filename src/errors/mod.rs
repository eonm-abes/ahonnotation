use std::error::Error;
use std::fmt;

/// This error is returned when a user tries to build a DictionnaryBuilder without setting a dictionary
#[derive(Debug, Clone)]
pub struct MissingDictionnary;

impl fmt::Display for MissingDictionnary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "A Tagger can't be build without a dictionary")
    }
}

impl Error for MissingDictionnary {}
