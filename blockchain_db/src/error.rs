use std::error::Error as StdError;
use std::io::Error as IOError;

pub struct Error {

}

#[derive(Debug)]
pub enum Error {
    GenericError,
    TransactionStartError,
    DBOpenError,
    DBCreateError,
    DBSyncError,
    DoesNotExist,
    Exists,
    Invalid
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.description())
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match self {
            Error::GenericError(message) => message,
            Error::TransactionStartError(message) => message,
            Error::DBOpenError(message) => message,
            Error::DBCreateError(message) => message,
            Error::DBSyncError(message) => message,
            Error::DoesNotExist(message) => message,
            Error::Exists(message) => message,
            Error::Invalid(message) => message
        }
    }
}

impl From<IOError> for Error {
    fn from(err: IOError) -> Self {

    }
}
