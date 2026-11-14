use std::fmt::Debug;
use std::fmt::Display;

pub struct LengthError;

impl std::error::Error for LengthError {}

impl Debug for LengthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Length error")
    }
}

impl Display for LengthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Length error")
    }
}
