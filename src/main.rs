use clap::Parser;
use monitor::PathWatcher;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::thread;

mod config;
mod monitor;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "./config.toml")]
    config: String,
}

fn main() {
    let args = Args::parse();

    let (sender, receiver) = async_channel::unbounded();
    let r1 = receiver.clone();
    println!("Is closed? {}", r1.is_closed());
    let p = PathBuf::from("/Users/nickrobison/Desktop/");
    let w = Arc::new(PathWatcher::new(p, sender));
    let h = thread::spawn(move || {
        let _ = w.start();
    });

    let h2 = thread::spawn(move || loop {
        let m = r1.recv_blocking();
        println!("Reeived msg: {:?}", m);
    });

    h.join().unwrap();
    println!("H done");
    h2.join().unwrap();
}
