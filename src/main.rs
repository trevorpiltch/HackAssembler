pub mod file;
pub mod symbol_table;
pub mod parser;

fn main() {
    let instr = parser::Instruction::new("D=A;JMP");

    println!("{:?} {:?} {:?}", instr.dest(), instr.comp(), instr.jump());
}
