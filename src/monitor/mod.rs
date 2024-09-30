use anyhow::Result;
use notify::{Config, Event, FsEventWatcher, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::PathBuf;
use tokio::sync::mpsc;

use crate::traits::FileMonitor;

pub struct PathWatcher {
    directory: PathBuf,
    watcher: Option<FsEventWatcher>,
}

impl FileMonitor for PathWatcher {
    fn create(path: PathBuf) -> Result<Self> {
        Ok(Self {
            directory: path,
            watcher: None,
        })
    }

    fn watch(&mut self) -> mpsc::UnboundedReceiver<Event> {
        let (send, recv) = mpsc::unbounded_channel();

        let mut watcher = RecommendedWatcher::new(
            move |res| match res {
                Ok(e) => {
                    let _ = send.send(e);
                    {}
                }
                Err(_) => {}
            },
            Config::default(),
        )
        .expect("Should have created watcher");
        let _ = watcher.watch(&self.directory, RecursiveMode::Recursive).expect("Should be watching");

        self.watcher = Some(watcher);

        recv
    }
}
