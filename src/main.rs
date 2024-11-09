mod config;
mod error;

// #[cfg(test)] // Commented during early development.
pub mod _dev_utils;

pub use self::error::{Error, Result};
pub use config::config;
use tracing::{info, error};
use tracing_subscriber::EnvFilter;
use informix_rust::Connection;

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
    info!("Creating database connection object");
    let conn = Connection::new()?;
    info!("Connection object created successfully");

    let cfg = config();
    let conn_string = cfg.get_connection_string();
    
    info!("Attempting connection with string: {}", conn_string);
    conn.connect_with_string(&conn_string)?;
    info!("Successfully connected to database");
    
    // Test query - counts tables in systables
    let query = "SELECT COUNT(*) as table_count FROM systables";
    let stmt = conn.prepare(query)?;
    info!("Query prepared successfully");
    
    stmt.execute()?;
    info!("Query executed successfully");

    if let Some(row) = stmt.fetch()? {
        info!("Database contains {} tables", row[0]);
    }

    info!("Database connection test completed successfully");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_connection() -> Result<()> {
        let _ = tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .try_init();
        
        info!("Starting database connection test");
        connect_to_db()?;
        info!("Database connection test completed");
        Ok(())
    }
}