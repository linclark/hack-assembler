use nom::{digit, alphanumeric};
use nom::IResult::Done;
use std::str::{self, FromStr};

pub fn parse(asm: String) -> String {
    asm.lines()
       .map(|x| encode_line(x) + "\n")
       .collect()
}

fn encode_line(line: &str) -> String {
    match line {
        line if line.contains("@") => {
            format!("{:016b}", line[1..].parse::<u16>().unwrap())
        },
        _ => String::from(line)
    }
}

struct Command {
  a: u8
}

#[test]
fn encodes_a_command() {
    let encoded = encode_line("@8");
    assert_eq!(encoded, "0000000000001000");
}

#[test]
fn parses_a_command() {
    named!(command<(&[u8], &[u8])>, pair!(tag!("@"), digit));

    assert_eq!(command(b"@8"), Done(&b""[..], (&b"@"[..], &b"8"[..])));
}

