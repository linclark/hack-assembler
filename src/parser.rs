use nom::{IResult, digit, not_line_ending, alphanumeric,multispace, space};
use std::str::{self, FromStr};

pub fn parse(asm: String) -> Vec<Command> {
    asm.lines()
       .map(|x| {
            match command(x.as_bytes()) {
                IResult::Done(I, O) => O,
                IResult::Incomplete(Needed) => panic!(Needed),
                IResult::Error(Error) => panic!(Error)
            }
       })
       .collect::<Vec<Command>>()
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
fn parses_a_command() {
    let res = command(b"@8");
    assert_eq!(command(b"@8"), IResult::Done(&b""[..], Command::ACommand{address: 8}));
}
