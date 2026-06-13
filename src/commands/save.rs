use std::{fs, error::Error};
use crate::models::Snapshot;

pub fn handle_save(snapshots: &mut Vec<Snapshot>, message: String) -> Result<(), Box<dyn Error>>{
    let new_snapshot = Snapshot {
        id: snapshots.len() as u32 + 1,
        message,
        files: Vec::new(),
    };
    snapshots.push(new_snapshot);
    let json = serde_json::to_string_pretty(snapshots)?;
    fs::write(".snapr/snapshots.json", json)?;
    Ok(())
}