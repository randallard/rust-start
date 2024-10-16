mod error;
mod fs;

pub use self::error::{Error, Result};

use crate::fs::list_files;

use tracing::{debug, error, info};
use tracing_subscriber::EnvFilter;


fn main() -> Result<()> {

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    info!("rust-start started");
    
    let paths = vec!["./some_dir", "./some_dir/empty_dir", "./some_dir/another_dir", "./non_existant"];
    
    for path in paths {
        debug!("Listing files for path: {}", path);        
        match list_files(path) {
            Ok(files) => {
                debug!("{files:#?}");
            },
            Err(e) => {
                error!("Error processing path '{}': {:?}", path, e);
            }
        }
    }

    info!("rust-start complete");

    Ok(())
}