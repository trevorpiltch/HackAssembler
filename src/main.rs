pub mod file;
pub mod symbol_table;

fn main() {
    let mut table = symbol_table::SymbolTable::new();
    table.add_entry("john", 1);

    println!("Does contain: {}", table.contains("john"));
    if table.contains("john") {
        println!("Address: {:?}", table.get_address("john").unwrap());
    }
}

//MARK: Tests
#[cfg(test)]
mod tests {
    use super::*;

    //MARK: File handling tests
    #[test]
    fn create_file() {
        file::create_file("tmp_test.txt", String::from("This is just a temporary file :)"));

        assert_eq!(file::get_file_test("tmp_test.txt"), String::from("This is just a temporary file :)"));
    }

    #[test]
    #[should_panic(expected = "Must be a file in the current directory.")]
    fn file_not_foun() {
        file::create_file("tmp_test.txt", String::from("This is just a temporary file :)"));
        file::get_file_test("temp_test.txt");
    }

    //MARK: Symbol table tests
    #[test]
    fn entry_added() {
        let mut table = symbol_table::SymbolTable::new();
        table.add_entry("TRUE", 1);

        assert_eq!(table.contains("TRUE"), true);
        assert_eq!(table.contains("FALSE"), false);
    }

    #[test]
    fn gets_address() {
        let mut table = symbol_table::SymbolTable::new();
        table.add_entry("TRUE", 1);

        assert_eq!(table.get_address("TRUE").unwrap(), 1);
        assert_eq!(table.get_address("FALSE"), None);
    }
}