extern crate crypto;
extern crate rand;
extern crate serde_bencode;
extern crate reqwest;
extern crate time;
extern crate rustc_serialize;
extern crate serde_bytes;
extern crate env_logger;
extern crate byteorder;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;

use std::io;
use std::convert;

pub mod hash;
mod info;
mod params;
mod metainfo;
mod response;
mod tracker;
mod decoder;
mod daemon;

pub use info::File;
pub use info::Info;

pub use metainfo::Metainfo;
pub use response::Peer;
pub use response::Response;
pub use daemon::Daemon;
pub use daemon::generate_peer_id;

#[derive(Debug)]
pub enum Error {
    ReqwestError(reqwest::Error),
    DecoderError(serde_bencode::Error),
    IoError(io::Error),
}

impl convert::From<serde_bencode::Error> for Error {
    fn from(err: serde_bencode::Error) -> Error {
        Error::DecoderError(err)
    }
}

impl convert::From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Error {
        Error::ReqwestError(err)
    }
}

impl convert::From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IoError(err)
    }
}
