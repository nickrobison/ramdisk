use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    path: String,
    threads: u8,
    sync_duration: u32,
}
