use async_channel::Sender;
use notify::{Config, RecommendedWatcher, Watcher};
use std::path::PathBuf;

pub struct PathWatcher {
    directory: PathBuf,
    channel: Sender<String>,
}

impl PathWatcher {
    pub fn new(directory: PathBuf, channel: Sender<String>) -> Self {
        Self { directory, channel }
    }

    pub fn start(&self) -> notify::Result<()> {
        println!("Starting watch of path: {:?}", self.directory);
        let c = self.channel.clone();
        let _ = c.send_blocking("Start me".to_string());
        let mut watcher = RecommendedWatcher::new(
            move |res| {
                println!("Received: {:?}", res);
                let _ = c.send_blocking("Hello2".to_string());
            },
            Config::default(),
        )?;

        watcher.watch(self.directory.as_path(), notify::RecursiveMode::Recursive)?;

        loop {}
    }
}
