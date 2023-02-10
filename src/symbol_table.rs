use dict::{ Dict, DictIface};
use crate::*;

pub struct SymbolTable {
    table: Dict::<String>
}

impl SymbolTable {
    /// Creates a new table
    pub fn new() -> SymbolTable {
        let mut table = SymbolTable { table: Dict::<String>::new() };

        table.add_entry("SP", parser::byte_creator("0"));
        table.add_entry("LCL", parser::byte_creator("1"));
        table.add_entry("ARG", parser::byte_creator("2"));
        table.add_entry("THIS", parser::byte_creator("3"));
        table.add_entry("THAT", parser::byte_creator("4"));
        table.add_entry("R0", parser::byte_creator("0"));
        table.add_entry("R1", parser::byte_creator("1"));
        table.add_entry("R2", parser::byte_creator("2"));
        table.add_entry("R3", parser::byte_creator("3"));
        table.add_entry("R4", parser::byte_creator("4"));
        table.add_entry("R5", parser::byte_creator("5"));
        table.add_entry("R6", parser::byte_creator("6"));
        table.add_entry("R7", parser::byte_creator("7"));
        table.add_entry("R8", parser::byte_creator("8"));
        table.add_entry("R9", parser::byte_creator("9"));
        table.add_entry("R10", parser::byte_creator("10"));
        table.add_entry("R11", parser::byte_creator("11"));
        table.add_entry("R12", parser::byte_creator("12"));
        table.add_entry("R13", parser::byte_creator("13"));
        table.add_entry("R14", parser::byte_creator("14"));
        table.add_entry("R15", parser::byte_creator("15"));
        table.add_entry("SCREEN", parser::byte_creator("16384"));
        table.add_entry("KBD", parser::byte_creator("24576"));

        return table
    }

    /// Adds the given symbol and address to the table
    pub fn add_entry(&mut self, symbol: &str, address: String) {
        self.table.add(symbol.to_string(), address);
    }

    /// Returns if the table contains the symbol
    pub fn contains(&mut self, symbol: &str) -> bool {
        let table_response = self.table.get(symbol);

        match table_response {
            Some(_string) => return true,
            None => return false
        }
    }

    /// Returns the address for the symbol if it exists, otherwise returns none
    pub fn get_address(&mut self, symbol: &str) -> Option<String> {
        let table_response = self.table.get(symbol);

        match table_response {
            Some(x) => return Some(x.to_string()),
            None => return None
        }
    }
}

pub fn init_table(lines: &Vec<&str>, symbol_table: &mut SymbolTable) {
    init_progress_bar(lines.len());
    set_progress_bar_action("Symbol Table", Color::Blue, Style::Bold);

    let mut constant: i64 = 0;

    for line in lines {
        inc_progress_bar();
        let mut split: String = line.to_string();

        if line.find("//") != None {
            let split_line: Vec<&str> = line.split("//").collect();
            split = split_line[0].to_string();
        }

        // Removes white space
        let new_line = &remove_white_space(&split);

        // Removes white space
        if !new_line.is_empty() {
            let instr = parser::Instruction::new(&new_line);

            if instr.kind == Some(parser::InstructionType::L) {
                if !symbol_table.contains(&instr.symbol().unwrap()) {
                    let address =  &parser::byte_creator(&constant.to_string());
                    symbol_table.add_entry(&instr.symbol().unwrap(), address.to_string());
                }
            }
            else {
                constant += 1;
            }
        }
    }

    print_progress_bar_info("Success", "Created symbol table", Color::Green, Style::Normal);
    finalize_progress_bar();
}

#[cfg(test)]
mod symbol_table_tests {
    use crate::symbol_table::SymbolTable;

    //MARK: Symbol table tests
    #[test]
    fn entry_added() {
        let mut table = SymbolTable::new();
        table.add_entry("TRUE", 1);

        assert_eq!(table.contains("TRUE"), true);
        assert_eq!(table.contains("FALSE"), false);
    }

    #[test]
    fn gets_address() {
        let mut table = SymbolTable::new();
        table.add_entry("TRUE", 1);

        assert_eq!(table.get_address("TRUE").unwrap(), 1);
        assert_eq!(table.get_address("FALSE"), None);
    }
}