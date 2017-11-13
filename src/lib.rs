extern crate crypto;
extern crate rand;
extern crate serde_bencode;
extern crate serde_bytes;
#[macro_use]
extern crate serde_derive;
extern crate time;

mod hash;
mod info;
mod metainfo;
mod response;
pub use info::File;
pub use info::Info;
pub use metainfo::Metainfo;
pub use response::Response;

use rand::Rng;
pub fn generate_peer_id() -> String {
    const PEER_ID_PREFIX: &'static str = "-RT0002-";
    let mut rng = rand::thread_rng();
    let rand_chars: String = rng.gen_ascii_chars()
        .take(20 - PEER_ID_PREFIX.len())
        .collect();
    format!("{}{}", PEER_ID_PREFIX, rand_chars)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
