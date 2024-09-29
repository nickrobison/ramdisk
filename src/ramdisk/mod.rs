use anyhow::Result;
use std::path::PathBuf;

use crate::traits::Disk;

mod imp;
mod structs;

#[derive(Debug)]
pub struct RamDisk {
    size: u64,
    name: String,
}

impl Disk for RamDisk {
    fn create(size: u64, name: String) -> Result<Self> {
        let s = RamDisk { size, name };

        let _ = s.initialize()?;

        Ok(s)
    }

    fn mount(&self) -> () {
        ()
    }

    fn get_path(&self) -> PathBuf {
        PathBuf::from("Hello")
    }
}
