//! A module to handle all errors via error-chain crate

#![allow(missing_docs)]

use failure::Error;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug, Fail)]
pub enum ProtoBufError {
    /// deprecated feature
    #[fail(display = "feature '{}' has been deprecated", feat)]
    Deprecated{
        feat: String,
    },

    /// unknown wire type
    #[fail(display = "wire type must be less than 6, found {}", t)]
    UnknownWireType {
        t: u8
    },

    /// cannot decode varint
    #[fail(display = "cannot decode varint")]
    Varint,

    /// error while parsing message
    #[fail(display = "error while parsing message: {}", s)]
    ParseMessage {
        s: String
    },

    /// unexpected map tag
    #[fail(display = "expecting a tag number 1 or 2, got {}", tag)]
    Map {
        tag: u8
    },

    /// unexpected end of buffer
    #[fail(display = "Cannot read next bytes")]
    UnexpectedEof,
}

