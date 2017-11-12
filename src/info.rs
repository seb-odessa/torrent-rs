use serde_bytes::ByteBuf;

#[derive(Debug, Deserialize)]
pub struct File {
    pub path: Vec<String>,
    pub length: i64,
    #[serde(default)] pub md5sum: Option<String>,
}
#[derive(Debug, Deserialize)]
pub struct Info {
    pub name: String,
    pub pieces: ByteBuf,
    #[serde(rename = "piece length")] pub piece_length: i64,
    #[serde(default)] pub md5sum: Option<String>,
    #[serde(default)] pub length: Option<i64>,
    #[serde(default)] pub files: Option<Vec<File>>,
    #[serde(default)] pub private: Option<u8>,
    #[serde(default)] pub path: Option<Vec<String>>,
    #[serde(default)]
    #[serde(rename = "root hash")]
    pub root_hash: Option<String>,
}
