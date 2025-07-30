# tonic-buf-build

[![Crates.io](https://img.shields.io/crates/v/tonic-buf-build.svg)](https://crates.io/crates/tonic-buf-build)
[![Documentation](https://docs.rs/tonic-buf-build/badge.svg)](https://docs.rs/tonic-buf-build)
[![License](https://img.shields.io/crates/l/tonic-buf-build.svg)](./LICENSE.md)

A build helper that allows you to integrate [buf.build](https://buf.build) with [tonic-prost-build](https://github.com/hyperium/tonic).
Using buf.build and tonic, you can easily manage third party dependencies for proto files and generate Rust code for your proto files.

## Features

- 🚀 **Simple API** - Just 3 functions for all use cases
- 📦 **Buf Integration** - Works with [buf.yaml](https://buf.build/docs/configuration/v1/buf-yaml) and [buf.work.yaml](https://buf.build/docs/configuration/v1/buf-work-yaml)
- 🏗️ **Modern Stack** - Uses `tonic-prost-build` for the latest tonic ecosystem
- 🔧 **Flexible Configuration** - Support for advanced tonic builder options
- 📚 **Well Documented** - Complete examples and migration guides

## Installation

Add the following to your `Cargo.toml`:

```toml
[build-dependencies]
tonic-buf-build = "0.14"
tonic-prost-build = "0.14"
```

## Quick Start

### Basic Usage

Create a `build.rs` file in your project root:

```rust
fn main() -> Result<(), tonic_buf_build::error::TonicBufBuildError> {
    tonic_buf_build::compile_from_buf()?;
    Ok(())
}
```

This will:

1. Read your `buf.yaml` configuration
2. Download dependencies via buf
3. Generate Rust code using tonic-prost-build

### Buf Workspaces

If you're using [buf workspaces](https://buf.build/docs/configuration/v1/buf-work-yaml):

```rust
fn main() -> Result<(), tonic_buf_build::error::TonicBufBuildError> {
    tonic_buf_build::compile_from_buf_workspace()?;
    Ok(())
}
```

### Advanced Configuration

For advanced tonic configuration (file descriptors, server/client generation, etc.):

```rust
use std::env;
use std::path::PathBuf;

fn main() -> Result<(), tonic_buf_build::error::TonicBufBuildError> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    
    tonic_buf_build::compile_from_buf_with_builder_config(|builder| {
        builder
            .file_descriptor_set_path(out_dir.join("descriptors.bin"))
            .build_server(true)
            .build_client(true)
            .build_transport(true)
    })?;
    Ok(())
}
```

### Custom Directory Usage

When proto files are not in the current directory (common in monorepos):

```rust
use std::path::PathBuf;
use tonic_buf_build::TonicBufConfig;

fn main() -> Result<(), tonic_buf_build::error::TonicBufBuildError> {
    let config = TonicBufConfig {
        buf_dir: Some(PathBuf::from("../shared-protos")),
    };
    
    tonic_buf_build::compile_from_buf_with_config(&config)?;
    Ok(())
}
```

## Examples

The repository includes complete working examples:

- **[buf-yaml](./examples/buf-yaml/)** - Basic buf.yaml usage
- **[buf-work-yaml](./examples/buf-work-yaml/)** - Buf workspace usage
- **[buf-yaml-advanced](./examples/buf-yaml-advanced/)** - Advanced configuration with file descriptors
- **[buf-yaml-custom-dir](./examples/buf-yaml-custom-dir/)** - Using proto files from custom directory

Run any example:

```bash
cd examples/buf-yaml
cargo run
```

## Migration from 0.13

If you're upgrading from version 0.13, see our [Migration Guide](./MIGRATION_0.13-0.14.md) for detailed instructions.

**Quick migration summary:**

- Update `tonic-build` → `tonic-prost-build` in Cargo.toml
- Remove function parameters: `compile_from_buf()` instead of `compile_from_buf(builder, config)`
- Use closure for advanced config: `compile_from_buf_with_builder_config(|builder| ...)`

## Requirements

- Rust 1.70+
- [buf CLI](https://buf.build/docs/installation) installed and available in PATH
- Valid `buf.yaml` or `buf.work.yaml` in your project

## Troubleshooting

### "buf command not found"

Install the buf CLI: <https://buf.build/docs/installation>

### "No buf.yaml found"

Make sure you have a `buf.yaml` file in your project root or the directory where you run the build.

### "Build failures after upgrade"

Check the [Migration Guide](./MIGRATION_0.13-0.14.md) for breaking changes between versions.

## Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Ensure all examples still work
5. Submit a pull request

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.
