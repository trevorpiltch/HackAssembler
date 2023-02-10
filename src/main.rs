use progress_bar::*;

pub mod file;
pub mod symbol_table;
pub mod translator;
pub mod parser;

fn main() {
    let content = file::get_file();
    let name = file::get_name();

    let lines = file::parse_lines(&content);

    let mut content = "".to_string();

    let mut symbol_table = symbol_table::SymbolTable::new();

    symbol_table::init_table(&lines, &mut symbol_table);

    let mut variable_address = 16;

    init_progress_bar(lines.len());
    set_progress_bar_action("Assembling", Color::Blue, Style::Bold);

    for mut line in lines {
        inc_progress_bar();

        // Remove comments
        if line.find("//") != None {
            let split_line: Vec<&str> = line.split("//").collect();
            line = split_line[0];
        }

        // Removes white space
        let new_line = &remove_white_space(&line);

        // Creates binary
        if !&new_line.is_empty() {
            let instr = parser::Instruction::new(&new_line);

            if instr.kind == Some(parser::InstructionType::C) {
                content = format!("{}{}\n",content, instr.get_c_code());
            }
            else if instr.kind == Some(parser::InstructionType::A) {
                let symbol = instr.symbol().unwrap();

                let num = match symbol.parse::<i32>() {
                    Ok(num) => format!("{:016b}", num),
                    Err(_) => {
                        if symbol_table.contains(&symbol) {
                            symbol_table.get_address(&symbol).unwrap()
                        }
                        else {
                            symbol_table.add_entry(&symbol, parser::byte_creator(&variable_address.to_string()));
                            let current_address = variable_address;
                            variable_address += 1;
                            format!("{:016b}", current_address)
                        }
                    }
                };

                content = format!("{}{}\n", content, num);
            }

        }
    }

    print_progress_bar_info("Success", "Created .hack file", Color::Green, Style::Normal);
    finalize_progress_bar();

    let file_name = format!("{}.hack", name);

    let _file_result = file::create_file(&file_name, content);
}

pub fn remove_white_space(line: &str) -> String {
    line.chars().filter(|c| !c.is_whitespace()).collect()
}