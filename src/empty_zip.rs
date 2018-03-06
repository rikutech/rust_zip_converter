#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate bincode;
extern crate rustc_serialize;

use std::fs;
use std::io::{self, Read};
use bincode::serialize_into;

struct EmptyZip {}

impl Zip for EmptyZip {
    const END = EndOfCentralDirectoryRecord {
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
        serialize_into(&mut f, &self::END).unwrap();
    }
}
