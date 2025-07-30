fn main() -> Result<(), tonic_buf_build::error::TonicBufBuildError> {
    println!("cargo:warning=BUILD.RS STARTED!");
    println!("BUILD.RS: Starting compilation...");
    let result = tonic_buf_build::compile_from_buf();
    println!("cargo:warning=BUILD.RS RESULT: {:?}", result);
    println!("BUILD.RS: Result: {:?}", result);
    result?;
    println!("cargo:warning=BUILD.RS FINISHED!");
    println!("BUILD.RS: Finished successfully!");
    Ok(())
}
