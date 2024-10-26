use crate::{Error, Result};
use std::{env, sync::OnceLock};

pub fn config() -> &'static Config {
    static INSTANCE: OnceLock<Config> = OnceLock::new();
    INSTANCE.get_or_init(|| {
        Config::load_from_env().unwrap_or_else(|ex| {
            panic!("FATAL - WHILE LOADING CONF - Cause: {ex:?}")
        })
    })
}

#[allow(non_snake_case)]
pub struct Config {
    // Existing config vars
    pub CONFIG_VAR_ONE: String,
    pub CONFIG_VAR_TWO: String,
    
    // Database config
    pub DB_SERVER: String,
    pub DB_NAME: String,
    pub DB_HOST: String,
    pub DB_PORT: String,
    pub DB_USER: String,
    pub DB_PASSWORD: String,
}

impl Config {
    fn load_from_env() -> Result<Config> {
        Ok(Config {
            CONFIG_VAR_ONE: get_env("SERVICE_CONFIG_VAR_ONE")?,
            CONFIG_VAR_TWO: get_env("SERVICE_CONFIG_VAR_TWO")?,
            
            // Database settings
            DB_SERVER: get_env("INFORMIXSERVER")?,
            DB_NAME: get_env("DB_NAME")?,
            DB_HOST: get_env("DB_HOST")?,
            DB_PORT: get_env("DB_PORT")?,
            DB_USER: get_env("DB_USER")?,
            DB_PASSWORD: get_env("DB_PASSWORD")?,
        })
    }

    pub fn get_connection_string(&self) -> String {
        format!(
            "SERVER={};DATABASE={};HOST={};SERVICE={};UID={};PWD={}",
            self.DB_SERVER,
            self.DB_NAME,
            self.DB_HOST,
            self.DB_PORT,
            self.DB_USER,
            self.DB_PASSWORD
        )
    }
}

fn get_env(name: &'static str) -> Result<String> {
    env::var(name).map_err(|_| Error::ConfigMissingEnv(name))
}