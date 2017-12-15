use std::fmt;
use std::collections::HashMap;

use metainfo::Metainfo;

#[derive(Debug)]
pub struct Params {
    params: HashMap<&'static str, String>,
}
impl Params {
    fn escape(buffer: &Vec<u8>) -> String {
        let mut result = String::with_capacity(3 * buffer.len());
        for byte in buffer {
            result += "%";
            result += &format!("{:02X}", byte);
        }
        result
    }

    pub fn from(metainfo: &Metainfo, id: &String) -> Self {
        let length = metainfo.info.length.unwrap_or_default().to_string();
        let mut params = HashMap::new();
        params.insert("left", length);
        params.insert("info_hash", Self::escape(&metainfo.info_hash()));
        params.insert("downloaded", String::from("0"));
        params.insert("uploaded", String::from("0"));
        params.insert("event", String::from("started"));
        params.insert("peer_id", id.clone());
        params.insert("compact", String::from("1"));
        params.insert("port", String::from("6886"));
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
