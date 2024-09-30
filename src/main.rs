use anyhow::Result;
use clap::Parser;
use copy::CopyHandler;
use monitor::PathWatcher;
use notify::EventKind;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::thread;
use tokio::task::yield_now;
use traits::{Disk, FileMonitor};

mod config;
mod copy;
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

    let mut fm = PathWatcher::create(p.clone()).expect("Created watcher");
    let mut recv = fm.watch();

    let ch = &Arc::new(CopyHandler::new(p, "/Volumes/Hello".into()));

    while let Some(event) = recv.recv().await {
        let ch = Arc::clone(ch);
        tokio::spawn(async move {
            if (event.kind.is_create()) {
                let pp = event.paths.first().expect("Should have path");
                println!("Events: {:?}", event);
                let _ = ch.copy(pp).await;
            }
        });
    }

    Ok(())
}
