# SIMP (Sample Integration Mock Platform)

A simulation tool for generating and validating payment integration samples. This tool provides both a CLI interface and a REST API server for generating realistic payment method samples and validating them against PSP (Payment Service Provider) configurations.

## Features

- Generate random payment method samples based on configured probabilities
- Validate payment parameters against PSP configurations
- Available as both CLI tool and HTTP server
- Configurable success rates for different PSPs
- Support for nested payment method configurations

## Installation

```bash
cargo install simp
```

## Configuration

The tool requires a configuration file in JSON format. You can specify the config file location in two ways:

1. Place it in the current directory as `config.json`
2. Set the `CONFIG_FILE` environment variable with the path to your config file

### Configuration Format

```json
{
  "user": {
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
  },
  "psp": {
    "config": {
      "stripe": {
        "key": {
          "payment_method": "card",
          "payment_method_type": "*"
        },
        "sr": 95
      }
    },
    "otherwise": "failure"
  }
}
```

## Usage

### CLI Interface

Generate a sample payment method:

```bash
simp generate-sample
```

Validate a sample against a PSP:

```bash
echo '{"payment_method": "card", "payment_method_type": "credit"}' | simp resolve-sample stripe
```

### HTTP Server

Start the server:

```bash
export PORT=8080  # Optional, defaults to 8080
export HOST=0.0.0.0  # Optional, defaults to localhost
simp-server
```

Available endpoints:

- `GET /generate` - Generate a new sample payment method
- `POST /resolve/:connector` - Validate payment parameters against a specific PSP
  - Request body should contain payment parameters as JSON
  - `:connector` is the PSP identifier (e.g., "stripe")

Example HTTP requests:

```bash
# Generate a sample
curl http://localhost:8080/generate

# Resolve a sample
curl -X POST \
  -H "Content-Type: application/json" \
  -d '{"payment_method": "card", "payment_method_type": "credit"}' \
  http://localhost:8080/resolve/stripe
```

## Development

### Project Structure

- `src/types/` - Core data structures and configuration types
- `src/simulate/` - Implementation of sample generation and validation logic
- `src/bin/` - CLI and server binaries

### Building from Source

```bash
cargo build --release
```

### Running Tests

```bash
cargo test
```

## Error Handling

The project uses `anyhow` for error handling and provides descriptive error messages for common issues:

- Configuration file not found
- Invalid configuration format
- Invalid payment parameters
- Server binding failures

## Environment Variables

- `CONFIG_FILE` - Path to configuration file
- `PORT` - Server port (default: 8080)
- `HOST` - Server host (default: localhost)
