pub mod file;
pub mod symbol_table;
pub mod parser;
pub mod translator;

fn main() {
    let content = file::get_file();

    let lines = file::parse_lines(&content);

    for line in lines {
        if !isLineComment(&line) && !line.is_empty() {
            let instr = parser::Instruction::new(&line);
//            instr.get_info();

            if instr.kind == Some(parser::InstructionType::C) {
                println!("Line: {}\nCode:{}", line, instr.get_code());
            }
        }
    }
}

fn isLineComment(line: &str) -> bool {
    if line.find("//") != None {
        return true
    }

    return false
}