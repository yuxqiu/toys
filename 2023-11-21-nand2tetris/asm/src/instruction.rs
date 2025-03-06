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
            "0" => 0b010_1010,
            "1" => 0b011_1111,
            "-1" => 0b011_1010,
            "D" => 0b000_1100,
            "A" => 0b011_0000,
            "M" => 0b111_0000,
            "!D" => 0b000_1101,
            "!A" => 0b011_0001,
            "!M" => 0b111_0001,
            "-D" => 0b000_1111,
            "-A" => 0b011_0011,
            "-M" => 0b111_0011,
            "D+1" => 0b001_1111,
            "A+1" => 0b011_0111,
            "M+1" => 0b111_0111,
            "D-1" => 0b000_1110,
            "A-1" => 0b011_0010,
            "M-1" => 0b111_0010,
            "D+A" => 0b000_0010,
            "D+M" => 0b100_0010,
            "D-A" => 0b001_0011,
            "D-M" => 0b101_0011,
            "A-D" => 0b000_0111,
            "M-D" => 0b100_0111,
            "D&A" => 0b000_0000,
            "D&M" => 0b100_0000,
            "D|A" => 0b001_0101,
            "D|M" => 0b101_0101,
            _ => unreachable!(),
        };

        (0b1110_0000_0000_0000) | (c << 6 | (d << 3) | j)
    }
}
