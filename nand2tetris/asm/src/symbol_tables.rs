use std::collections::HashMap;

pub struct SymbolsTable {
    symbols_table: HashMap<String, u16>,
    next_value: u16,
}

impl SymbolsTable {
    pub fn new() -> SymbolsTable {
        SymbolsTable {
            symbols_table: HashMap::from([
                ("R0".to_string(), 0),
                ("R1".to_string(), 1),
                ("R2".to_string(), 2),
                ("R3".to_string(), 3),
                ("R4".to_string(), 4),
                ("R5".to_string(), 5),
                ("R6".to_string(), 6),
                ("R7".to_string(), 7),
                ("R8".to_string(), 8),
                ("R9".to_string(), 9),
                ("R10".to_string(), 10),
                ("R11".to_string(), 11),
                ("R12".to_string(), 12),
                ("R13".to_string(), 13),
                ("R14".to_string(), 14),
                ("R15".to_string(), 15),
                ("SP".to_string(), 0),
                ("LCL".to_string(), 1),
                ("ARG".to_string(), 2),
                ("THIS".to_string(), 3),
                ("THAT".to_string(), 4),
                ("SCREEN".to_string(), 16384),
                ("KBD".to_string(), 24576),
            ]),
            next_value: 16,
        }
    }

    pub fn contains_key(&self, k: &str) -> bool {
        self.symbols_table.contains_key(k)
    }

    pub fn get(&self, k: &str) -> Option<&u16> {
        self.symbols_table.get(k)
    }

    pub fn insert_variable(&mut self, k: String) {
        self.symbols_table.insert(k, self.next_value);
        self.next_value += 1;
    }

    pub fn insert_label(&mut self, k: String, line_number: u16) {
        self.symbols_table.insert(k, line_number);
    }
}
