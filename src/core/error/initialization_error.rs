use crate::core::error::failed_to_open_lock_file_error::FailedToOpenLockfileError;
use crate::core::error::lock_file_error::LockFileError;

#[derive(Debug)]
pub enum InitializationError {
    GenericLockfile(LockFileError),
    FailedToOpenLockfile(FailedToOpenLockfileError),
    UnknownError(String)
}



