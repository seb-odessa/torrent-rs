extern crate crypto;
extern crate rand;
extern crate serde_bencode;
extern crate reqwest;
extern crate time;
extern crate url;

extern crate serde_bytes;
#[macro_use]
extern crate serde_derive;

mod hash;
mod info;
mod metainfo;
mod response;
mod tracker;

pub use hash::Sha1;
pub use info::File;
pub use info::Info;

pub use metainfo::Metainfo;
pub use response::Peer;
pub use response::Response;
pub use tracker::TrackerDaemon;
pub use tracker::generate_peer_id;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
