/// Returns the binary code of dest
pub fn dest_code<'a>(str: &'a str) -> &'a str {
    match str {
        "M" => "001",
        "D" => "010",
        "DM" | "MD" => "011",
        "A" => "100",
        "AM" | "MA" => "101",
        "AD" | "DA" => "110",
        "ADM" | "AMD" | "MAD" | "MDA" | "DMA" | "DAM" => "111",
        &_ => "000"
    }
}

/// Returns the binary code of comp
//TODO: Change to return tuple (a, ccccccc) where a is 0 or 1
pub fn comp_code<'a>(str: &'a str) -> (&'a str, &'a str) {
    match str {
        "0" => ("0", "101010"),
        "1" => ("0", "111111"),
        "-1" => ("0", "111010"),
        "D" => ("0", "001100"),
        "A" => ("0", "110000"),
        "M" => ("1", "110000"),
        "!D" => ("0", "001101"),
        "!A" => ("0", "110001"),
        "!M" => ("1", "110001"),
        "-D" => ("0", "001111"),
        "-A" => ("0", "110011"),
        "-M" => ("1", "110011"),
        "D+1" => ("0", "011111"),
        "A+1" => ("0","110111"),
        "M+1" => ("1", "110111"),
        "D-1" => ("0", "001110"),
        "A-1" => ("0", "110010"),
        "M-1" => ("1", "110010"),
        "D+A" => ("0", "000010"),
        "D+M" => ("1", "000010"),
        "D-A" => ("0", "010011"),
        "D-M" => ("1", "010011"),
        "A-D" => ("0", "000111"),
        "M-D" => ("1", "000111"),
        "D&A" |"A&D" => ("0", "000000"),
        "D&M" | "M&D" => ("1", "000000"),
        "D|A" | "A|D" => ("0","010101"),
        "D|M" | "M|D" => ("1", "010101"),
        &_ => panic!("instr: {}", str)
    }
}

/// Returns the binary code of jump
pub fn jump_code<'a>(str: &'a str) -> &'a str {
    match str {
        "JGT" => "001",
        "JEQ" => "010",
        "JGE" => "011",
        "JLT" => "100",
        "JNE" => "101",
        "JLE" => "110",
        "JMP" => "111",
        &_ => "000"
    }
}

#[cfg(test)]
mod translator_tests {
    use super::*;

    #[test]
    pub fn dest_test() {
        assert_eq!(dest_code("D"), "010");
    }

    #[test]
    pub fn comp_test() {
        assert_eq!(comp_code("D|A"), "010101");
    }

    #[test]
    pub fn jump_test() {
        assert_eq!(jump_code("JLE"), "110");
    }
}