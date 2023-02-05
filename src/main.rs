pub mod file;
pub mod symbol_table;
pub mod parser;
pub mod translator;

fn main() {
    let instr = parser::Instruction::new("A=D+1;JMP");

    instr.get_info();
}
