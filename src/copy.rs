use anyhow::Result;
use log::info;
use std::path::PathBuf;
use tokio::fs;

pub struct CopyHandler {
    source_path: PathBuf,
    base_path: PathBuf,
}

impl CopyHandler {
    pub fn new(source: PathBuf, dest: PathBuf) -> Self {
        Self {
            source_path: source,
            base_path: dest,
        }
    }

    pub async fn copy(&self, file: &PathBuf) -> Result<()> {
        // TODO: Do I really need clone here?
        let f = file.strip_prefix(self.source_path.clone())?;
        let dest = &self.base_path.join(f);
        info!(
            "Received file {}, stripped: {}, to {}",
            file.display(),
            f.display(),
            dest.display()
        );
        let _ = fs::copy(file, dest).await?;

        Ok(())
    }
}
