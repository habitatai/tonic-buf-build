fn main() -> Result<(), tonic_buf_build::error::TonicBufBuildError> {
    let result = tonic_buf_build::compile_from_buf();
    result?;
    Ok(())
}
