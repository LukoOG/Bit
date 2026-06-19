use crate::hash::{hash_file, hash_file_bytes};
use crate::models::FileEntry;
use crate::models::Snapshot;

use std::error::Error;
use std::fs;
use std::path::Path;

fn restore() -> Result<(), Box<dyn Error>> {
    // let contents = fs::read(&object_path).map_err(|_| format!("Missing object: {}", hash))?;

    // if let Some(parent) = Path::new(path).parent() {
    //     fs::create_dir_all(parent)?;
    // }

    // fs::write(path, contents)?
    Ok(())
}

pub fn handle_restore(snapshots: &[Snapshot], snapshot_id: u32) -> Result<(), Box<dyn Error>> {
    let snapshot = snapshots
        .iter()
        .find(|s| s.id == snapshot_id)
        .ok_or("Snapshot not found")?;

    for FileEntry { path, hash } in snapshot.files.iter() {
        let object_path = format!(".snapr/objects/{}", hash);
        println!("{:?} {:?}", &object_path, path);

        match fs::read(path).ok() {
            Some(current) => {
                let current_hash = hash_file_bytes(&current)?;
                if &current_hash == hash {
                    println!("Skipping file");
                    continue;
                } else {
                    todo!("Restore")
                }
            }

            None => todo!("Restore"),
        }
    }

    println!("Restored snapshot {}", snapshot_id);
    Ok(())
}
