use std::collections::HashMap;

use crate::models::{Snapshot, FileEntry};
use super::models::DiffResult;

pub(super) fn calculate_diff(old_snapshot: &Snapshot, new_snapshot: &Snapshot) -> DiffResult {
    let old_map = old_snapshot
        .files
        .iter()
        .map(|FileEntry { hash, path }| (path.clone(), hash.clone()))
        .collect::<HashMap<String, String>>();
    let new_map = new_snapshot
        .files
        .iter()
        .map(|FileEntry { hash, path }| (path.clone(), hash.clone()))
        .collect::<HashMap<String, String>>();

    let mut result = DiffResult::default();

    for (path, hash) in new_map.iter() {
        if let Some(old_hash) = old_map.get(path) {
            if hash != old_hash {
                result.modified.push(path.clone());
            }
        } else {
            result.added.push(path.clone())
        }
    }

    for path in old_map.keys() {
        if !new_map.contains_key(path) {
            result.removed.push(path.clone())
        }
    }

    result.added.sort();
    result.modified.sort();
    result.removed.sort();

    result
}