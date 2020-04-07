use std::ffi::NulError;
use std::fmt;
use std::fmt::Display;
use std::io::Error as IoError;

use failure::{Backtrace, Context, Fail};
use log::SetLoggerError;
use nix::Error as NixError;
use serde_json::Error as SerdeJSONError;

#[derive(Fail, Debug)]
pub enum ErrorKind {
    #[fail(display = "std::io::Error {}", error)]
    Io { error: IoError },
    #[fail(display = "nix::Error {}", error)]
    Nix { error: NixError },
    #[fail(display = "serde_json::Error {}", error)]
    SerdeJSON { error: SerdeJSONError },
    #[fail(display = "fern::InitError {}", error)]
    Log { error: SetLoggerError },
    #[fail(display = "OCIError {}", error)]
    OCI { error: OCIError },
    #[fail(display = "Null Error")]
    Null { error: NulError },
}

#[derive(Debug)]
pub struct Error {
    inner: Context<ErrorKind>,
}

#[derive(Debug)]
pub struct OCIError {
    #[allow(dead_code)]
    kind: OCIErrorKind,
    #[allow(dead_code)]
    description: String,
}

#[derive(Debug)]
pub enum OCIErrorKind {
    #[allow(dead_code)]
    InvaliedStatus,
}

impl Display for OCIErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl OCIError {
    #[allow(dead_code)]
    pub fn new(kind: OCIErrorKind, description: &str) -> Self {
        OCIError {
            kind,
            description: description.into(),
        }
    }

    #[allow(dead_code)]
    pub fn context(&self, kind: ErrorKind) -> Context<ErrorKind> {
        Context::new(kind)
    }
}

impl Display for OCIError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "kind: {}, description: {}", self.kind, self.description)
    }
}

impl Fail for Error {
    fn cause(&self) -> Option<&dyn Fail> {
        self.inner.cause()
    }
    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl Error {
    #[allow(dead_code)]
    pub fn new(inner: Context<ErrorKind>) -> Error {
        Error { inner }
    }
    #[allow(dead_code)]
    pub fn kind(&self) -> &ErrorKind {
        self.inner.get_context()
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error {
            inner: Context::new(kind),
        }
    }
}

impl From<Context<ErrorKind>> for Error {
    fn from(inner: Context<ErrorKind>) -> Error {
        Error { inner }
    }
}

impl From<IoError> for Error {
    fn from(error: IoError) -> Error {
        Error {
            inner: Context::new(ErrorKind::Io { error }),
        }
    }
}

impl From<NixError> for Error {
    fn from(error: NixError) -> Error {
        Error {
            inner: Context::new(ErrorKind::Nix { error }),
        }
    }
}

impl From<SerdeJSONError> for Error {
    fn from(error: SerdeJSONError) -> Error {
        Error {
            inner: Context::new(ErrorKind::SerdeJSON { error }),
        }
    }
}

impl From<SetLoggerError> for Error {
    fn from(error: SetLoggerError) -> Error {
        Error {
            inner: Context::new(ErrorKind::Log { error }),
        }
    }
}

impl From<OCIError> for Error {
    fn from(error: OCIError) -> Error {
        Error {
            inner: Context::new(ErrorKind::OCI { error }),
        }
    }
}

impl From<NulError> for Error {
    fn from(error: NulError) -> Error {
        Error {
            inner: Context::new(ErrorKind::Null { error }),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn nixerror_to_error() {
        let f = || -> Result<(), Error> { Err(nix::Error::invalid_argument().into()) };
        let _: Error = f().unwrap_err();
    }

    #[test]
    fn ocierror_to_error() {
        let f = || -> Result<(), Error> {
            Err(OCIError::new(OCIErrorKind::InvaliedStatus, "container must be stopped").into())
        };
        let _: Error = f().unwrap_err();
    }
}
