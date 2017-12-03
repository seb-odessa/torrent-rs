extern crate torrent;
extern crate rustc_serialize;

use torrent::Metainfo;
use torrent::hash::SHA1_LEN;
use rustc_serialize::hex::ToHex;

use std::io::Read;
use std::fs::File;
use std::io;
use std::env;

fn get_content() -> Result<Vec<u8>, io::Error> {
    let args = env::args().collect::<Vec<_>>();
    let mut buffer = Vec::new();
    if 1 == args.len() {
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        handle.read_to_end(&mut buffer)?
    } else {
        File::open(&args[1])?.read_to_end(&mut buffer)?
    };
    Ok(buffer)
}

fn main() {

    match get_content() {
        Ok(buffer) => {
            match Metainfo::from(&buffer) {
                Ok(metainfo) => {
                    println!("{}", &metainfo);
                    let pieces: &[u8] = metainfo.info.pieces.as_ref();
                    let mut index = 0;
                    for sha1 in pieces.chunks(SHA1_LEN) {
                        println!("{:>6} {}", index, sha1.to_hex().to_uppercase());
                        index += 1;
                    }
                }
                Err(e) => println!("ERROR: {:?}", e),
            }
        }
        Err(e) => println!("ERROR: {:?}", e),
    }
}
