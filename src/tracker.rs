use std::io;
use std::convert;
use std::fmt;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::collections::HashSet;

use reqwest;
use rand;
use rand::Rng;
use serde_bencode;

use rustc_serialize::hex::ToHex;

use hash::Sha1;
use response::{Response, Peer};
use metainfo::Metainfo;
use params::Params;

#[derive(Debug)]
pub struct TrackerDaemon {
    torrents: HashMap<Sha1, Metainfo>,
    peers: HashMap<Sha1, HashSet<Peer>>,
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
        for (key, metainfo) in &self.torrents {
            println!("{}", metainfo);
            if let Some(response) = get_peers_from_anounce(&metainfo, &self.peer_id).ok() {
                println!("Tracker Response received:\n{}", response);
                for peer in &response.peers {
                    println!("Inserting peer {} to the Set.", &peer);
                    self.peers.entry(key.clone()).or_insert(HashSet::new());
                    if let Entry::Occupied(mut peers) = self.peers.entry(key.clone()) {
                        peers.get_mut().insert(peer.clone());
                    }
                }
            }
        }
    }
}
impl fmt::Display for TrackerDaemon {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        for (sha1, metainfo) in &self.torrents {
            writeln!(fmt, "SHA1: {} => {}", sha1.to_hex(), metainfo.info.name)?;
        }
        for (sha1, peers) in &self.peers {
            for peer in peers {
                writeln!(fmt, "SHA1: {:?} => {}", sha1.to_hex(), peer)?;
            }
        }
        write!(fmt, "")
    }
}

#[derive(Debug)]
pub enum Error {
    ReqwestError(reqwest::Error),
    ReqwestStatus(reqwest::StatusCode),
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

pub fn generate_peer_id() -> String {
    const PEER_ID_PREFIX: &'static str = "-RT0002-";
    let mut rng = rand::thread_rng();
    let rand_chars: String = rng.gen_ascii_chars()
        .take(20 - PEER_ID_PREFIX.len())
        .collect();
    format!("{}{}", PEER_ID_PREFIX, rand_chars)
}

fn get_peers_from_anounce(metainfo: &Metainfo, id: &String) -> Result<Response, Error> {
    let announce = metainfo.announce.clone().unwrap_or_default();
    let param = Params::from(metainfo, id);
    let url = format!("{}?{}", &announce, &param);
    let mut response = reqwest::get(&url)?;
    let mut body = Vec::new();
    response.copy_to(&mut body)?;
    if reqwest::StatusCode::Ok == response.status() {
        Ok(Response::from(&body)?)
    } else {
        Err(Error::ReqwestStatus(response.status()))
    }
}