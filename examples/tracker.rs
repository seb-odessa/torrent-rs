extern crate torrent;
extern crate reqwest;
extern crate url;

use torrent::Metainfo;

use std::io;
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
    ];
    let url = format!("{}?{}", &announce, encode_query_params(&params));
    println!("url: {}", &url);
    tracker(&url).unwrap();
}

fn tracker(url: &str) -> Result<Vec<u8>, reqwest::Error> {
    let mut response = reqwest::get(url)?;
    let mut body = Vec::new();
    response.copy_to(&mut body)?;
    println!("Status: {}", response.status());

    if reqwest::StatusCode::Ok == response.status() {
        println!("body.len(): {}", body.len());
        let r = torrent::Response::from(&body);
        println!("{:?}", r);

        use std::io::prelude::*;
        use std::fs::File;
        let mut buffer = File::create("response.bin").unwrap();
        buffer.write(&body).unwrap();
    }

    //println!("body: {:?}", body);
    Ok(body)
}

fn encode_query_params(params: &[(&str, &str)]) -> String {
    let param_strings: Vec<String> = params
        .iter()
        .map(|&(k, v)| format!("{}={}", k, v))
        .collect();
    param_strings.join("&")
}
