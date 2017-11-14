extern crate torrent;
extern crate reqwest;
extern crate serde_bencode;
extern crate url;

use torrent::Metainfo;

use std::io;
use std::convert;
use std::io::Read;

use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};

fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut buffer = Vec::new();
    match handle.read_to_end(&mut buffer) {
        Ok(_) => {
            match Metainfo::from(&buffer) {
                Ok(metainfo) => {
                    println!("{}", &metainfo);
                    let id = torrent::generate_peer_id();
                    println!("Peer Id: {}", id);
                    run(&metainfo, &id);
                }
                Err(e) => println!("ERROR: {:?}", e),
            }
        }
        Err(e) => println!("ERROR: {:?}", e),
    }
}

fn run(metainfo: &Metainfo, id: &str) {
    let announce = metainfo.announce.clone().unwrap_or_default();
    let length = metainfo.info.length.unwrap_or_default().to_string();
    let info_hash = percent_encode(&metainfo.info.sha1(), DEFAULT_ENCODE_SET).to_string();
    let params = vec![
        ("left", length.as_ref()),
        ("info_hash", info_hash.as_ref()),
        ("downloaded", "0"),
        ("uploaded", "0"),
        ("event", "started"),
        ("peer_id", id),
        ("compact", "1"),
        ("port", "6881"),
        ("ip", "192.168.0.100"),
    ];
    let url = format!("{}?{}", &announce, encode_query_params(&params));
    println!("URL: {}", &url);
    tracker(&url).unwrap();
}

fn tracker(url: &str) -> Result<Vec<u8>, Error> {
    let mut response = reqwest::get(url)?;
    let mut body = Vec::new();
    response.copy_to(&mut body)?;
    if reqwest::StatusCode::Ok == response.status() {
        let data = torrent::Response::from(&body)?;
        println!("Tracker Response:\n{}", data);
    } else {
        println!("Tracker Response Status is :{}", response.status());
    }
    Ok(body)
}

fn encode_query_params(params: &[(&str, &str)]) -> String {
    let param_strings: Vec<String> = params
        .iter()
        .map(|&(k, v)| format!("{}={}", k, v))
        .collect();
    param_strings.join("&")
}

#[derive(Debug)]
pub enum Error {
    ReqwestError(reqwest::Error),
    DecoderError(serde_bencode::Error),
    IoError(io::Error),
}

impl convert::From<serde_bencode::Error> for Error {
    fn from(err: serde_bencode::Error) -> Error {
        Error::DecoderError(err)
    }
}

impl convert::From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Error {
        Error::ReqwestError(err)
    }
}

impl convert::From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IoError(err)
    }
}
