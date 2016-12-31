mod parser;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::env;
use std::process;

fn main() {
    let filename = match env::args().nth(1) {
        Some(x) => x,
        None    => {
            println!("usage: hack-assembler <input_file>");
            process::exit(1);
        },
    };
    let asm = read_file(&filename);

    let binary = parser::parse(asm);

    let destination = filename.replace(".asm", ".hack");
    write_binary(binary, destination);
}

fn read_file(source: &String) -> String {
    let path = Path::new(source.as_str());
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(err) => panic!("couldn't open {}: {}", display, err.description()),
        Ok(file) => file,
    };

    let mut s = String::new();
    let asm = match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   why.description()),
        Ok(_) => s,
    };
    asm
}

fn write_binary(binary: String, destination: String) {
    let path = Path::new(destination.as_str());
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(err) => panic!("couldn't create {}: {}", display, err.description()),
        Ok(file) => file,
    };

    match file.write_all(binary.as_bytes()) {
        Err(err) => {
            panic!("couldn't write to {}: {}", display, err.description())
        },
        Ok(_) => println!("successfully wrote to {}", display),
    }
}
