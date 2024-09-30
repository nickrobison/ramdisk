use anyhow::Result;
use notify::Event;
use std::path::PathBuf;
use tokio::sync::mpsc;

pub trait Disk: Sized {
    fn create(size: u64, name: String) -> Result<Self>;

    fn mount(&self) -> ();

    fn get_path(&self) -> PathBuf;
}

pub trait FileMonitor: Sized {
    fn create(path: PathBuf) -> Result<Self>;

    fn watch(&mut self) -> mpsc::UnboundedReceiver<Event>;
}
