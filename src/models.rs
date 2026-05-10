pub struct FileObject {
    hash: String,
    path: String,
}

pub struct Snapshot {
    id: u32,
    message: String,
    files: Vec<FileObject>,
}