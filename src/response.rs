use std::fmt;
use serde_bencode::{de, Error};
use serde_bytes::ByteBuf;
use std::ops::Deref;
use byteorder::{ByteOrder, BigEndian};

const BYTES_PER_PEER: usize = 6;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Peer {
    host: String,
    port: u32,
}
impl Peer {
    pub fn from(chunk: &[u8]) -> Result<Self, Error> {
        if chunk.len() != BYTES_PER_PEER {
            Err(Error::Custom(
                String::from("Chunk length is not equal to BYTES_PER_PEER"),
            ))
        } else {
            let le = BigEndian::read_u16(&chunk[4..6]);
            let be = BigEndian::read_u16(&chunk[4..6]);
            println!(
                "{:X} {:X}, le = {:X}, be = {:X}",
                chunk[4],
                chunk[5],
                le,
                be
            );
            Ok(Peer {
                host: format!("{}.{}.{}.{}", chunk[0], chunk[1], chunk[2], chunk[3]),
                port: 0xFF * chunk[4] as u32 + chunk[5] as u32,
            })
        }
    }
}
impl fmt::Display for Peer {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(fmt, "{}:{}", self.host, self.port)
    }
}


#[derive(Debug)]
pub struct Response {
    pub interval: i64,
    pub complete: i64,
    pub incomplete: i64,
    pub peers: Vec<Peer>,
}
impl Response {
    pub fn from(buffer: &[u8]) -> Result<Self, Error> {
        let response = de::from_bytes::<ResponseCompact>(&buffer)?;
        if let Some(failure) = response.failure_reason {
            Err(Error::Custom(failure))
        } else {
            Ok(Response {
                interval: response.interval.unwrap_or_default(),
                complete: response.complete.unwrap_or_default(),
                incomplete: response.incomplete.unwrap_or_default(),
                peers: Self::peers(&response)?,
            })
        }
    }

    fn peers(response: &ResponseCompact) -> Result<Vec<Peer>, Error> {
        let mut peers = Vec::new();
        if let Some(ref records) = response.peers {
            let mut it = records.deref().chunks(BYTES_PER_PEER);
            while let Some(chunk) = it.next() {
                let peer = Peer::from(chunk);
                peers.push(peer?);
            }
        }
        Ok(peers)
    }
}
impl fmt::Display for Response {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        writeln!(fmt, "Interval:\t\t{}", self.interval)?;
        writeln!(fmt, "Seeders:\t\t{}", self.complete)?;
        writeln!(fmt, "Leechers:\t\t{}", self.incomplete)?;
        for peer in &self.peers {
            writeln!(fmt, "\tPeer:\t{}", peer)?;
        }
        write!(fmt, "")
    }
}


#[derive(Debug, Deserialize)]
struct ResponseCompact {
    #[serde(rename = "failure reason")]
    pub failure_reason: Option<String>,
    pub interval: Option<i64>,
    pub complete: Option<i64>,
    pub incomplete: Option<i64>,
    pub peers: Option<ByteBuf>,
}
