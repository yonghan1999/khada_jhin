use crate::core::error::failed_to_open_lock_file_error::FailedToOpenLockfileError;
use crate::core::error::lcu_address_error::LcuAddressError;

#[derive(Debug)]
pub enum InitializationError {
    CommandNotFound(LcuAddressError),
    UnknownError(String)
}



