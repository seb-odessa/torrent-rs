use std::fmt;
use serde_bencode::{de, Error};
use info::Info;
use time::{at, Timespec};

#[derive(Debug, Deserialize)]
pub struct Metainfo {
    pub info: Info,
    pub announce: Option<String>,
    pub nodes: Option<Vec<(String, i64)>>,
    pub encoding: Option<String>,
    pub httpseeds: Option<Vec<String>>,
    #[serde(rename = "announce-list")] pub announce_list: Option<Vec<Vec<String>>>,
    #[serde(rename = "creation date")] pub creation_date: Option<i64>,
    pub comment: Option<String>,
    #[serde(rename = "created by")] pub created_by: Option<String>,
}
impl Metainfo {
    pub fn from(buffer: &[u8]) -> Result<Self, Error> {
        de::from_bytes::<Metainfo>(&buffer)
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
        if index <= last_full_piece {

        }

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
        writeln!(fmt, "length:\t\t{:?}", self.info.length.unwrap_or_default())?;
        if let &Some(ref files) = &self.info.files {
            for f in files {
                writeln!(fmt, "file path:\t{:?}", f.path)?;
                writeln!(fmt, "file length:\t{}", f.length)?;
            }
        }
        write!(fmt, "")
    }
}
