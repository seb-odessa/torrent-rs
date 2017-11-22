extern crate torrent;
extern crate reqwest;
extern crate serde_bencode;
extern crate url;

extern crate log;
extern crate env_logger;

use torrent::Metainfo;
use torrent::TrackerDaemon;

use std::io;
use std::io::Read;

fn main() {
    env_logger::init().unwrap();
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
    println!("{}", daemon);
}
