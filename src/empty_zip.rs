use std::fs;
use bincode::serialize_into;
use end;
use zip;

pub struct EmptyZip {}

impl zip::Zip for EmptyZip {
    const END: end::EndOfCentralDirectoryRecord = end::EndOfCentralDirectoryRecord {
        signature: 0x06054b50,
        disk: 0,
        central_dir_disk: 0,
        entry_number: 0, total_entry_number: 0,
        central_dir_size: 0,
        central_dir_offset: 0,
        comment_length: 0
    };

    fn generate(&self) {
        let mut f = fs::File::create("empty.zip").unwrap();
        serialize_into(&mut f, &EmptyZip::END).unwrap();
    }
}
