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
    pub CONFIG_VAR_ONE: String,
    pub CONFIG_VAR_TWO: String,
}

impl Config {
    fn load_from_env() -> Result<Config> {
        Ok(Config {
            CONFIG_VAR_ONE: get_env("SERVICE_CONFIG_VAR_ONE")?,
            CONFIG_VAR_TWO: get_env("SERVICE_CONFIG_VAR_TWO")?,
        })
    }
}

fn get_env(name: &'static str) -> Result<String> {
    env::var(name).map_err(|_| Error::ConfigMissingEnv(name))
}