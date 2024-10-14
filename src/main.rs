use std::path::Path;
use tracing::{debug, info, warn};
use tracing_subscriber::EnvFilter;

pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>; // For early dev.

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    info!("rust-start started");
    
    let paths = vec!["./some_dir", "./some_dir/another_dir", "./non_existant"];
    
    for path in paths {
        debug!("Listing files for path: {}", path);
        let path = Path::new(path);
        
        match std::fs::read_dir(path) {
            Ok(entries) => {
                let files: Vec<String> = entries
                    .filter_map(|re| re.ok())
                    .filter(|e| e.file_type().map(|ft| ft.is_file()).unwrap_or(false))
                    .filter_map(|e| e.file_name().into_string().ok())
                    .collect();
                debug!("Files found: {files:#?}");
            },
            Err(e) => {
                warn!("Failed to read directory '{}': {}", path.display(), e);
            }
        }
    }

    info!("rust-start complete");
    Ok(())
}