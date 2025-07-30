use std::env;
use std::path::PathBuf;

fn main() -> Result<(), tonic_buf_build::error::TonicBufBuildError> {
    println!("cargo:warning=Advanced example: Building with file_descriptor_set_path for mock server");
    
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    
    // Example showing how the old API looked:
    // tonic_buf_build::compile_from_buf(
    //     tonic_build::Builder::new()
    //         .file_descriptor_set_path(out_dir.join("services_descriptor.bin")),
    //     None,
    // )?;
    
    // New API with tonic-prost-build 0.14.0:
    tonic_buf_build::compile_from_buf_with_builder_config(|builder| {
        builder
            .file_descriptor_set_path(out_dir.join("services_descriptor.bin"))
            .build_server(true)   // Generate server code for mock implementation
            .build_client(true)   // Also generate client code for testing
    })?;
    
    println!("cargo:warning=Successfully generated proto files with descriptor at {:?}", 
             out_dir.join("services_descriptor.bin"));
    
    Ok(())
}