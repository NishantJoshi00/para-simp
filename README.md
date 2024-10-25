# SIMP (Simulate Parametric Data)

A simulation tool for generating and validating parametric data based on configurable probability distributions. This tool provides both a CLI interface and a REST API server for generating samples and validating them against predefined rules.

## Features

- Generate random samples based on configured probability distributions
- Validate parameters against rule configurations
- Available as both CLI tool and HTTP server
- Configurable success rates for different validation rules
- Support for nested parameter configurations

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
    "category_a": {
      "option_1": {
        "percentage": 50,
        "next": {
          "subcategory": {
            "type_x": 50,
            "type_y": 50
          }
        }
      },
      "option_2": 30,
      "option_3": 20
    }
  },
  "psp": {
    "config": {
      "validator_1": {
        "key": {
          "category_a": "option_1",
          "subcategory": "*"
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

Generate a sample:
```bash
simp generate-sample
```

Validate a sample against rules:
```bash
echo '{"category_a": "option_1", "subcategory": "type_x"}' | simp resolve-sample validator_1
```

### HTTP Server

Start the server:
```bash
export PORT=8080  # Optional, defaults to 8080
export HOST=0.0.0.0  # Optional, defaults to localhost
simp-server
```

Available endpoints:

- `GET /generate` - Generate a new parameter sample
- `POST /resolve/:validator` - Validate parameters against specific rules
  - Request body should contain parameters as JSON
  - `:validator` is the validator identifier

Example HTTP requests:

```bash
# Generate a sample
curl http://localhost:8080/generate

# Validate a sample
curl -X POST \
  -H "Content-Type: application/json" \
  -d '{"category_a": "option_1", "subcategory": "type_x"}' \
  http://localhost:8080/resolve/validator_1
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
- Invalid parameters
- Server binding failures

## Environment Variables

- `CONFIG_FILE` - Path to configuration file
- `PORT` - Server port (default: 8080)
- `HOST` - Server host (default: localhost)
