use std::error;
use std::fmt;
use std::result;
use std;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    repr: Repr,
}

#[derive(Debug)]
enum Repr {
    Simple(ErrorKind),
    Custom(Custom),
}

#[derive(Debug)]
struct Custom {
    kind: ErrorKind,
    error: Box<error::Error + Send + Sync>,
}

#[derive(Debug)]
pub enum ErrorKind {
    IO,
    InvalidInput,
    Validation,
}


impl ErrorKind {
    fn as_str(&self) -> &'static str {
        match *self {
            ErrorKind::IO => "IO error",
            ErrorKind::InvalidInput => "Invalid input",
            ErrorKind::Validation => "Validation error",
        }
    }
}


impl Error {
    pub fn new<E>(kind: ErrorKind, error: E) -> Error
    where
        E: Into<Box<error::Error + Send + Sync>>,
    {
        Error {
            repr: Repr::Custom(Custom {
                kind: kind,
                error: error.into(),
            }),
        }
    }

    pub fn simple(kind: ErrorKind) -> Error {
        Error { repr: Repr::Simple(kind) }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self.repr {
            Repr::Simple(ref kind) => write!(fmt, "{}", kind.as_str()),
            Repr::Custom(ref c) => c.error.fmt(fmt),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match self.repr {
            Repr::Simple(ref kind) => kind.as_str(),
            Repr::Custom(ref c) => c.error.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match self.repr {
            Repr::Simple(..) => None,
            Repr::Custom(ref c) => c.error.cause(),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error {
        Error::new(ErrorKind::IO, e)
    }
}
