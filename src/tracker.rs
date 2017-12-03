use std::io;
use std::convert;
use std::fmt;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::collections::HashSet;

use rand;
use rand::Rng;
use serde_bencode;
use reqwest;
use rustc_serialize::hex::ToHex;

use hash::Sha1;
use response::Peer;
use metainfo::Metainfo;
use params::Params;
use http_tracker::HttpTracker;

pub type TorrentMap = HashMap<Sha1, Metainfo>;
pub type TrackerMap = HashMap<Sha1, HashSet<HttpTracker>>;
pub type PeerMap = HashMap<Sha1, HashSet<Peer>>;


#[derive(Debug)]
pub struct TrackerDaemon {
    peer_id: String,
    torrents: TorrentMap,
    trackers: TrackerMap,
    peers: PeerMap,
}
impl TrackerDaemon {
    pub fn new() -> Self {
        TrackerDaemon {
            peer_id: generate_peer_id(),
            torrents: TorrentMap::new(),
            trackers: TrackerMap::new(),
            peers: PeerMap::new(),
        }
    }

    pub fn register(&mut self, metainfo: Metainfo) {
        let hash: Sha1 = metainfo.info_hash();
        let url = metainfo.announce.clone().unwrap_or_default();
        let params = Params::from(&metainfo, &self.peer_id);

        self.torrents.insert(hash.clone(), metainfo);

        let tracker = HttpTracker::new(url, params);
        self.trackers.entry(hash.clone()).or_insert(HashSet::new());
        if let Entry::Occupied(mut trackers) = self.trackers.entry(hash.clone()) {
            trackers.get_mut().insert(tracker);
        }


    }

    pub fn update(&mut self) {
        for (hash, metainfo) in &self.torrents {
            info!("TrackerDaemon::update():\n{}", metainfo);
            if let Entry::Occupied(trackers) = self.trackers.entry(hash.clone()) {
                for tracker in trackers.get() {
                    tracker.update_peers(&hash, &mut self.peers);
                }
            }
        }
    }
}
impl fmt::Display for TrackerDaemon {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        for (sha1, metainfo) in &self.torrents {
            writeln!(
                fmt,
                "SHA1: {} => {}",
                sha1.to_hex().to_uppercase(),
                metainfo.info.name
            )?;
        }
        for (sha1, peers) in &self.peers {
            for peer in peers {
                writeln!(fmt, "SHA1: {} => {}", sha1.to_hex().to_uppercase(), peer)?;
            }
        }
        write!(fmt, "")
    }
}

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

pub fn generate_peer_id() -> String {
    const PEER_ID_PREFIX: &'static str = "-RT0002-";
    let mut rng = rand::thread_rng();
    let rand_chars: String = rng.gen_ascii_chars()
        .take(20 - PEER_ID_PREFIX.len())
        .collect();
    format!("{}{}", PEER_ID_PREFIX, rand_chars)
}
