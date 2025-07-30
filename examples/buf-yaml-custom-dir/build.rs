use std::path::PathBuf;

fn main() -> Result<(), tonic_buf_build::error::TonicBufBuildError> {
    println!("cargo:warning=Custom Directory example: Building with proto files in ./proto/ directory");
    
    // Example of using custom directory for buf files
    // This is useful when:
    // - Proto files are in a subdirectory (common in monorepos)
    // - Sharing proto files between multiple language clients
    // - Organizing proto files separately from Rust code
    
    let proto_dir = PathBuf::from("proto");
    let config = tonic_buf_build::TonicBufConfig {
        buf_dir: Some(proto_dir),
    };
    
    tonic_buf_build::compile_from_buf_with_config(&config)?;
    
    println!("cargo:warning=Successfully compiled proto files from custom directory: ./proto/");
    Ok(())
}