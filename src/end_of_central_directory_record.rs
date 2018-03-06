#[derive(Serialize, Debug)]
struct EndOfCentralDirectoryRecord {
    signature: u32,
    disk: u16,
    central_dir_disk: u16,
    entry_number: u16,
    total_entry_number: u16,
    central_dir_size: u32,
    central_dir_offset: u32,
    comment_length: u16
}
