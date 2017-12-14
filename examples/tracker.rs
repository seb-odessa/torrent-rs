extern crate torrent;
extern crate log;
extern crate env_logger;

use torrent::Metainfo;
use torrent::Daemon;

use std::io::Read;
use std::fs::File;
use std::io;
use std::env;

fn main() {
    env_logger::init().unwrap();
    match get_content() {
        Ok(buffer) => handle(&buffer).unwrap(),
        Err(e) => println!("ERROR: {:?}", e),        
    }
}

fn handle(buffer: &Vec<u8>) -> Result<(), io::Error> {
    let mut daemon = Daemon::new();
    let metainfo = Metainfo::from(&buffer).map_err(|_| {
        io::Error::new(io::ErrorKind::Other, "Cant create metainfo")
    })?;
    daemon.register(metainfo);
    daemon.update();
    println!("{}", daemon);
    Ok(())
}


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
