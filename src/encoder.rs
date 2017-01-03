use constants::{Command, OPCODES, DESTCODES, JUMPCODES};
use std::fmt;

pub fn encode(ref command_nodes: &Vec<Command>) -> String {
    let encoded = command_nodes.iter().map(|node| format!("{:016b}", node));
    encoded.collect::<Vec<String>>().join("\n")
}

impl fmt::Binary for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output = match *self {
            Command::ACommand { address: address } => address,
            Command::CCommand { dest: ref dest, comp: ref comp, jump: ref jump } => {
                // command flag + 2 filler bits
                0b111u16
                * 2_u16.pow(13)
                // opcode
                + OPCODES.get(comp.as_str()).unwrap()
                * 2_u16.pow(6)
                // dest
                + DESTCODES.get(dest.clone()
                                    .or(Some(String::from("0")))
                                    .unwrap()
                                    .as_str()).unwrap()
                * 2_u16.pow(3)
                // jump
                + JUMPCODES.get(jump.clone()
                                    .or(Some(String::from("0")))
                                    .unwrap()
                                    .as_str()).unwrap()
            }
        };
        write!(f, "{:016b}", output)
    }
}

#[test]
fn encodes_a_command() {
    let command_nodes = vec![Command::ACommand { address: 8 }, Command::ACommand { address: 3231 }];
    let encoded = encode(&command_nodes);
    assert_eq!(encoded, "0000000000001000\n0000110010011111");
}

#[test]
fn encodes_c_command() {
    let command_nodes = vec![Command::CCommand {
                                 dest: None,
                                 comp: String::from("1"),
                                 jump: None,
                             },
                             Command::CCommand {
                                 dest: None,
                                 comp: String::from("D+A"),
                                 jump: None,
                             }];
    let encoded = encode(&command_nodes);
    assert_eq!(encoded, "1110111111000000\n1110000010000000");
}

#[test]
fn encodes_c_command_with_dest_and_jump() {
    let command_nodes = vec![Command::CCommand {
                                 dest: Some(String::from("D")),
                                 comp: String::from("D+A"),
                                 jump: Some(String::from("JGT")),
                             }];
    let encoded = encode(&command_nodes);
    assert_eq!(encoded, "1110000010010001");
}

#[test]
fn encodes_c_command_with_jump() {
    let command_nodes = vec![Command::CCommand {
                                 dest: None,
                                 comp: String::from("D+A"),
                                 jump: Some(String::from("JGT")),
                             }];
    let encoded = encode(&command_nodes);
    assert_eq!(encoded, "1110000010000001");
}
