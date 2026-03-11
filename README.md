# rustcroissant

[![Rust](https://img.shields.io/badge/rust-1.92.0%2B-green.svg?maxAge=3600)](https://github.com/aqora-io/rustcroissant)

A Rust implementation for working with the ML Commons Croissant metadata format,
a standardized way to describe machine learning datasets using JSON-LD.

## Installation

### Cargo

```bash
cargo install --git https://github.com/aqora-io/rustcroissant --features "arrow, parquet"
```

## Usage

```bash
rustcroissant --help

rustcroissant generate ./tests/parquet/titanic.parquet ./titanic.jsonld
```
