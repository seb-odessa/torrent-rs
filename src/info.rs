use serde_bytes::ByteBuf;

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
