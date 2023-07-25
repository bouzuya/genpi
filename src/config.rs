use std::{env, ffi::OsString};

use anyhow::Context;

#[derive(Clone, Debug)]
pub struct Config {
    pub port: u16,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        let port = get_var_opt("PORT")
            .map(|s| s.unwrap_or("3000".to_owned()))
            .map_err(|_| anyhow::anyhow!("PORT is not unicode"))
            .and_then(|s| {
                s.as_str()
                    .parse::<u16>()
                    .context("PORT range is (0..=65535)")
            })?;
        Ok(Self { port })
    }
}

fn get_var_opt(name: &str) -> Result<Option<String>, OsString> {
    use std::env::VarError::*;
    match env::var(name) {
        Ok(s) => Ok(Some(s)),
        Err(e) => match e {
            NotPresent => Ok(None),
            NotUnicode(e) => Err(e),
        },
    }
}
