use std::{error::Error, fs, path::Path};

use sha2::{Digest, Sha256};

//trying std and hex crate
// pub fn hash_file(path: &Path) -> Result<String, Box<dyn Error>> {
//     let mut hasher = Sha256::new();
//     let contents = fs::read(path)?;
//     hasher.update(contents);
//     let result = hasher.finalize();
//     Ok(result.iter().map(|byte| format!("{:02x}", byte)).collect())
// }

pub fn hash_file(path: &Path) -> Result<String, Box<dyn Error>> {
    let mut hasher = Sha256::new();
    let contents = fs::read(path)?;
    hasher.update(contents);
    let result = hasher.finalize();
    Ok(hex::encode(result))
}
