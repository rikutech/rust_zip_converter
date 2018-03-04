#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate bincode;
extern crate rustc_serialize;

use std::fs;
use bincode::serialize_into;

fn main() {
    println!("generating empty zip...");
    //is this weird? dont know the rust way to define key-value type.
    let end = EndOfCentralDirectoryRecord {
        signature: 0x06054b50,
        disk: 0,
        central_dir_disk: 0,
        entry_number: 0, total_entry_number: 0,
        central_dir_size: 0,
        central_dir_offset: 0,
        comment_length: 0
    };

    let mut f = fs::File::create("empty.zip").unwrap();
    serialize_into(&mut f, &end).unwrap();
    println!("empty.zip generated!!!!!!!");
}

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
