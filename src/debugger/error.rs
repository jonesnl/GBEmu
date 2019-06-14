use std::fmt;
use std::error;

#[derive(Clone, Debug)]
pub struct DebugError;

impl fmt::Display for DebugError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Debugger encountered an error")
    }
}

impl error::Error for DebugError {}

pub type DebugResult<T> = Result<T, DebugError>;
