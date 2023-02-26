# Project structure

## Top level

### `pyproject.toml`

This is a Poetry project, mostly for managing the virtual environment

### `Cargo.toml`

Cargo workspace

## Projects

### string_sum

The main library project, compiled both as a Rust lib (for the CLI to link to),
and as a C dynamic library for the Python bindings

### string_cli

Pure Rust CLI

### string_python

Python CLI using the `string_sum` module

# Building

- virtual environment: `poetry shell` at the top level
- building the library: (with venv active) `maturin develop` in `string_sum`
- building the Rust CLI: `cargo build` in `string_cli`