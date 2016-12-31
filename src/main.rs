// open.rs
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::env;

fn main() {
    let assembly = get_assembly();
    let machine_code = translate(assembly);
    write_machine_code(machine_code);
}

fn translate(assembly: String) -> String {
    assembly.lines()
            .map(|x| encode_line(x))
            .collect()
}

fn encode_line(line: &str) -> String {
    let mut l = match line {
        line if line.contains("@") => {
            format!("{:016b}", line[1..].parse::<u16>().unwrap())
        },
        _ => String::from(line)
    };
    l.push_str("\n");
    l
}

fn get_assembly() -> String {
    let mut args = env::args();
    let filename = args.nth(1).unwrap();

    let path = Path::new(filename.as_str());
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(err) => panic!("couldn't open {}: {}", display, err.description()),
        Ok(file) => file,
    };

    let mut s = String::new();
    let assembly = match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   why.description()),
        Ok(_) => s,
    };
    assembly
}

fn write_machine_code(machine_code: String) {
    let mut args = env::args();
    let filename = args.nth(2).unwrap();

    let path = Path::new(filename.as_str());
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(err) => panic!("couldn't create {}: {}", display, err.description()),
        Ok(file) => file,
    };

    match file.write_all(machine_code.as_bytes()) {
        Err(err) => {
            panic!("couldn't write to {}: {}", display, err.description())
        },
        Ok(_) => println!("successfully wrote to {}", display),
    }
}
