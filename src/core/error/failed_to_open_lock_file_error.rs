use std::fmt;
use std::io::Error;

#[derive(Debug)]
pub struct FailedToOpenLockfileError {
    _error: Error
}

impl FailedToOpenLockfileError {
    pub(crate) fn new(error: Error) -> FailedToOpenLockfileError {
        return FailedToOpenLockfileError {_error: error};
    }
}

impl fmt::Display for FailedToOpenLockfileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}