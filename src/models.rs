use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FileEntry {
    pub hash: String,
    pub path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Snapshot {
    pub id: u32,
    pub message: String,
    pub files: Vec<FileEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SnaprConfig {
    version: u32
}