# Migration Guide: tonic-buf-build 0.13 → 0.14

This guide helps you migrate from tonic-buf-build 0.13 to 0.14, which introduces significant API simplifications and updates to use `tonic-prost-build` instead of `tonic-build`.

## API Changes

### Basic Usage Migration

**0.13 (Old):**

```rust
fn main() -> Result<(), tonic_buf_build::error::TonicBufBuildError> {
    tonic_buf_build::compile_from_buf(tonic_build::configure(), None)?;
    Ok(())
}
```

**0.14 (New):**

```rust
fn main() -> Result<(), tonic_buf_build::error::TonicBufBuildError> {
    tonic_buf_build::compile_from_buf()?;
    Ok(())
}
```

### Workspace Usage Migration

**0.13 (Old):**

```rust
fn main() -> Result<(), tonic_buf_build::error::TonicBufBuildError> {
    tonic_buf_build::compile_from_buf_workspace(tonic_build::configure(), None)?;
    Ok(())
}
```

**0.14 (New):**

```rust
fn main() -> Result<(), tonic_buf_build::error::TonicBufBuildError> {
    tonic_buf_build::compile_from_buf_workspace()?;
    Ok(())
}
```

### Advanced Configuration Migration

**0.13 (Old):**

```rust
fn main() -> Result<(), tonic_buf_build::error::TonicBufBuildError> {
    let builder = tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("descriptor.bin"))
        .build_server(false);
    tonic_buf_build::compile_from_buf(builder, None)?;
    Ok(())
}
```

**0.14 (New):**

```rust
fn main() -> Result<(), tonic_buf_build::error::TonicBufBuildError> {
    tonic_buf_build::compile_from_buf_with_builder_config(|builder| {
        builder
            .file_descriptor_set_path(out_dir.join("descriptor.bin"))
            .build_server(false)
    })?;
    Ok(())
}
```

### Custom Directory Configuration

**0.13 (Old):**

```rust
use tonic_buf_build::TonicBufConfig;

fn main() -> Result<(), tonic_buf_build::error::TonicBufBuildError> {
    let config = TonicBufConfig {
        buf_dir: Some(PathBuf::from("../proto")),
    };
    tonic_buf_build::compile_from_buf_with_config(
        tonic_build::configure(),
        None,
        &config
    )?;
    Ok(())
}
```

**0.14 (New):**

```rust
use std::path::PathBuf;
use tonic_buf_build::TonicBufConfig;

fn main() -> Result<(), tonic_buf_build::error::TonicBufBuildError> {
    let config = TonicBufConfig {
        buf_dir: Some(PathBuf::from("../proto")),
    };
    tonic_buf_build::compile_from_buf_with_config(&config)?;
    Ok(())
}
```

## Cargo.toml Changes

**0.13 (Old):**

```toml
[build-dependencies]
tonic-buf-build = "0.13"
tonic-build = "0.11"
```

**0.14 (New):**

```toml
[build-dependencies]
tonic-buf-build = "0.14"
tonic-prost-build = "0.14"  # Note: tonic-build is no longer needed
```

## Removed Functions

## API Changes Summary

**New simplified API in 0.14:**
- `compile_from_buf()` - Basic usage  
- `compile_from_buf_workspace()` - Workspace usage
- `compile_from_buf_with_builder_config()` - Advanced builder configuration
- `compile_from_buf_with_config()` - Custom directory support
- `compile_from_buf_workspace_with_config()` - Workspace with custom directory
- `compile_from_buf_with_builder_and_buf_config()` - Advanced + custom directory

**Removed complex parameter functions:**
- Old functions that required `tonic_builder` and `config` parameters are gone
- Replaced with simpler functions that take closures or configuration structs

## Migration Steps

1. **Update Cargo.toml dependencies**:
   - Change `tonic-build` to `tonic-prost-build = "0.14"`
   - Update `tonic-buf-build` to `"0.14"`

2. **Simplify your build.rs**:
   - Remove `tonic_build::configure()` and `None` parameters
   - Use the new simplified function calls

3. **Update advanced configurations**:
   - Wrap your builder configuration in a closure for `compile_from_buf_with_builder_config`

4. **Test your build**:
   - Run `cargo build` to ensure everything works
   - Check that generated code is still correct

## Benefits of Migration

- **Simpler API**: Much easier to use for common cases
- **Better performance**: Fewer allocations and simplified code paths
- **Cleaner code**: Less boilerplate in your build.rs files
- **Future-proof**: Based on the latest tonic ecosystem updates

## Need Help?

- Check the `examples/` directory for working examples of all use cases
- Review the updated README.md for current best practices
- Open an issue if you encounter migration problems

## Backward Compatibility

This is a **breaking change**. Version 0.14 is not backward compatible with 0.13 due to the API simplification. Please follow this migration guide to update your code.
