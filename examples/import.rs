extern crate torrent;
extern crate rustc_serialize;
extern crate rusqlite;
extern crate time;

use rusqlite::Connection;
use torrent::Metainfo;

use std::error::Error;
use std::io::Read;
use std::fs::File;
use std::io;
use std::env;

const DATABASE_FILE: &'static str = "lib.rus.ec.db";
const INSERT_ARCHIVE: &'static str =
"INSERT INTO
    archives (name, created, hash_info, total_length, piece_length, pieces_count)
    VALUES (?, ?, ?, ?, ?, ?)";

fn load() -> Result<Vec<u8>, io::Error> {
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

fn upload(metainfo: Metainfo) -> Result<(), rusqlite::Error> {
    println!("file name:     {}", &metainfo.get_file_name());
    println!("creation date: {}", &metainfo.get_creation_date());
    println!("info hash:     {}", &metainfo.get_info_hash());
    println!("total length:  {}", &metainfo.get_length());
    println!("piece length:  {}", &metainfo.get_piece_length());
    println!("piece count:   {}", &metainfo.get_piece_count());

    let conn = Connection::open(DATABASE_FILE)?;
    let mut stmt = conn.prepare(INSERT_ARCHIVE)?;
    stmt.execute(&[
        &metainfo.get_file_name(),
        &metainfo.get_creation_date(),
        &metainfo.get_info_hash(),
        &(metainfo.get_length() as i64),
        &(metainfo.get_piece_length() as i64),
        &(metainfo.get_piece_count() as i64)
        ])?;
    Ok(())
}

fn insert(data: Metainfo) -> Result<(), io::Error> {
    upload(data).map_err(|e| io::Error::new(io::ErrorKind::Other, e.description()))
}

fn parse(data: Vec<u8>) -> Result<Metainfo, io::Error> {
    Metainfo::from(&data).map_err(|e| io::Error::new(io::ErrorKind::Other, e.description()))
}

fn main() {
    println!("{:?}", load().and_then(parse).and_then(insert));
}
