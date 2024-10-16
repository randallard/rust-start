mod error;
mod my_module;

pub use self::error::{Error, Result};

use crate::my_module::my_function;

use tracing::{debug, error, info};
use tracing_subscriber::EnvFilter;


fn main() -> Result<()> {

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    info!("rust-start started");
    
    for num in 5..8 {
        debug!("Processing num: {}", num);        
        match my_function(num) {
            Ok(result) => {
                debug!("returned: {result}");
            },
            Err(e) => {
                error!("Error processing num '{}': {:?}", num, e);
            }
        }
    }

    info!("rust-start complete");

    Ok(())
}