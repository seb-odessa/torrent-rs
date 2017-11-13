use std::fmt;
use serde_bencode::{de, Error};
use std::ffi::CString;

#[derive(Debug)]
pub struct Response {
    pub interval: i64,
    pub complete: i64,
    pub incomplete: i64,
    pub peers: Vec<String>,
}

impl Response {
    pub fn from(buffer: &[u8]) -> Result<Self, Error> {
        let response = de::from_bytes::<ResponseCompact>(&buffer)?;
        Ok(Response {
            interval: response.interval.unwrap_or_default(),
            complete: response.complete.unwrap_or_default(),
            incomplete: response.incomplete.unwrap_or_default(),
            peers: Self::get_peers(&response),
        })
    }

    fn get_peers(response: &ResponseCompact) -> Vec<String> {
        let mut peers = Vec::new();
        const BYTES_PER_PEER: usize = 6;

        if let Some(ref records) = response.peers {
            let bytes = records.clone().into_bytes();
            let mut it = bytes.chunks(BYTES_PER_PEER);
            while let Some(chunk) = it.next() {
                let ip = format!(
                    "{}.{}.{}.{}:{}",
                    chunk[0],
                    chunk[1],
                    chunk[2],
                    chunk[3],
                    0xFF * chunk[4] as u32 + chunk[5] as u32
                );
                peers.push(ip);
            }
        }
        peers
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
        write!(fmt,"")
    }
}


#[derive(Debug, Deserialize)]
struct ResponseCompact {
    #[serde(rename = "failure reason")] pub failure_reason: Option<String>,
    #[serde(rename = "warning message")] pub warning_message: Option<String>,
    pub interval: Option<i64>,
    pub complete: Option<i64>,
    pub incomplete: Option<i64>,
    pub peers: Option<CString>,
}
