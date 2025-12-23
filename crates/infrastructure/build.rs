fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("../../proto/tenant/tenant.proto")?;
    tonic_build::compile_protos("../../proto/staff/staff.proto")?;
    Ok(())
}
