use anyhow::Result;
use std::path::PathBuf;

pub trait Disk: Sized {
    fn create(size: u64, name: String) -> Result<Self>;

    fn mount(&self) -> ();

    fn get_path(&self) -> PathBuf;
}
