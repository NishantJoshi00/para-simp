use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use simp::types::config::Config;
use std::io::{self, Read};

#[derive(Parser)]
#[command(name = "sample_cli")]
#[command(about = "A CLI tool for generating and resolving samples", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a new sample
    GenerateSample,
    /// Resolve an existing sample
    ResolveSample {
        /// The connector to use
        #[arg(value_name = "CONNECTOR")]
        connector: String,
    },
}

fn read_stdin() -> Result<String> {
    let mut buffer = String::new();
    io::stdin()
        .read_to_string(&mut buffer)
        .context("Failed to read from stdin")?;
    Ok(buffer)
}

fn generate_sample(config: &Config) -> Result<()> {
    let output = simp::simulate::user::generate_sample(&config.user)?;
    let output = serde_json::to_string_pretty(&output)?;
    println!("{}", output);
    Ok(())
}

fn resolve_sample(config: &Config, connector: String) -> Result<()> {
    let params = read_stdin()?;
    let params = serde_json::from_str(&params)?;
    let output = simp::simulate::psp::validate_parameters(&config.psp, connector, params)?;
    let output = serde_json::to_string_pretty(&output)?;
    println!("{}", output);
    Ok(())
}

fn main() -> Result<()> {
    // Load configuration
    let config = Config::load()?;

    // Parse command line arguments
    let cli = Cli::parse();

    // Execute the appropriate command
    match cli.command {
        Commands::GenerateSample => generate_sample(&config)?,
        Commands::ResolveSample { connector } => resolve_sample(&config, connector)?,
    }

    Ok(())
}
