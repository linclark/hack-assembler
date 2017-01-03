use phf;

pub static OPCODES: phf::Map<&'static str, u16> = phf_map! {
    "0" => 0b0101010u16,
    "1" => 0b0111111u16
};

#[derive(Debug,PartialEq)]
pub enum Command {
    ACommand { address: u16 },
    CCommand {
        dest: Option<String>,
        comp: String,
        jump: Option<String>
    }
}
