#[derive(Serialize, Debug)]
pub struct EndOfCentralDirectoryRecord {
    pub signature: u32,
    pub disk: u16,
    pub central_dir_disk: u16,
    pub entry_number: u16,
    pub total_entry_number: u16,
    pub central_dir_size: u32,
    pub central_dir_offset: u32,
    pub comment_length: u16
}
