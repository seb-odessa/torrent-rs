//use std::fmt;
use serde_bencode::{de, Error};
use std::ffi::CString;

#[derive(Debug)]
pub struct Response {
    pub interval: i64,
    pub tracker_id: String,
    pub complete: i64,
    pub incomplete: i64,
    pub peers: Vec<String>,
}

impl Response {
    //fn try_load_binary
    pub fn from(buffer: &[u8]) -> Result<Self, Error> {



        // let response =
        //     de::from_bytes::<ResponseBinary>(&buffer).or(de::from_bytes::<ResponseLong>(&buffer));
        Ok(Response {
            interval: 0,
            tracker_id: "".to_owned(),
            complete: 0,
            incomplete: 0,
            peers: Vec::new(),
        })
    }
}


trait ResponseAccessor {
    fn failure_reason(&self) -> Option<String>;
    fn warning_message(&self) -> Option<String>;
    fn interval(&self) -> i64;
    fn minimal_interval(&self) -> i64;
    fn tracker_id(&self) -> String;
    fn complete(&self) -> i64;
    fn incomplete(&self) -> i64;
    fn peers(&self) -> Vec<String>;
}


#[derive(Debug, Deserialize, Serialize)]
struct Peers {
    #[serde(rename = "peer id")]
    pub peer_id: String,
    pub ip: String,
    pub port: i64,
}

#[derive(Debug, Deserialize)]
struct ResponseLong {
    #[serde(default)]
    #[serde(rename = "failure reason")]
    pub failure_reason: Option<String>,

    #[serde(default)]
    #[serde(rename = "warning message")]
    pub warning_message: Option<String>,

    pub interval: i64,
    #[serde(default)]
    #[serde(rename = "min interval")]
    pub min_interval: Option<i64>,

    #[serde(default)]
    #[serde(rename = "tracker id")]
    pub tracker_id: Option<String>,
    pub complete: i64,
    pub incomplete: i64,

    #[serde(default)]
    pub peers: Option<Vec<Peers>>,
}
impl ResponseAccessor for ResponseLong {
    fn failure_reason(&self) -> Option<String> {
        self.failure_reason.clone()
    }
    fn warning_message(&self) -> Option<String> {
        self.warning_message.clone()
    }

    fn interval(&self) -> i64 {
        0
    }

    fn minimal_interval(&self) -> i64 {
        0
    }
    fn tracker_id(&self) -> String {
        "".to_owned()
    }
    fn complete(&self) -> i64 {
        0
    }
    fn incomplete(&self) -> i64 {
        0
    }
    fn peers(&self) -> Vec<String> {
        Vec::new()
    }
}


#[derive(Debug, Deserialize)]
struct ResponseBinary {
    #[serde(default)]
    #[serde(rename = "failure reason")]
    pub failure_reason: Option<String>,

    #[serde(default)]
    #[serde(rename = "warning message")]
    pub warning_message: Option<String>,

    pub interval: i64,
    #[serde(default)]
    #[serde(rename = "min interval")]
    pub min_interval: Option<i64>,

    #[serde(default)]
    #[serde(rename = "tracker id")]
    pub tracker_id: Option<String>,
    pub complete: i64,
    pub incomplete: i64,

    #[serde(default)]
    pub peers: Option<CString>,
}
