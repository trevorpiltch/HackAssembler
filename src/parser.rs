use regex::Regex;

pub fn decode_A_instruction(instr: &str) {
    let re = Regex::new("@([A-Z0-9-]+)");

    let captures = re.captures_iter(instr);

    for cap in captures {
        println!("Instruction: {}", &cap[0]);
    }
}