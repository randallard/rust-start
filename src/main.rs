mod error;
mod fs;

pub use self::error::{Error, Result};

use crate::fs::list_files;

use tracing::{debug, info};
use tracing_subscriber::EnvFilter;


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