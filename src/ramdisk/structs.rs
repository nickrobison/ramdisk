use std::path::PathBuf;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct HdiUtilOutput<T> {
    pub system_entities: Vec<T>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct RdiskCreate {
    pub dev_entry: String,
    pub potentially_mountable: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct VolAttach {
    pub content_hint: String,
    pub dev_entry: String,
    pub potentially_mountable: bool,
    pub unmapped_content_hint: String,
    pub volume_kind: Option<String>,
    pub mount_point: Option<PathBuf>,
}
