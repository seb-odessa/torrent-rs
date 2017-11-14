extern crate torrent;
extern crate reqwest;
extern crate serde_bencode;
extern crate url;

use torrent::Metainfo;
use torrent::TrackerDaemon;

use std::io;
use std::io::Read;

fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut buffer = Vec::new();
    let mut daemon = TrackerDaemon::new();
    match handle.read_to_end(&mut buffer) {
        Ok(_) => {
            match Metainfo::from(&buffer) {
                Ok(metainfo) => {
                    daemon.register(metainfo);
                }
                Err(e) => println!("ERROR: {:?}", e),
            }
        }
        Err(e) => println!("ERROR: {:?}", e),
    }
    daemon.update();
}
