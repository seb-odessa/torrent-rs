use std::fmt;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::collections::HashSet;

use rand;
use rand::Rng;
use rustc_serialize::hex::ToHex;

use hash::Sha1;
use response::Peer;
use metainfo::Metainfo;
use tracker::Tracker;

pub type TorrentMap = HashMap<Sha1, Metainfo>;
pub type TrackerMap = HashMap<Sha1, HashSet<Tracker>>;
pub type PeerMap = HashMap<Sha1, HashSet<Peer>>;


#[derive(Debug)]
pub struct Daemon {
    peer_id: String,
    torrents: TorrentMap,
    trackers: TrackerMap,
    peers: PeerMap,
}
impl Daemon {
    pub fn new() -> Self {
        Daemon {
            peer_id: generate_peer_id(),
            torrents: TorrentMap::new(),
            trackers: TrackerMap::new(),
            peers: PeerMap::new(),
        }
    }

    pub fn register(&mut self, metainfo: Metainfo) {
        let hash: Sha1 = metainfo.info_hash();
        let tracker = Tracker::new(&self.peer_id, &metainfo);
        self.trackers.entry(hash.clone()).or_insert(HashSet::new());
        if let Entry::Occupied(mut trackers) = self.trackers.entry(hash.clone()) {
            trackers.get_mut().insert(tracker);
        }
        self.torrents.insert(hash.clone(), metainfo);
    }

    pub fn update(&mut self) {
        for (hash, metainfo) in &self.torrents {
            info!("Daemon::update():\n{}", metainfo);
            if let Entry::Occupied(trackers) = self.trackers.entry(hash.clone()) {
                for tracker in trackers.get() {
                    tracker.update_peers(&hash, &mut self.peers);
                }
            }
        }
    }
}
impl fmt::Display for Daemon {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        for (_, metainfo) in &self.torrents {
            writeln!(fmt, "{}", metainfo)?;
        }
        for (sha1, peers) in &self.peers {
            for peer in peers {
                writeln!(fmt, "SHA1: {} => {}", sha1.to_hex().to_uppercase(), peer)?;
            }
        }
        write!(fmt, "")
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
