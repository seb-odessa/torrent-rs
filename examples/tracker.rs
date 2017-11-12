extern crate hyper;
extern crate tokio_core;
extern crate torrent;
extern crate url;

use torrent::Metainfo;

use std::io;
use std::io::Read;
use tokio_core::reactor::Core;
use hyper::Client;
use hyper::header::Connection;
use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};

fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut buffer = Vec::new();
    match handle.read_to_end(&mut buffer) {
        Ok(_) => match Metainfo::from(&buffer) {
            Ok(metainfo) => {
                println!("{}", &metainfo);
                let id = torrent::generate_peer_id();
                println!("Peer Id: {}", id);
                run(&metainfo, &id);
            }
            Err(e) => println!("ERROR: {:?}", e),
        },
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

fn tracker(url: &str)->Result<Vec<u8>, hyper::Error>{
//    let uri = url.parse()?;
    let mut core = Core::new()?;
    let client = Client::new(&core.handle());
    // let response = client.get(uri).header(Connection::close()).send()?;
    // println!("http response: {:?}", &response);

    let mut body = Vec::new();
    // response.read_to_end(&mut body)?;
    println!("body.len(): {:?}", body.len());
    Ok(body)
}

fn encode_query_params(params: &[(&str, &str)]) -> String {
    let param_strings: Vec<String> = params
        .iter()
        .map(|&(k, v)| format!("{}={}", k, v))
        .collect();
    param_strings.join("&")
}
