fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("../../proto/tenant.proto")?;
    tonic_build::compile_protos("../../proto/staff.proto")?;
    Ok(())
}
