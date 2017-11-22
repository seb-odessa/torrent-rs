use std::collections::HashSet;
use std::collections::hash_map::Entry;
use std::hash::{Hash, Hasher};

use reqwest;

use hash::Sha1;
use params::Params;
use tracker::PeerMap;
use response::Response;
use tracker::Error;


#[derive(Debug)]
pub struct HttpTracker {
    tracker: String,
    params: Params,
}
impl Hash for HttpTracker {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.tracker.hash(state);
    }
}
impl PartialEq for HttpTracker {
    fn eq(&self, other: &HttpTracker) -> bool {
        self.tracker == other.tracker
    }
}
impl Eq for HttpTracker {
    //
}
impl HttpTracker {
    pub fn new(url: String, params: Params) -> Self {
        HttpTracker {
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
        if tracker_response.is_err() {
            error!(
                "HttpTracker::update_peers(): {:?}",
                tracker_response.unwrap_err()
            );
            return;
        }
        let response = tracker_response.unwrap();
        info!("HttpTracker::update_peers(), Response:\n{}", &response);
        for peer in &response.peers {
            peers.entry(hash.clone()).or_insert(HashSet::new());
            if let Entry::Occupied(mut peers) = peers.entry(hash.clone()) {
                peers.get_mut().insert(peer.clone());
            }
        }
    }
}
