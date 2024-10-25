mod config;
mod error;

pub use self::error::{Error, Result};
pub use config::config;
use tracing::info;
use tracing_subscriber::EnvFilter;


fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    info!("rust-start started");  

    info!("rust-start complete");

    Ok(())
}