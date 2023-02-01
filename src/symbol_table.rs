use dict::{ Dict, DictIface};

pub struct SymbolTable {
    table: Dict::<u8>
}

impl SymbolTable {
    /// Creates a new table
    pub fn new() -> SymbolTable {
        return SymbolTable {
            table: Dict::<u8>::new()
        }
    }

    /// Adds the given symbol and address to the table
    pub fn add_entry(&mut self, symbol: &str, address: u8) {
        self.table.add(symbol.to_string(), address);
    }

    /// Returns if the table contains the symbol
    pub fn contains(&mut self, symbol: &str) -> bool {
        let table_response = self.table.get(symbol);

        match table_response {
            Some(&_u8) => return true,
            None => return false
        }
    }

    /// Returns the address for the symbol if it exists, otherwise returns none
    pub fn get_address(&mut self, symbol: &str) -> Option<u8> {
        let table_response = self.table.get(symbol);

        match table_response {
            Some(x) => return Some(*x),
            None => return None
        }
    }
}