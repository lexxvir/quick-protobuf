//! A module to handle all errors via error-chain crate

#![allow(missing_docs)]

use std::str::Utf8Error;
use std::io;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    /// deprecated feature
    // TODO #[fail(display = "feature '{}' has been deprecated", feat)]
    Deprecated {
        feat: &'static str,
    },

    /// unknown wire type
    // TODO #[fail(display = "wire type must be less than 6, found {}", t)]
    UnknownWireType {
        t: u8
    },

    /// cannot decode varint
    // TODO #[fail(display = "cannot decode varint")]
    Varint,

    /// error while parsing message
    // TODO #[fail(display = "error while parsing message: {}", s)]
    ParseMessage {
        s: &'static str,
    },

    /// unexpected map tag
    // TODO #[fail(display = "expecting a tag number 1 or 2, got {}", tag)]
    Map {
        tag: u8
    },

    /// unexpected end of buffer
    // TODO #[fail(display = "Cannot read next bytes")]
    UnexpectedEof,
    IoError(io::Error),
    Utf8Error(Utf8Error),
}

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

