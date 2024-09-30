use clap::Parser;
use monitor::PathWatcher;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::thread;
use traits::{Disk, FileMonitor};
use anyhow::Result;

mod config;
mod monitor;
mod ramdisk;
mod traits;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "./config.toml")]
    config: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::builder()
        .filter(None, log::LevelFilter::Debug)
        .init();
    let _args = Args::parse();

    // Create the temp disk
    let d = ramdisk::RamDisk::create(1, "Hello".to_string());
    println!("Created: {:?}", d);

    let p = PathBuf::from("/Users/nickrobison/Desktop/");

    let mut fm = PathWatcher::create(p).expect("Created watcher");
    let mut recv = fm.watch();

    while let Some(event) = recv.recv().await {
       println!("Event: {:?}", event);
    };

    Ok(())




}
