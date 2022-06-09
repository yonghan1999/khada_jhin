use std::fmt;

#[derive(Debug)]
pub struct LcuAddressError {
    _error: LcuAddressErrorTypes
}

impl LcuAddressError {
    pub(crate) fn new(error: LcuAddressErrorTypes) -> LcuAddressError {
        return LcuAddressError {_error: error};
    }
}

#[derive(Debug)]
pub enum LcuAddressErrorTypes {
    LeagueNotRunning,
    LockfileUnexpectedFormat
}

impl fmt::Display for LcuAddressError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}