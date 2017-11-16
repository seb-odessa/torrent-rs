use serde_bytes::ByteBuf;
use serde_bencode::ser;
use hash::sha1;

#[derive(Debug, Deserialize, Serialize)]
pub struct File {
    pub length: i64,
    pub path: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Info {
    #[serde(rename = "piece length")]
    pub piece_length: i64,
    pub pieces: ByteBuf,
    pub name: String,
    pub length: Option<i64>,
    pub files: Option<Vec<File>>,
}
impl Info {
    pub fn sha1(&self) -> Vec<u8> {
        sha1(&ser::to_bytes::<Info>(&self).unwrap_or_default())
    }
}