use crate::symbol_tables::SymbolsTable;

pub struct A<'a>(&'a str);
pub struct D<'a>(&'a str);

pub enum Instruction<'a> {
    A(A<'a>),
    D(D<'a>),
}

impl A<'_> {
    pub fn from(s: &str) -> A {
        A(s)
    }

    pub fn resolve(&self, symbols_table: &mut SymbolsTable) -> u16 {
        let value = &self.0[1..];

        if let Ok(num) = value.parse::<u16>() {
            if num <= 32767 {
                return num;
            }
            unreachable!();
        }

        // Assume valid symbol now
        // Symbols: A symbol can be any sequence of letters, digits,
        // underscore (_), dot (.), dollar sign ($), and colon (:) that does not begin with a digit.

        // Refer to Label
        if let Some(num) = symbols_table.get(value) {
            if *num <= 32767 {
                return *num;
            }
            unreachable!();
        }

        // Refer to Variable
        symbols_table.insert_variable(value.to_string());
        let num = symbols_table.get(value).unwrap();
        if *num <= 32767 {
            return *num;
        }
        unreachable!();
    }
}

impl D<'_> {
    pub fn from(s: &str) -> D {
        D(s)
    }

    // Assume valid D-instruction
    pub fn resolve(&self) -> u16 {
        let mut d: u16 = 0;
        let mut j: u16 = 0;

        let remaining = if let Some((dest, other)) = self.0.split_once('=') {
            let dest = dest.trim_end();
            if dest.contains('M') {
                d |= 0b001;
            }
            if dest.contains('D') {
                d |= 0b010;
            }
            if dest.contains('A') {
                d |= 0b100;
            }
            other.trim_start()
        } else {
            self.0
        };

        let comp = if let Some((comp, jump)) = remaining.split_once(';') {
            j = match jump.trim_start() {
                "JGT" => 0b001,
                "JEQ" => 0b010,
                "JGE" => 0b011,
                "JLT" => 0b100,
                "JNE" => 0b101,
                "JLE" => 0b110,
                "JMP" => 0b111,
                _ => unreachable!(),
            };

            comp.trim_end()
        } else {
            remaining
        };

        let c: u16 = match comp {
            "0" => 0b0101010,
            "1" => 0b0111111,
            "-1" => 0b0111010,
            "D" => 0b0001100,
            "A" => 0b0110000,
            "M" => 0b1110000,
            "!D" => 0b0001101,
            "!A" => 0b0110001,
            "!M" => 0b1110001,
            "-D" => 0b0001111,
            "-A" => 0b0110011,
            "-M" => 0b1110011,
            "D+1" => 0b0011111,
            "A+1" => 0b0110111,
            "M+1" => 0b1110111,
            "D-1" => 0b0001110,
            "A-1" => 0b0110010,
            "M-1" => 0b1110010,
            "D+A" => 0b0000010,
            "D+M" => 0b1000010,
            "D-A" => 0b0010011,
            "D-M" => 0b1010011,
            "A-D" => 0b0000111,
            "M-D" => 0b1000111,
            "D&A" => 0b0000000,
            "D&M" => 0b1000000,
            "D|A" => 0b0010101,
            "D|M" => 0b1010101,
            _ => unreachable!(),
        };

        (0b1110000000000000) | (c << 6 | (d << 3) | j)
    }
}
