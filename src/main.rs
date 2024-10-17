mod config;
mod error;

pub use self::error::{Error, Result};
pub use config::config;

use tracing::{debug,info};
use tracing_subscriber::EnvFilter;


fn main() -> Result<()> {

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    info!("rust-start started");

    debug!("got config CONFIG_VAR_ONE: {}", &config().CONFIG_VAR_ONE);
    debug!("got config CONFIG_VAR_TWO: {}", &config().CONFIG_VAR_TWO);

    info!("rust-start complete");

    Ok(())
}