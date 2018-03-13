use std::io::{self};
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate bincode;
extern crate rustc_serialize;

mod end;
mod zip;
mod empty_zip;
use zip::Zip;

fn main() {
    println!("what do you wanna do?");
    println!("em: generate empty zip without comment");
    println!("em-c: generate empty zip with comment");

    let mut buffer: String = String::new();
    let _ = io::stdin().read_line(&mut buffer).unwrap();
    match buffer.trim().as_ref() as &str {
        "em" => empty_zip::EmptyZip {}.generate(),
        _ => println!("no match"),
    }

    println!("empty.zip generated!!!!!!!");
}

