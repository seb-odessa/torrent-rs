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

    pub fn get_file_name(&self) -> String {
        self.info.name.clone()
    }

    pub fn info_hash(&self) -> Vec<u8> {
        self.info_hash.clone().unwrap_or_default().into()
    }

    pub fn get_creation_date(&self) -> String {
        let time = self.creation_date.unwrap_or_default();
        format!("{}", at(Timespec::new(time, 0)).asctime())
    }

    pub fn get_length(&self) -> u64 {
        let mut length = 0;
        if let Some(ref files) = self.info.files {
            for file in files {
                length += file.length;
            }
        } else if let Some(len) = self.info.length {
            length += len;
        }
        return length as u64;
    }

    pub fn get_piece_length(&self) -> u64 {
        self.info.piece_length as u64
    }

    pub fn get_piece_count(&self) -> u64 {
        let count = self.get_length() / self.get_piece_length();
        if 0 != self.get_length() - count * self.get_piece_length() {
            count + 1
        } else {
            count
        }
    }
    pub fn get_info_hash(&self) -> String {
        let info_hash: hash::Sha1 = self.info_hash.clone().unwrap_or_default().into();
        info_hash.to_hex().to_uppercase()
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
        writeln!(fmt, "piece length:\t{:?}", self.get_piece_length())?;
        writeln!(fmt, "pieces count:\t{:?}", self.get_piece_count())?;
        writeln!(fmt, "length:\t\t{:?}", self.get_length())?;
        if let &Some(ref files) = &self.info.files {
            for f in files {
                writeln!(fmt, "file path:\t{:?}", f.path)?;
                writeln!(fmt, "file length:\t{}", f.length)?;
            }
        }
        writeln!(fmt, "hash info:\t{}", self.get_info_hash())?;
        write!(fmt, "")
    }
}
