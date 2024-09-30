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

    pub async fn remove(&self, file: &PathBuf) -> Result<()> {
        // TODO: Do I really need clone here?
        let f = file.strip_prefix(self.source_path.clone())?;
        let dest = &self.base_path.join(f);
        let _ = fs::remove_file(dest).await?;
        Ok(())
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

    pub async fn rename(&self, from: &PathBuf, to: &PathBuf) -> Result<()> {
        let old = from.strip_prefix(self.source_path.clone())?;
        let nw = to.strip_prefix(self.base_path.clone())?;
        let old_path = &self.base_path.join(old);
        let new_path = &self.base_path.join(nw);
        info!("Renaming {} to {}", old.display(), nw.display());
        let _ = fs::rename(old_path, new_path).await?;
        Ok(())
    }
}
