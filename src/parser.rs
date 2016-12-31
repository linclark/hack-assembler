use nom::{IResult, digit, not_line_ending, alphanumeric,multispace, space};
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

#[derive(Debug,PartialEq)]
pub enum Command {
    ACommand { address: u16 }
}

named!(command<Command>,
  ws!(
    alt!(
      a_command => { |address|   Command::ACommand{address: address} }
    )
  )
);

named!(a_command <&[u8], u16>,
    preceded!(
        tag!("@"),
        map_res!(
            map_res!(digit, str::from_utf8),
            str::FromStr::from_str
        )
    )
);

#[test]
fn encodes_a_command() {
    let encoded = encode_line("@8");
    assert_eq!(encoded, "0000000000001000");
}

#[test]
fn parses_a_command() {
    let res = command(b"@8");
    assert_eq!(command(b"@8"), IResult::Done(&b""[..], Command::ACommand{address: 8}));
}
