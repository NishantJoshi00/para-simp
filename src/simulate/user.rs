use crate::types::config::{Key, Parameters, SimulationConfig, UserSimulationConfig};
use anyhow::Result;
use rand::Rng;
use std::collections::HashMap;

pub fn generate_sample(config: &UserSimulationConfig) -> Result<HashMap<&Key, &Key>> {
    list_parameters(&config.parameters)
}

fn list_parameters(config: &SimulationConfig) -> Result<HashMap<&Key, &Key>> {
    config
        .iter()
        .try_fold(HashMap::new(), |mut acc, (key, param)| {
            let (value, next) = choose_parameter(param)?;
            acc.insert(key, value);
            if let Some(next) = next {
                let next = list_parameters(next)?;
                acc.extend(next);
            }

            Ok(acc)
        })
}

fn choose_parameter(param: &Parameters) -> Result<(&Key, Option<&SimulationConfig>)> {
    let mut rng = rand::thread_rng();
    let mut number = rng.gen_range(0..100);
    let varients = param.iter().fold(None, |acc, (key, info)| {
        if acc.is_some() {
            acc
        } else {
            match info {
                crate::types::config::ParameterConfig::Percentage(val) => {
                    if number < *val {
                        Some((key, None))
                    } else {
                        number -= val;
                        None
                    }
                }
                crate::types::config::ParameterConfig::Composite { percentage, next } => {
                    if number < *percentage {
                        Some((key, Some(next)))
                    } else {
                        number -= percentage;
                        None
                    }
                }
            }
        }
    });
    let output = varients.ok_or_else(|| anyhow::anyhow!("No parameter found"))?;
    Ok(output)
}

#[cfg(test)]
mod tests {
    use crate::types::config::UserSimulationConfig;

    #[test]
    fn test_generate_sample() {
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

        let config: UserSimulationConfig = serde_json::from_value(json).unwrap();

        let sample = super::generate_sample(&config.parameters).unwrap();
        panic!("{:#?}", sample);
    }
}
