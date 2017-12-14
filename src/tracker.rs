use std::collections::HashSet;
use std::collections::hash_map::Entry;
use std::hash::{Hash, Hasher};

use reqwest;

use Error;
use hash::Sha1;
use params::Params;
use daemon::PeerMap;
use metainfo::Metainfo;
use response::Response;

#[derive(Debug)]
pub struct Tracker {
    tracker: String,
    params: Params,
}
impl Hash for Tracker {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.tracker.hash(state);
    }
}
impl PartialEq for Tracker {
    fn eq(&self, other: &Tracker) -> bool {
        self.tracker == other.tracker
    }
}
impl Eq for Tracker {
    //
}
impl Tracker {
    pub fn new(peer_id: &String, metainfo: &Metainfo) -> Self {
        let url = metainfo.announce.clone().unwrap_or_default();
        let params = Params::from(&metainfo, peer_id);
        Tracker {
            tracker: url,
            params: params,
        }
    }

    fn request(&self) -> Result<Response, Error> {
        let mut body = Vec::new();
        let url = format!("{}?{}", &self.tracker, &self.params);
        reqwest::get(&url)?.copy_to(&mut body)?;
        Response::from(&body).map_err(|e| Error::from(e))
    }

    pub fn update_peers(&self, hash: &Sha1, peers: &mut PeerMap) {
        let tracker_response = self.request();
        if tracker_response.is_ok() {
            let response = tracker_response.unwrap();
            info!("Tracker::update_peers(), Response:\n{}", &response);
            for peer in &response.peers {
                peers.entry(hash.clone()).or_insert(HashSet::new());
                if let Entry::Occupied(mut peers) = peers.entry(hash.clone()) {
                    peers.get_mut().insert(peer.clone());
                }
            }
        } else {
            error!("{:?}", tracker_response.unwrap_err());
        }
    }
}
