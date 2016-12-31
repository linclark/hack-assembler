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

#[test]
fn encodes_a_command() {
    let encoded = encode_line("@8");
    assert_eq!(encoded, "0000000000001000");
}
