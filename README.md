# Evan_Li_IDS706_Hwk8 - Rust Command-Line Tool

[![CI](https://github.com/bionicotaku/Evan_Li_IDS706_Hwk8/actions/workflows/CI.yml/badge.svg)](https://github.com/bionicotaku/Evan_Li_IDS706_Hwk8/actions/workflows/CI.yml)

## Overview

This project is a Rust-based command-line tool. The tool reads user input and provides feedback via the command line. It includes a functional CI/CD pipeline using GitHub Actions and an optional Docker container setup for development and deployment.

### Key Features

- **Rust-based command-line tool**: Written in Rust, read two numbers from command-line and print the addition and subtraction value.
- **CI/CD Pipeline**: Automated builds, testing, formatting, and releases with GitHub Actions.
- **Devcontainer**: Containerized development environment for consistent workflows in VSCode.
- **Packaged Binary**: Built and distributed using a release build in CI/CD.


### Functionality

This Rust-based command-line tool performs two basic arithmetic operations:

1. Addition: It adds two integer numbers provided by the user.
2. Subtraction: It subtracts the second integer from the first integer provided by the user.

The tool uses the `clap` crate for parsing command-line arguments, making it easy to use and extend in the future.

### Usage

To use this tool, follow these steps:

1. Ensure you have Rust installed on your system.
2. Clone the repository and navigate to the project directory (add_sub).
3. Build the project using cargo:
   ```
   cargo build --release
   ```
4. Run the tool from the command line, providing two integer arguments:
   ```
   ./target/release/add_sub <first_number> <second_number>
   ```
   For example:
   ```
   ./target/release/add_sub 10 5
   ```
   This will output:
   ```
   Addition result: 15
   Subtraction result: 5
   ```

You can also use the provided Makefile to build and run the project:

1. To build and run with default values (100 and 99):
   ```
   make all
   ```
2. To run with custom values:
   ```
   cargo run -- <first_number> <second_number>
   ```

Note: The tool is set up with a CI/CD pipeline using GitHub Actions, which automatically builds, tests, and releases the binary.
