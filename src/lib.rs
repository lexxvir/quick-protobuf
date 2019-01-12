//! A library to read binary protobuf files
//!
//! This reader is developed similarly to a pull reader

#![deny(missing_docs)]
#![recursion_limit = "1024"]
#![allow(dead_code)]

#![cfg_attr(not(feature="std"),no_std)]
#![cfg_attr(all(not(feature="std"),feature="collections"),feature(collections))]

extern crate failure;

#[cfg(feature = "std")]
extern crate byteorder;

#[cfg(feature = "core_io_collections")]
extern crate byteorder_core_io as byteorder;

#[cfg(feature = "core_io_collections")]
extern crate collections;

#[cfg(feature = "core_io_collections")]
extern crate core_io;

#[cfg(feature = "core_io_collections")]
pub(crate) mod std {
    pub(crate) mod io {
        pub(crate) use core_io::*;
    }

    pub(crate) mod slice {
        pub(crate) use core::slice::*;
    }

    pub(crate) mod mem {
        pub(crate) use core::mem::*;
    }

    pub(crate) mod fmt {
        pub(crate) use core::fmt::*;
    }

    pub(crate) mod str {
        pub(crate) use core::str::*;
    }

    pub(crate) mod result {
        pub(crate) use core::result::*;
    }
}

pub mod errors;
pub mod message;
pub mod reader;
pub mod writer;
pub mod sizeofs;
pub mod into_owned;

pub use errors::Result;
pub use message::{MessageRead, MessageWrite};
pub use reader::{BytesReader, Reader};
pub use writer::Writer;
pub use into_owned::IntoOwned;
