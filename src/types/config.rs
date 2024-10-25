use anyhow::{ensure, Context, Result};
use core::ops::Deref;
use std::collections::HashMap;
use std::env;
use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub user: UserSimulationConfig,
    pub psp: PspSimulationConfig,
}

impl Config {
    pub fn load() -> Result<Self> {
        // First try environment variable
        if let Ok(config_path) = env::var("CONFIG_FILE") {
            return Self::load_from_path(&config_path);
        }

        // Then try current directory
        let default_path = Path::new("config.json");
        if default_path.exists() {
            let output = Self::load_from_path(default_path)?;
            output.user.validate()?;
            return Ok(output);
        }

        anyhow::bail!("No config file found. Please provide it either in ./config.json or set `CONFIG_FILE` environment variable")
    }

    fn load_from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        let config_str = std::fs::read_to_string(path.as_ref())
            .with_context(|| format!("Failed to read config file: {:?}", path.as_ref()))?;

        serde_json::from_str(&config_str).with_context(|| "Failed to parse config file")
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Success,
    Failure,
}

#[derive(Debug, Deserialize, Serialize, Hash, Eq, PartialEq, Clone)]
#[serde(transparent)]
pub struct Key(pub String);

#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct SimulationConfig(HashMap<Key, Parameters>);

#[derive(Debug, Deserialize, Serialize)]
pub struct UserSimulationConfig {
    #[serde(flatten)]
    pub parameters: SimulationConfig,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Parameters(HashMap<Key, ParameterConfig>);

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ParameterConfig {
    Percentage(u8),
    Composite {
        percentage: u8,
        next: SimulationConfig,
    },
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PspSimulationConfig {
    pub config: HashMap<String, PspVariant>,
    pub otherwise: Status,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PspVariant {
    pub key: HashMap<Key, Possible>,
    pub sr: u8,
}

#[derive(Debug, Serialize)]
pub enum Possible {
    Value(Key),
    Any,
}

impl PartialEq for Possible {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Possible::Any, _) => true,
            (_, Possible::Any) => true,
            (Possible::Value(a), Possible::Value(b)) => a == b,
        }
    }
}

impl<'de> serde::Deserialize<'de> for Possible {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        if value == "*" {
            Ok(Possible::Any)
        } else {
            Ok(Possible::Value(Key(value)))
        }
    }
}

impl Parameters {
    pub fn validate(&self) -> Result<()> {
        let mut total = 0;
        for (_key, value) in self.0.iter() {
            match value {
                ParameterConfig::Percentage(value) => total += value,
                ParameterConfig::Composite { percentage, next } => {
                    total += percentage;
                    next.validate()?;
                }
            }
        }
        ensure!(total == 100, "Total percentage must be 100");
        Ok(())
    }
}

impl SimulationConfig {
    pub fn validate(&self) -> Result<()> {
        self.0.iter().try_for_each(|(key, value)| {
            value
                .validate()
                .context(format!("validation failed for: {}", key.0))?;
            Ok(())
        })
    }
}

impl UserSimulationConfig {
    pub fn validate(&self) -> Result<()> {
        self.parameters.validate()
    }
}

impl Deref for SimulationConfig {
    type Target = HashMap<Key, Parameters>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for Parameters {
    type Target = HashMap<Key, ParameterConfig>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for PspSimulationConfig {
    type Target = HashMap<String, PspVariant>;

    fn deref(&self) -> &Self::Target {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_deserialize_json() {
        let json = serde_json::json!({
            "payment_method": {
                "card": {
                    "percentage": 50,
                    "next": {
                        "payment_method_type": {
                            "credit": 50,
                            "debit": 50
                        }
                    }
                },
                "bnpl": 30,
                "wallet": 20
            }
        });

        let config: super::UserSimulationConfig = serde_json::from_value(json).unwrap();

        assert!(
            config
                .parameters
                .0
                .keys()
                .next()
                .expect("No keys found")
                .0
                .as_str()
                == "payment_method",
            "Key not found"
        );

        assert!(
            config
                .parameters
                .0
                .get(&super::Key("payment_method".to_string()))
                .expect("No key found")
                .0
                .keys()
                .map(|k| k.0.clone())
                .collect::<Vec<_>>()
                .len()
                == 3,
            "Parameter keys not found"
        );
    }

    #[test]
    fn test_validate() -> anyhow::Result<()> {
        let json = serde_json::json!({
            "payment_method": {
                "card": {
                    "percentage": 50,
                    "next": {
                        "payment_method_type": {
                            "credit": 50,
                            "debit": 50
                        }
                    }
                },
                "bnpl": 30,
                "wallet": 20
            }
        });

        let config: super::UserSimulationConfig = serde_json::from_value(json).unwrap();
        config.parameters.validate()
    }
}
