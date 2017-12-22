//! A module to handle all errors via error-chain crate

#![allow(missing_docs)]

use std::fmt::{self, Display, Formatter};
use std::io;
use std::str::Utf8Error;

use failure::Fail;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    /// deprecated feature
    Deprecated{
        feat: &'static str,
    },

    /// unknown wire type
    UnknownWireType {
        t: u8
    },

    /// cannot decode varint
    Varint,

    /// error while parsing message
    ParseMessage {
        s: &'static str,
    },

    /// unexpected map tag
    Map {
        tag: u8
    },

    /// unexpected end of buffer
    UnexpectedEof,

    /// IO underlaying error
    IoError(io::Error),

    /// UTF8 underlaying error
    Utf8Error(Utf8Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Error::Deprecated{ feat } => write!(f, "feature '{}' has been deprecated", feat),
            Error::ParseMessage{ s } => write!(f, "error while parsing message: {}", s),
            Error::UnknownWireType{ t } => write!(f, "wire type must be less than 6, found {}", t),
            Error::Map{ tag } => write!(f, "expecting a tag number 1 or 2, got {}", tag),
            Error::Varint => write!(f, "cannot decode varint"),
            Error::UnexpectedEof => write!(f, "Cannot read next bytes"),
            Error::Utf8Error(_) => write!(f, "Underlaying UTF8 error"),
            Error::IoError(_) => write!(f, "Underlaying IO error"),
        }
    }
}

impl Fail for Error {}

impl From<io::Error> for Error {
	fn from(e: io::Error) -> Error {
		Error::IoError(e)
	}
}

impl From<Utf8Error> for Error {
	fn from(e: Utf8Error) -> Error {
		Error::Utf8Error(e)
	}
}

