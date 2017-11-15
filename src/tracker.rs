use std::io;
use std::convert;
use std::fmt;
use std::collections::HashMap;

use reqwest;
use rand;
use rand::Rng;
use serde_bencode;
use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};

use hash::Sha1;
use response::{Response, Peer};
use metainfo::Metainfo;

#[derive(Debug)]
pub struct TrackerDaemon {
    torrents: HashMap<Sha1, Metainfo>,
    peers: HashMap<Sha1, Peer>,
    peer_id: String,
}
impl TrackerDaemon {
    pub fn new() -> Self {
        TrackerDaemon {
            torrents: HashMap::new(),
            peers: HashMap::new(),
            peer_id: generate_peer_id(),
        }
    }

    pub fn register(&mut self, metainfo: Metainfo) {
        let hash = metainfo.info.sha1();
        self.torrents.insert(hash, metainfo);
    }

    pub fn update(&mut self) {
        for (_, metainfo) in &self.torrents {
            println!("{}", metainfo);
            let param = Params::from(metainfo, &self.peer_id);
            let announce = metainfo.announce.clone().unwrap_or_default();
            let url = format!("{}?{}", &announce, param.query());
            println!("URL: {}", &url);
            if let Some(response) = get_peers(&url).ok() {
                println!("Tracker Response:\n{}", response);
            }
        }
    }
}
impl fmt::Display for TrackerDaemon {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        for (sha1, metainfo) in &self.torrents {
            writeln!(fmt, "{:?} => {}", sha1, metainfo)?;
        }
        write!(fmt, "")
    }
}

#[derive(Debug)]
struct Params {
    params: HashMap<&'static str, String>,
}
impl Params {
    pub fn from(metainfo: &Metainfo, id: &String) -> Self {
        let length = metainfo.info.length.unwrap_or_default().to_string();
        let info_hash = percent_encode(&metainfo.info.sha1(), DEFAULT_ENCODE_SET).to_string();
        let mut params = HashMap::new();
        params.insert("left", length);
        params.insert("info_hash", info_hash);
        params.insert("downloaded", String::from("0"));
        params.insert("uploaded", String::from("0"));
        params.insert("event", String::from("started"));
        params.insert("peer_id", id.clone());
        params.insert("compact", String::from("1"));
        params.insert("port", String::from("6881"));
        return Params { params };
    }

    pub fn query(&self) -> String {
        let param_strings: Vec<String> = self.params
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect();
        param_strings.join("&")
    }
}
#[derive(Debug)]
pub enum Error {
    ReqwestError(reqwest::Error),
    ReqwestStatus(reqwest::StatusCode),
    DecoderError(serde_bencode::Error),
    IoError(io::Error)
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

pub fn generate_peer_id() -> String {
    const PEER_ID_PREFIX: &'static str = "-RT0002-";
    let mut rng = rand::thread_rng();
    let rand_chars: String = rng.gen_ascii_chars()
        .take(20 - PEER_ID_PREFIX.len())
        .collect();
    format!("{}{}", PEER_ID_PREFIX, rand_chars)
}

fn get_peers(url: &str) -> Result<Response, Error> {
    let mut response = reqwest::get(url)?;
    let mut body = Vec::new();
    response.copy_to(&mut body)?;
    if reqwest::StatusCode::Ok == response.status() {
        Ok(Response::from(&body)?)
    } else {
        Err(Error::ReqwestStatus(response.status()))
    }
}