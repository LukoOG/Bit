use std::error::Error;

use crate::{config::load_config, models::Snapshot};

pub fn handle_history(snapshots: &[Snapshot]) -> Result<(), Box<dyn Error>> {
    let config = load_config()?;
    //Can never panic if snapshots is not empty
    let current_id = config.current_snapshot.unwrap();
    if snapshots.is_empty() {
        println!("No snapshots yet!");
        return Ok(());
    }

    for snapshot in snapshots {
        if snapshot.id == current_id {
            println!("{}. {} {}", snapshot.id, snapshot.message, '*');
            continue;
        }
        println!("{}. {}", snapshot.id, snapshot.message)
    }

    Ok(())
}
