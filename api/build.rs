fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/grpc.proto")?;
    tonic_build::compile_protos("proto/types.proto")?;
    Ok(())
}