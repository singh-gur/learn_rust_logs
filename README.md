# learn_rust_logs

A Rust project for learning log parsing and basic utility functions.

## Overview

This project demonstrates:
- Log file parsing and filtering by log type (INFO, WARNING, ERROR)
- Basic mathematical operations with error handling
- Rust module organization and error handling patterns

## Project Structure

```
learn_rust_logs/
├── Cargo.toml          # Project configuration
├── data/
│   └── logs.txt        # Sample log file for parsing
└── src/
    ├── main.rs         # Entry point
    └── log_handler/
        ├── mod.rs      # Module declaration
        └── utils.rs    # Utility functions
```

## Features

### Log Processing
- **extract_log_type**: Filters log entries by type (WARNING, ERROR, INFO)
- Reads from `data/logs.txt` containing timestamped log entries

### Utility Functions
- **divide**: Safe integer division with zero-division error handling

## Usage

Run the project:
```bash
cargo run
```

This will:
1. Extract and display all WARNING log entries from `data/logs.txt`
2. Demonstrate safe division operations (4/2 and 4/0)

## Sample Output

```
Warnings:
[
    "WARNING 14:32:10 Missing configuration defaults to 'standard mode'.",
    "WARNING 14:37:50 Unable to find user profile, using guest settings.",
    "WARNING 14:42:30 Low disk space on the backup drive.",
    "WARNING 14:47:12 Configuration file not found, using default configuration.",
    "WARNING 14:54:42 Failed to stop background services."
]
Division result: 2
Error dividing numbers: Division by zero is not allowed
```

## Functions

### `extract_log_type(logs: &String, log_type: &str) -> Vec<String>`
Located in `src/log_handler/utils.rs:3`

Filters log entries by the specified log type.

### `divide(a: i32, b: i32) -> Result<i32, Error>`
Located in `src/log_handler/utils.rs:13`

Performs safe integer division with error handling for division by zero.

## Dependencies

This project uses only Rust standard library features:
- `std::fs::read_to_string` for file reading
- `std::io::Error` for error handling

## Requirements

- Rust 2024 edition
- No external dependencies