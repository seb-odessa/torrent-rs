use std::fmt;

use serde_bytes::ByteBuf;
use serde_bencode::{de, Error};
use rustc_serialize::hex::ToHex;

use hash;
use decoder;
use info::Info;
use time::{at, Timespec};


#[derive(Debug, Deserialize)]
pub struct Metainfo {
    pub info: Info,
    pub announce: Option<String>,
    pub nodes: Option<Vec<(String, i64)>>,
    pub encoding: Option<String>,
    pub httpseeds: Option<Vec<String>>,
    #[serde(rename = "announce-list")]
    pub announce_list: Option<Vec<Vec<String>>>,
    #[serde(rename = "creation date")]
    pub creation_date: Option<i64>,
    pub comment: Option<String>,
    #[serde(rename = "created by")]
    pub created_by: Option<String>,
    pub info_hash: Option<ByteBuf>,
}
impl Metainfo {
    pub fn from(buffer: &[u8]) -> Result<Self, Error> {
        let metainfo = de::from_bytes::<Metainfo>(&buffer);
        if let Some(info) = decoder::get_info_bytes(buffer) {
            if metainfo.is_ok() {
                let mut meta = metainfo.unwrap();
                let sha1 = hash::sha1(&ByteBuf::from(info));
                meta.info_hash = Some(ByteBuf::from(sha1));
                return Ok(meta);
            } else {
                return metainfo;
            }
        }
        return Err(Error::Custom(String::from("Can't read INFO")));

    }

    pub fn info_hash(&self) -> Vec<u8> {
        self.info_hash.clone().unwrap_or_default().into()
    }

    fn get_length(&self) -> i64 {
        let mut length = 0;
        if let Some(ref files) = self.info.files {
            for file in files {
                length += file.length;
            }
        } else if let Some(len) = self.info.length {
            length += len;
        }
        return length;
    }

    pub fn get_piece_length(&self, index: i64) -> Option<i64> {
        let length = self.get_length();
        let max_index = length / self.info.piece_length;
        let last_full_piece = max_index * self.info.piece_length;
        if index <= last_full_piece {}

        None
    }
}
impl fmt::Display for Metainfo {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        writeln!(fmt, "name:\t\t{}", self.info.name)?;
        writeln!(
            fmt,
            "announce:\t{}",
            self.announce.clone().unwrap_or_default()
        )?;
        writeln!(fmt, "nodes:\t\t{:?}", self.nodes)?;
        if let &Some(ref al) = &self.announce_list {
            for a in al {
                writeln!(fmt, "announce:\t{}", a[0])?;
            }
        }
        writeln!(fmt, "httpseeds:\t{:?}", self.httpseeds)?;
        writeln!(
            fmt,
            "creation date:\t{}",
            at(Timespec::new(self.creation_date.unwrap_or_default(), 0)).asctime()
        )?;
        writeln!(
            fmt,
            "comment:\t{}",
            self.comment.clone().unwrap_or_default()
        )?;
        writeln!(
            fmt,
            "created by:\t{}",
            self.created_by.clone().unwrap_or_default()
        )?;
        writeln!(
            fmt,
            "encoding:\t{}",
            self.encoding.clone().unwrap_or_default()
        )?;
        writeln!(fmt, "piece length:\t{:?}", self.info.piece_length)?;
        writeln!(fmt, "pieces count:\t{:?}", self.info.pieces.len() / 20)?;
        writeln!(fmt, "length:\t\t{:?}", self.info.length.unwrap_or_default())?;
        if let &Some(ref files) = &self.info.files {
            for f in files {
                writeln!(fmt, "file path:\t{:?}", f.path)?;
                writeln!(fmt, "file length:\t{}", f.length)?;
            }
        }
        let info_hash: hash::Sha1 = self.info_hash.clone().unwrap_or_default().into();
        writeln!(fmt, "hash info:\t{}", info_hash.to_hex().to_uppercase())?;
        write!(fmt, "")
    }
}
