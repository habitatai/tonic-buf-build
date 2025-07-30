//! tonic-buf-build allows you to integrate [buf.build](https://buf.build) with [tonic-build](https://github.com/hyperium/tonic/tree/master/tonic-build).
//! Using buf.build and tonic, you can easily manage third party dependencies for proto files and generate code for your proto files in Rust.
//! Works with both [buf.yaml](https://buf.build/docs/configuration/v1/buf-yaml) and [buf.work.yaml](https://buf.build/docs/configuration/v1/buf-work-yaml).
//!
//!
//! ## Usage
//!
//! Add the following to your Cargo.toml:
//!
//! ```toml
//! tonic-buf-build = "*"
//! tonic-build = "*"
//! ```
//!
//! Then, in your build.rs:
//!
//! ```rust
//! fn main() -> Result<(), tonic_buf_build::error::TonicBufBuildError> {
//!    tonic_buf_build::compile_from_buf()?;
//!    Ok(())
//! }
//! ```
//!
//! To use buf workspaces, you simply call `tonic_buf_build::compile_from_buf_workspace` instead.
//!
//! For complete and working examples, take a look at the examples folder.
//!

use scopeguard::defer;
use std::path::{Path, PathBuf};

use crate::error::TonicBufBuildError;

mod buf;
pub mod error;

fn tempdir() -> PathBuf {
    std::env::temp_dir().join(uuid::Uuid::new_v4().to_string())
}

/// Configuration for tonic-buf-build compilation.
#[derive(Default)]
pub struct TonicBufConfig<P: AsRef<Path> = &'static str> {
    pub buf_dir: Option<P>,
}

/// Compiles protobuf files using buf workspace configuration.
///
/// This function reads buf.work.yaml from the current directory to determine
/// workspace structure and compiles all protobuf files found in the workspace.
///
/// # Errors
///
/// Returns `TonicBufBuildError` if:
/// - buf.work.yaml file cannot be read or parsed
/// - buf binary execution fails
/// - Proto file compilation fails
/// - File system operations fail during compilation
pub fn compile_from_buf_workspace() -> Result<(), TonicBufBuildError> {
    compile_from_buf_workspace_with_config(&TonicBufConfig::<&str>::default())
}

/// Compiles protobuf files using buf workspace configuration with custom config.
///
/// Similar to `compile_from_buf_workspace` but allows specifying custom
/// configuration options such as the buf directory.
///
/// # Errors
///
/// Returns `TonicBufBuildError` if:
/// - buf.work.yaml file cannot be read or parsed
/// - buf binary execution fails
/// - Proto file compilation fails
/// - File system operations fail during compilation
pub fn compile_from_buf_workspace_with_config<P: AsRef<Path>>(
    tonic_buf_config: &TonicBufConfig<P>,
) -> Result<(), TonicBufBuildError> {
    let export_dir = tempdir();
    defer! {
        // This is just cleanup, it's not important if it fails
        let _ = std::fs::remove_dir(&export_dir);
    }
    let buf_dir: &Path = match &tonic_buf_config.buf_dir {
        Some(dir) => dir.as_ref(),
        None => Path::new("."),
    };

    let buf_work = buf::BufWorkYaml::load(&PathBuf::from(buf_dir).join("buf.work.yaml"))?;

    buf::export_all_from_workspace(&buf_work, &export_dir, buf_dir)?;
    let protos = buf::ls_files(&export_dir)?;
    let includes = vec![
        buf_dir.to_string_lossy().to_string(),
        export_dir.to_string_lossy().to_string(),
    ];

    tonic_prost_build::configure()
        .compile_protos(&protos, &includes)
        .map_err(|e| TonicBufBuildError::new("error running tonic build", e.into()))
}

/// Compiles protobuf files using buf configuration.
///
/// This function reads buf.yaml from the current directory and compiles
/// protobuf files according to the buf configuration.
///
/// # Errors
///
/// Returns `TonicBufBuildError` if:
/// - buf.yaml file cannot be read or parsed
/// - buf binary execution fails  
/// - Proto file compilation fails
/// - File system operations fail during compilation
pub fn compile_from_buf() -> Result<(), TonicBufBuildError> {
    compile_from_buf_with_config(&TonicBufConfig::<&str>::default())
}

/// Compiles protobuf files using buf configuration with custom config.
///
/// Similar to `compile_from_buf` but allows specifying custom configuration
/// options such as the buf directory.
///
/// # Errors
///
/// Returns `TonicBufBuildError` if:
/// - buf.yaml file cannot be read or parsed
/// - buf binary execution fails
/// - Proto file compilation fails  
/// - File system operations fail during compilation
pub fn compile_from_buf_with_config<P: AsRef<Path>>(
    tonic_buf_config: &TonicBufConfig<P>,
) -> Result<(), TonicBufBuildError> {
    let export_dir = tempdir();
    defer! {
        // This is just cleanup, it's not important if it fails
        let _ = std::fs::remove_dir(&export_dir);
    }
    let buf_dir: &Path = match &tonic_buf_config.buf_dir {
        Some(dir) => dir.as_ref(),
        None => Path::new("."),
    };

    let buf = buf::BufYaml::load(&PathBuf::from(buf_dir).join("buf.yaml"))?;

    buf::export_all(&buf, buf_dir, &export_dir)?;
    let protos = buf::ls_files(&export_dir)?;
    let includes = vec![export_dir.to_string_lossy().to_string()];

    tonic_prost_build::configure()
        .compile_protos(&protos, &includes)
        .map_err(|e| TonicBufBuildError::new("error running tonic build", e.into()))
}

/// Compiles protobuf files using buf configuration with advanced `tonic_prost_build` configuration.
///
/// This function allows you to provide a closure that configures the `tonic_prost_build::Builder`.
/// This is useful for advanced options like `file_descriptor_set_path`, `build_server`, etc.
///
/// # Example
///
/// ```rust,no_run
/// use std::env;
/// use std::path::PathBuf;
///
/// fn main() -> Result<(), tonic_buf_build::error::TonicBufBuildError> {
///     let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
///     
///     tonic_buf_build::compile_from_buf_with_builder_config(|builder| {
///         builder
///             .file_descriptor_set_path(out_dir.join("services_descriptor.bin"))
///             .build_server(false)
///     })?;
///     Ok(())
/// }
/// ```
///
/// # Errors
///
/// Returns `TonicBufBuildError` if:
/// - buf.yaml file cannot be read or parsed
/// - buf binary execution fails
/// - Proto file compilation fails
/// - File system operations fail during compilation
pub fn compile_from_buf_with_builder_config<F>(
    configure_builder: F,
) -> Result<(), TonicBufBuildError>
where
    F: FnOnce(tonic_prost_build::Builder) -> tonic_prost_build::Builder,
{
    compile_from_buf_with_builder_and_buf_config(
        configure_builder,
        &TonicBufConfig::<&str>::default(),
    )
}

/// Compiles protobuf files using buf configuration with advanced builder and buf configuration.
///
/// This function combines both buf directory configuration and `tonic_prost_build` configuration.
///
/// # Errors
///
/// Returns `TonicBufBuildError` if:
/// - buf.yaml file cannot be read or parsed
/// - buf binary execution fails
/// - Proto file compilation fails
/// - File system operations fail during compilation
pub fn compile_from_buf_with_builder_and_buf_config<F, P: AsRef<Path>>(
    configure_builder: F,
    tonic_buf_config: &TonicBufConfig<P>,
) -> Result<(), TonicBufBuildError>
where
    F: FnOnce(tonic_prost_build::Builder) -> tonic_prost_build::Builder,
{
    let export_dir = tempdir();
    defer! {
        // This is just cleanup, it's not important if it fails
        let _ = std::fs::remove_dir(&export_dir);
    }
    let buf_dir: &Path = match &tonic_buf_config.buf_dir {
        Some(dir) => dir.as_ref(),
        None => Path::new("."),
    };

    let buf = buf::BufYaml::load(&PathBuf::from(buf_dir).join("buf.yaml"))?;

    buf::export_all(&buf, buf_dir, &export_dir)?;
    let protos = buf::ls_files(&export_dir)?;
    let includes = vec![export_dir.to_string_lossy().to_string()];

    let builder = tonic_prost_build::configure();
    let final_builder = configure_builder(builder);

    final_builder
        .compile_protos(&protos, &includes)
        .map_err(|e| TonicBufBuildError::new("error running tonic build", e.into()))
}
