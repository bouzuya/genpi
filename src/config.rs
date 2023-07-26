use std::env;

use anyhow::Context;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Config {
    pub base_path: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        let base_path = get_var_opt("BASE_PATH")?.unwrap_or_default();
        let port = get_var_opt("PORT")
            .map(|s| s.unwrap_or("3000".to_owned()))
            .and_then(|s| {
                s.as_str()
                    .parse::<u16>()
                    .context("PORT range is (0..=65535)")
            })?;
        Ok(Self { base_path, port })
    }
}

fn get_var_opt(name: &str) -> anyhow::Result<Option<String>> {
    use std::env::VarError::*;
    match env::var(name) {
        Ok(s) => Ok(Some(s)),
        Err(e) => match e {
            NotPresent => Ok(None),
            NotUnicode(_) => Err(anyhow::anyhow!("{name} is not unicode")),
        },
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, ffi::OsStr, os::unix::prelude::OsStrExt};

    use super::*;

    struct EnvVars(HashMap<&'static str, Option<&'static OsStr>>);

    impl EnvVars {
        fn into_kvs(self) -> Vec<(&'static str, Option<&'static OsStr>)> {
            self.0
                .into_iter()
                .collect::<Vec<(&'static str, Option<&'static OsStr>)>>()
        }

        fn set(mut self, name: &'static str, value: &'static [u8]) -> Self {
            self.0.insert(name, Some(OsStr::from_bytes(value)));
            self
        }

        fn unset(mut self, name: &'static str) -> Self {
            self.0.insert(name, None);
            self
        }
    }

    impl Default for EnvVars {
        fn default() -> Self {
            let mut env_vars = HashMap::new();
            env_vars.insert("BASE_PATH", Some(OsStr::new("/lab/genpi")));
            env_vars.insert("PORT", Some(OsStr::new("3000")));
            Self(env_vars)
        }
    }

    #[test]
    fn test_empty() -> anyhow::Result<()> {
        temp_env::with_vars(
            EnvVars::default()
                .unset("BASE_PATH")
                .unset("PORT")
                .into_kvs(),
            || {
                let config = Config::from_env()?;
                assert_eq!(
                    config,
                    Config {
                        base_path: "".to_owned(),
                        port: 3000
                    }
                );
                Ok(())
            },
        )
    }

    #[test]
    fn test_all() -> anyhow::Result<()> {
        temp_env::with_vars(
            EnvVars::default()
                .set("BASE_PATH", b"/lab/genpi")
                .set("PORT", b"3000")
                .into_kvs(),
            || {
                let config = Config::from_env()?;
                assert_eq!(
                    config,
                    Config {
                        base_path: "/lab/genpi".to_owned(),
                        port: 3000
                    }
                );
                Ok(())
            },
        )
    }

    #[test]
    fn test_base_path_is_not_unicode() -> anyhow::Result<()> {
        temp_env::with_vars(
            EnvVars::default()
                .set("BASE_PATH", &[0x66, 0x6f, 0x80, 0x6f])
                .into_kvs(),
            || {
                assert_eq!(
                    Config::from_env().unwrap_err().to_string(),
                    "BASE_PATH is not unicode"
                );
                Ok(())
            },
        )
    }

    #[test]
    fn test_port_is_not_unicode() -> anyhow::Result<()> {
        temp_env::with_vars(
            EnvVars::default()
                .set("PORT", &[0x66, 0x6f, 0x80, 0x6f])
                .into_kvs(),
            || {
                assert_eq!(
                    Config::from_env().unwrap_err().to_string(),
                    "PORT is not unicode"
                );
                Ok(())
            },
        )
    }

    #[test]
    fn test_port_is_not_number() -> anyhow::Result<()> {
        temp_env::with_vars(EnvVars::default().set("PORT", b"a").into_kvs(), || {
            assert_eq!(
                Config::from_env().unwrap_err().to_string(),
                "PORT range is (0..=65535)"
            );
            Ok(())
        })
    }
}
