use std::fmt;
use std::collections::HashMap;
use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
use metainfo::Metainfo;

#[derive(Debug)]
pub struct Params {
    params: HashMap<&'static str, String>,
}
impl Params {
    pub fn from(metainfo: &Metainfo, id: &String) -> Self {
        let length = metainfo.info.length.unwrap_or_default().to_string();
        let info_hash = percent_encode(&metainfo.info.sha1(), DEFAULT_ENCODE_SET).to_string();
        let mut params = HashMap::new();
        params.insert("left", length);
        params.insert("info_hash", info_hash);
        params.insert("downloaded", String::from("0"));
        params.insert("uploaded", String::from("0"));
        params.insert("event", String::from("started"));
        params.insert("peer_id", id.clone());
        params.insert("compact", String::from("1"));
        params.insert("port", String::from("6881"));
        return Params { params };
    }

    fn query(&self) -> String {
        let param_strings: Vec<String> = self.params
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect();
        param_strings.join("&")
    }
}
impl fmt::Display for Params {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(fmt, "{}", &self.query())
    }
}
