extern crate torrent;
use torrent::Metainfo;
use std::io::Read;
use std::io;

fn main() {
    let stdin = io::stdin();
    let mut buffer = Vec::new();
    let mut handle = stdin.lock();
    match handle.read_to_end(&mut buffer) {
        Ok(_) => {
            match Metainfo::from(&buffer) {
                Ok(metainfo) => println!("{}", &metainfo),
                Err(e) => println!("ERROR: {:?}", e),
            }
        }
        Err(e) => println!("ERROR: {:?}", e),

    }
}
