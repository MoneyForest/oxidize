use oxidize_domain::Staff;

#[derive(Debug)]
pub struct ListStaffOutput {
    pub staff: Vec<Staff>,
    pub total_count: u64,
}
