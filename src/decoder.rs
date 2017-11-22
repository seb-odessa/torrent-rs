extern crate bencode;

use self::bencode::Bencode;
use self::bencode::util::ByteString;
use std::{convert, io};

#[macro_export]
macro_rules! get_field_with_default {
    ($m:expr, $field:expr, $default:expr) => (
        match $m.get(&ByteString::from_str($field)) {
            Some(a) => try!(FromBencode::from_bencode(a)),
            None => $default
        }
    )
}

#[macro_export]
macro_rules! get_field {
    ($m:expr, $field:expr) => (
        get_field_with_default!($m, $field, return Err(decoder::Error::DoesntContain($field)))
    )
}

#[macro_export]
macro_rules! get_optional_field {
    ($m:expr, $field:expr) => (
        get_field_with_default!($m, $field, None)
    )
}

#[macro_export]
macro_rules! get_raw_field {
    ($m:expr, $field:expr) => (
        match $m.get(&ByteString::from_str($field)) {
            Some(a) => a,
            None => return Err(Error::DoesntContain($field))
        }
    )
}

#[macro_export]
macro_rules! get_field_as_bencoded_bytes {
    ($m:expr, $field:expr) => (
        try!(get_raw_field!($m, $field).to_bytes())
    )
}

#[macro_export]
macro_rules! get_field_as_bytes {
    ($m:expr, $field:expr) => (
        match get_raw_field!($m, $field) {
            &Bencode::ByteString(ref v) => v.clone(),
            _ => return Err(decoder::Error::NotAByteString)
        }
    )
}

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    DecodingError(bencode::streaming::Error),
    DoesntContain(&'static str),
    NotANumber(bencode::NumFromBencodeError),
    NotAString(bencode::StringFromBencodeError),
}

impl convert::From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IoError(err)
    }
}

impl convert::From<bencode::streaming::Error> for Error {
    fn from(err: bencode::streaming::Error) -> Error {
        Error::DecodingError(err)
    }
}

impl convert::From<bencode::NumFromBencodeError> for Error {
    fn from(err: bencode::NumFromBencodeError) -> Error {
        Error::NotANumber(err)
    }
}

impl convert::From<bencode::StringFromBencodeError> for Error {
    fn from(err: bencode::StringFromBencodeError) -> Error {
        Error::NotAString(err)
    }
}

pub fn get_info_bytes(metainfo: &[u8]) -> Option<Vec<u8>> {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(metainfo);
    extract_info_bytes(bytes).ok()
}

fn extract_info_bytes(metainfo: Vec<u8>) -> Result<Vec<u8>, Error> {
    if let Some(bencoded) = bencode::from_vec(metainfo).ok() {
        match bencoded {
            Bencode::Dict(ref m) => {
                let info: Vec<_> = get_field_as_bencoded_bytes!(m, "info");
                return Ok(info);
            }
            _ => {}
        }
    }
    Err(Error::DoesntContain("info map"))
}
