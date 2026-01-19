mod Definations;
mod Functions;

use std::env;
use std::fs::File;
use std::io::{self, Read};

use crate::Functions::initiate::initiate;

fn get_file_ptr() -> Vec<u8> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!(
            r#"Please provide a file name
       Usage: {} <filename.exe>"#,
            args[0]
        );
        std::process::exit(1);
    }
    let filename = &args[1];
    let mut file = File::open(filename).unwrap();
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).unwrap();
    println!("Read {} bytes from {}", contents.len(), filename);
    return contents;
}
fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!(
            r#"Please provide a file name
       Usage: {} <filename.exe>"#,
            args[0]
        );
        std::process::exit(1);
    }
    let filename = &args[1];
    let file = File::open(filename).unwrap();
    initiate(file);
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(())
}
