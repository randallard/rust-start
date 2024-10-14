use tracing::{debug,info};
use tracing_subscriber::EnvFilter;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    info!("rust-start started");
    debug!("debugging");
    info!("rust-start complete");
}
