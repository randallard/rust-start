mod config;
mod error;

pub use self::error::{Error, Result};
pub use config::config;
use tracing::{info, error};
use tracing_subscriber::EnvFilter;
use informix_rust::Connection;
use std::env;

fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    info!("rust-start started");

    // Attempt database connection
    info!("Attempting to connect to Informix database");
    match connect_to_db() {
        Ok(_) => info!("Successfully connected to database"),
        Err(e) => error!("Failed to connect to database: {}", e)
    }

    info!("rust-start complete");

    Ok(())
}

fn connect_to_db() -> Result<()> {
    let conn = Connection::new()?;
    info!("Connection object created");

    let conn_string = &env::var("INFORMIXDB_CONN_PARAMS")
        .map_err(|_| Error::ConfigMissingEnv("INFORMIXDB_CONN_PARAMS"))?;
    
    info!("Connecting with connection string");
    conn.connect_with_string(conn_string)?;
    
    // Example query - modify according to your database schema
    let query = "SELECT FIRST 1 * FROM systables";
    let stmt = conn.prepare(query)?;
    
    stmt.execute()?;

    while let Some(row) = stmt.fetch()? {
        info!("Query result: {:?}", row);
    }

    Ok(())
}