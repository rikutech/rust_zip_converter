use std::io::{self, Read};

fn main() {
    println!("what do you wanna do?");
    println!("em: generate empty zip without comment");
    println!("em-c: generate empty zip with comment");

    let mut buffer = String::new();
    let _ = io::stdin().read_line(&mut buffer).unwrap();
    match buffer {
        "em" => EmptyZip.generate(),
        _ => println!("no match"),
    }

    println!("empty.zip generated!!!!!!!");
}

