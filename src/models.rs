use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FileEntry {
    hash: String,
    path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Snapshot {
    id: u32,
    message: String,
    files: Vec<FileEntry>,
}