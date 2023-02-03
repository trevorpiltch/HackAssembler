use regex::Regex;

pub struct Instruction<'a> {
    pub text: &'a str,
    pub kind: Option<InstructionType>
}

#[derive(PartialEq, Debug)]
pub enum InstructionType {
    A,
    C,
    L
}

impl Instruction<'_> {
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
        let c = Regex::new(r"((\w)?)(=?)((\w\W\d)?)(;?)(\w\w\w)?").unwrap();

        let mut instr_type: Option<InstructionType> = None;

        if a.is_match(instr) {
            instr_type = Some(InstructionType::A);
        }
        else if l.is_match(instr) {
            instr_type = Some(InstructionType::L);
        }
        else if c.is_match(instr) {
            instr_type = Some(InstructionType::C);
        }

        return instr_type
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
            let re = Regex::new(r"([A-Z]*)(=?)(\w\W\d|\w)?(;?)(\w\w\w)?").unwrap();
            let caps = re.captures(self.text).unwrap();

            return Some(caps[1].to_string())
        }

        return None
    }

    /// Returns the comp in dest=comp;jmp
    pub fn comp(&self) -> Option<String> {
        if self.kind == Some(InstructionType::C) {
            let re = Regex::new(r"([A-Z]*)(=?)(\w\W\d|\w)?(;?)(\w\w\w)?").unwrap();
            let caps = re.captures(self.text);

            println!("{:?}", caps);

            match caps {
                Some(x) =>  return Some(x[3].to_string()),
                None => return None
            }
        }

        return None
    }

    /// Returns the jmp in dest=comp;jmp
    pub fn jump(&self) -> Option<String> {
        if self.kind == Some(InstructionType::C) {
            let re = Regex::new(r"([A-Z]*)(=?)(\w\W\d|\w)?(;?)(\w\w\w)?").unwrap();
            let caps = re.captures(self.text).unwrap();

            println!("{:?}", caps);

            if caps[4] == ";".to_string() {
                return Some(caps[5].to_string())
            }
        }

        return None
    }
}

#[cfg(test)]
mod parser_tests {
    use super::*;

    #[test]
    fn c_instr() {
        let c1 = Instruction::new("D=D+1;JMP");
        let c2 = Instruction::new("D=A");
        let c3 = Instruction::new("DM=A");
        let c4 = Instruction::new("0;JLT");

        assert_eq!(c1.symbol(), None);

        assert_eq!(c1.dest().unwrap(), "D");
        assert_eq!(c2.dest().unwrap(), "D");
        assert_eq!(c3.dest().unwrap(), "DM");
        assert_eq!(c4.dest().unwrap(), "");

        assert_eq!(c1.comp().unwrap(), "D+1");
        assert_eq!(c2.comp().unwrap(), "A");
        assert_eq!(c3.comp().unwrap(), "A");
        assert_eq!(c4.comp().unwrap(), "0");

        assert_eq!(c1.jump().unwrap(), "JMP");
        assert_eq!(c2.jump(), None);
        assert_eq!(c3.jump(), None);
        assert_eq!(c4.jump().unwrap(), "JLT");
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