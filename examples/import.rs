extern crate rusqlite;
extern crate rustc_serialize;
extern crate time;
extern crate torrent;

use rusqlite::Connection;
use torrent::Metainfo;
use rustc_serialize::hex::ToHex;

use std::error::Error;
use std::io::Read;
use std::fs::File;
use std::io;
use std::env;

const DATABASE_FILE: &'static str = "lib.rus.ec.db";

const INSERT_ARCHIVE: &'static str = "INSERT INTO archives (name, created, hash, total_length, piece_length, pieces_count)
VALUES (?, ?, ?, ?, ?, ?)";

const INSERT_PIECE: &'static str = "INSERT INTO pieces (archive_id, piece_idx, hash) VALUES (?, ?, ?)";

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

fn insert_metainfo(metainfo: &Metainfo, conn: &Connection) -> Result<i64, rusqlite::Error> {
    conn.execute(
        INSERT_ARCHIVE,
        &[
            &metainfo.get_file_name(),
            &metainfo.get_creation_date(),
            &metainfo.get_info_hash(),
            &(metainfo.get_length() as i64),
            &(metainfo.get_piece_length() as i64),
            &(metainfo.get_piece_count() as i64),
        ],
    )?;
    Ok(conn.last_insert_rowid())
}

fn insert_pieces(
    metainfo: &Metainfo,
    archive_id: i64,
    conn: &mut Connection,
) -> Result<(), rusqlite::Error> {
    let tx = conn.transaction()?;
    {
        let mut stmt = tx.prepare(INSERT_PIECE)?;
        let pieces: &[u8] = metainfo.info.pieces.as_ref();
        let mut index = 0;
        for hash in pieces.chunks(20) {
            stmt.execute(&[&archive_id, &index, &hash.to_hex()])?;
            index += 1;
        }
    }
    tx.commit()?;
    Ok(())
}

fn upload(metainfo: Metainfo) -> Result<(), rusqlite::Error> {
    println!("file name:     {}", &metainfo.get_file_name());
    println!("creation date: {}", &metainfo.get_creation_date());
    println!("info hash:     {}", &metainfo.get_info_hash());
    println!("total length:  {}", &metainfo.get_length());
    println!("piece length:  {}", &metainfo.get_piece_length());
    println!("piece count:   {}", &metainfo.get_piece_count());

    let mut conn = Connection::open(DATABASE_FILE)?;
    let archive_id = insert_metainfo(&metainfo, &conn)?;
    insert_pieces(&metainfo, archive_id, &mut conn)
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
