use std::fs;
use std::io::{BufWriter, Write};

fn main() {
    println!("generating empty zip...");
    //TODO: implement zip generation
    println!("empty.zip generated!!!!!!!");
}

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
