fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("../../schema/proto/tenant/tenant.proto")?;
    tonic_build::compile_protos("../../schema/proto/staff/staff.proto")?;
    Ok(())
}
