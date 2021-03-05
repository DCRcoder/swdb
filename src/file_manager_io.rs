use std::fs::{OpenOptions, File};
use std::path::{Path, PathBuf};
use crate::error::SWDBError;
pub struct IOManager {
    f: File,
}

impl IOManager {
    pub fn new<P: AsRef<Path>>(path: P, cap: u64) -> Result<IOManager, SWDBError> {
        let f = OpenOptions::new().create(true).write(true).read(true).open(path)?;
        f.set_len(cap)?;
        return Ok(IOManager{f: f});
    }
}