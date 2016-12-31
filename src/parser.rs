pub fn parse(asm: String) -> String {
    asm.lines()
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
