use std::fmt;

#[derive(Debug)]
pub struct FailedToOpenLockfileError {
    _error: IOError
}

impl FailedToOpenLockfileError {
    pub(crate) fn new(error: IOError) -> FailedToOpenLockfileError {
        return FailedToOpenLockfileError {_error: error};
    }
}

impl fmt::Display for FailedToOpenLockfileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}