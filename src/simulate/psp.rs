use crate::types::config::{Key, Possible, PspSimulationConfig, Status};
use anyhow::Result;
use rand::Rng;
use std::collections::HashMap;

pub fn validate_parameters(
    config: &PspSimulationConfig,
    connector: String,
    params: HashMap<Key, Key>,
) -> Result<Status> {
    let mut rng = rand::thread_rng();
    let param = params
        .into_iter()
        .map(|(key, value)| (key, Possible::Value(value)))
        .collect::<HashMap<_, _>>();

    Ok(config
        .iter()
        .fold(None, |acc, (key, value)| {
            if connector != *key {
                return acc;
            }

            if acc.is_some() {
                acc
            } else if value.key == param {
                let status = rng.gen_bool(value.sr as f64 / 100.0);
                Some(if status {
                    Status::Success
                } else {
                    Status::Failure
                })
            } else {
                None
            }
        })
        .unwrap_or(config.otherwise))
}
