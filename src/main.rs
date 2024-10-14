mod config;

use std::path::Path;
use tracing::{debug, info};
use tracing_subscriber::EnvFilter;

pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>; // For early dev.

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    info!("rust-start started");
    
    let paths = vec!["./some_dir", "./some_dir/empty_dir", "./some_dir/another_dir", "./non_existant"];
    
    for path in paths {
        debug!("Listing files for path: {}", path);
        let files = list_files(path);
        debug!("{files:#?}");
    }

    info!("rust-start complete");
    Ok(())
}

fn list_files(path: &str) -> Result<Vec<String>> {
    let path = Path::new(path);

    let files: Vec<String> = std::fs::read_dir(path)?
        .filter_map(|re| re.ok())
        .filter(|e| e.file_type().map(|ft| ft.is_file()).unwrap_or(false))
        .filter_map(|e| e.file_name().into_string().ok())
        .collect();
    if files.is_empty() {
        return Err("Return error manually".into())
    }

    Ok(files)
}