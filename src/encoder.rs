use parser::Command;

pub fn encode(ref command_nodes: &Vec<Command>) -> String {
    let encoded = command_nodes.iter().map(|node| {
        match *node {
            Command::ACommand {address: address} => format!("{:016b}", address),
            Command::CCommand {dest: ref dest, comp: ref comp, jump: ref jump} => format!("{:?}", comp)
        }
    });
    encoded.collect::<Vec<String>>().join("\n")
}

#[test]
fn encodes_a_command() {
    let command_nodes = vec!(Command::ACommand {address: 8}, Command::ACommand {address: 3231});
    let encoded = encode(&command_nodes);
    assert_eq!(encoded, "0000000000001000\n0000110010011111");
}
