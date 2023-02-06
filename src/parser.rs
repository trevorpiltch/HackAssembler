use crate::translator;
use regex::Regex;

/// Helper struct for parsing instructions
pub struct Instruction<'a> {
    pub text: &'a str,
    pub kind: Option<InstructionType>
}

/// Type of instruction
#[derive(PartialEq, Debug)]
pub enum InstructionType {
    A,
    C,
    L
}

impl Instruction<'_> {
    /// Create a new instruction
    pub fn new(instr: &str) -> Instruction {
        Instruction {
            text: instr,
            kind: Self::instruction_type(&instr)
        }
    }

    /// Returns the type of the instruction
   fn instruction_type(instr: &str) -> Option<InstructionType> {
        let a = Regex::new("@([(a-z)(A-Z)(0-9)]+)").unwrap();
        let l = Regex::new(r"\(([(a-z)(A-Z)(0-9)]+)\)").unwrap();
        let has_dest = instr.find('=');
        let has_jump = instr.find(';');

        let mut instr_type: Option<InstructionType> = None;

        if a.is_match(instr) {
            instr_type = Some(InstructionType::A);
        }
        else if l.is_match(instr) {
            instr_type = Some(InstructionType::L);
        }
        else if (has_dest != None) || (has_jump != None) {
            instr_type = Some(InstructionType::C);
        }

        return instr_type
    }

    /// Returns the full binary code for the instruction (if its a c instruction)
    pub fn get_code(&self) -> String {
        let dest = self.dest();
        let comp = self.comp().unwrap();
        let jump = self.jump();

        let comp_code = translator::comp_code(&comp);

        let mut code: String = format!("1110{}", comp_code);

        if dest != None {
            code = format!("{}{}", code, translator::dest_code(&dest.unwrap()));
        }
        else {
            code = format!("{}{}", "000", code);
        }

        if jump != None {
            code = format!("{}{}", code, translator::jump_code(&jump.unwrap()));
        }
        else {
            code = format!("{}{}", code, "000");
        }

        return code;
    }

    /// Returns the symbol xxx in @xxx or (xxx)
    pub fn symbol(&self) -> Option<String> {
        if self.kind ==  Some(InstructionType::A) {
            let re = Regex::new("@([(a-z)(A-Z)(0-9)]+)").unwrap();
            let caps = re.captures(self.text).unwrap();

            return Some(caps[1].to_string())
        }
        else if self.kind == Some(InstructionType::L) {
            let re = Regex::new(r"\(([(a-z)(A-Z)(0-9)]+)\)").unwrap();
            let caps = re.captures(self.text).unwrap();

            return Some(caps[1].to_string())
        }

        return None
    }

    /// Returns the dest in dest=comp;jmp
    pub fn dest(&self) -> Option<String> {
        if self.kind == Some(InstructionType::C) {
            let has_dest = self.text.find('=');

            if has_dest != None {
                let dest: Vec<&str> = self.text.split('=').collect();

                return Some(dest[0].to_string())
            }
        }

        return None
    }

    /// Returns the comp in dest=comp;jmp
    pub fn comp(&self) -> Option<String> {
        if self.kind == Some(InstructionType::C) {
            let has_dest = self.text.find('=');
            let has_jump = self.text.find(';');

            if (has_dest != None) && (has_jump != None) {
                let split_1: Vec<&str> = self.text.split('=').collect();
                let split_2: Vec<&str> = split_1[1].split(';').collect();

                return Some(split_2[0].to_string())
            }
            else if has_dest != None {
                let split: Vec<&str> = self.text.split('=').collect();

                return Some(split[1].to_string())
            }
            else if has_jump != None {
                let split: Vec<&str> = self.text.split(';').collect();

                return Some(split[0].to_string());
            }
        }

        return None
    }

    /// Returns the jmp in dest=comp;jmp
    pub fn jump(&self) -> Option<String> {
        if self.kind == Some(InstructionType::C) {
            let has_jump = self.text.find(';');

            if has_jump != None {
                let split: Vec<&str> = self.text.split(';').collect();

                return Some(split[1].to_string())
            }
        }

        return None
    }

    /// Helper method to print instruction info to std output
    pub fn get_info(&self) {
        println!("Type: {:?}", self.kind);

        if self.kind != Some(InstructionType::C) {
            println!("Symbol: {:?}", self.symbol());
        }
        else {
            println!("Dest: {:?}", self.dest());
            println!("Comp: {:?}", self.comp());
            println!("Jump: {:?}", self.jump());
        }
    }
}

#[cfg(test)]
mod parser_tests {
    use super::*;

    #[test]
    fn c_all() {
        let all = Instruction::new("D=D+1;JMP");

        assert_eq!(all.symbol(), None);
        assert_eq!(all.dest().unwrap(), "D");
        assert_eq!(all.comp().unwrap(), "D+1");
        assert_eq!(all.jump().unwrap(), "JMP");
    }

    #[test]
    fn c_no_jump() {
        let no_jump = Instruction::new("DM=D+1");

        assert_eq!(no_jump.dest().unwrap(), "DM");
        assert_eq!(no_jump.comp().unwrap(), "D+1");
        assert_eq!(no_jump.jump(), None);
    }

    #[test]
    fn c_no_dest() {
        let no_dest = Instruction::new("0;JMP");

        assert_eq!(no_dest.dest(), None);
        assert_eq!(no_dest.comp().unwrap(), "0");
        assert_eq!(no_dest.jump().unwrap(), "JMP");
    }

    #[test]
    fn a_instr() {
        let a1 = Instruction::new("@17");
        let a2 = Instruction::new("@symbol");

        assert_eq!(a1.symbol().unwrap(), "17");
        assert_eq!(a2.symbol().unwrap(), "symbol");
    }

    #[test]
    fn l_instr() {
        let l1 = Instruction::new("(LABEL)");
        let l2 = Instruction::new("(ERROR)");
        let l3 = Instruction::new("(123)");

        assert_eq!(l1.symbol().unwrap(), "LABEL");
        assert_eq!(l2.symbol().unwrap(), "ERROR");
        assert_eq!(l3.symbol().unwrap(), "123");
    }
}