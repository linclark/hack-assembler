use nom::{IResult, alpha, anychar, digit, non_empty, not_line_ending, multispace, space};
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
    ACommand { address: u16 },
    CCommand {
        dest: Option<String>,
        comp: String,
        jump: Option<String>
    }
}

named!(command<Command>,
  ws!(
    alt_complete!(
        a_command => { |address| Command::ACommand{address: address} } |
        c_command => { |(dest, comp, jump)| Command::CCommand{dest: dest, comp: comp, jump: jump} }
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

named!(c_command <&[u8], (Option<String>, String, Option<String>)>,
    do_parse!(
        dest: opt!(
            complete!(
                terminated!(
                    map_res!(
                        map_res!(
                            alpha,
                            str::from_utf8
                        ),
                        str::FromStr::from_str
                    ),
                    tag!("=")
                )
            )
        ) >>
        comp: map_res!(
            map_res!(
                is_a!("ADM-+"),
                str::from_utf8
            ),
            str::FromStr::from_str
        ) >>
        jump: opt!(
            complete!(
                preceded!(
                    tag!(";"),
                    map_res!(
                        map_res!(
                            non_empty,
                            str::from_utf8
                        ),
                        str::FromStr::from_str
                    )
                )
            )
        ) >>
        (dest, comp, jump)
    )
);

#[test]
fn parses_a_command() {
    let result = command(b"@8");
    assert_eq!(result, IResult::Done(&b""[..], Command::ACommand{address: 8}));
}

#[test]
fn parses_c_command() {
    let result = command(b"M-D");
    assert_eq!(result, IResult::Done(&b""[..], Command::CCommand{dest: None, comp: String::from("M-D"), jump: None}));
}

#[test]
fn parses_c_command_with_jump() {
    let result = command(b"M-D;JMP");
    assert_eq!(result, IResult::Done(&b""[..], Command::CCommand{dest: None, comp: String::from("M-D"), jump: Some(String::from("JMP"))}));
}

#[test]
fn parses_c_command_with_dest() {
    let result = command(b"D=M-D");
    assert_eq!(result, IResult::Done(&b""[..], Command::CCommand{dest: Some(String::from("D")), comp: String::from("M-D"), jump: None}));
}
