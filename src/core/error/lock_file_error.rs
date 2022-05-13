use std::fmt;

#[derive(Debug)]
pub struct LockFileError {
    _error: LockfileErrorTypes
}

impl LockFileError {
    pub(crate) fn new(error: LockfileErrorTypes) -> LockFileError {
        return LockFileError {_error: error};
    }
}

#[derive(Debug)]
pub enum LockfileErrorTypes {
    LeagueNotRunning,
    LockfileUnexpectedFormat
}

impl fmt::Display for LockFileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}