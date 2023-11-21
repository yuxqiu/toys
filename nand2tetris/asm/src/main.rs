mod instruction;
mod label;
mod parser;
mod symbol_tables;

use std::env;
use std::fs::{self};
use std::io::{BufWriter, Write};

use anyhow::{bail, Context, Result};

use parser::{parse, Kind};
use symbol_tables::SymbolsTable;

fn main() -> Result<()> {
    let arguments: Vec<String> = env::args().collect();
    if arguments.len() != 2 {
        bail!(
            "Usage: {} file",
            arguments.get(0).unwrap_or(&"asm".to_string())
        );
    }
    if !arguments[1].ends_with(".asm") {
        bail!("Input file must be .asm file. Got {} instead", arguments[1]);
    }

    let contents = fs::read_to_string(&arguments[1])
        .with_context(|| format!("Failed to read {}", arguments[1]))?;
    let instructions: Vec<Kind> = contents.lines().filter_map(parse).collect();

    let mut symbols_table = SymbolsTable::new();

    // First pass to build symbol table
    let mut line_num: u16 = 0;
    for kind in &instructions {
        match kind {
            parser::Kind::Instruction(_) => {
                line_num += 1;
            }
            parser::Kind::Label(name) => {
                let label = name.get_label();
                if symbols_table.contains_key(label) {
                    bail!(
                        "Failed to build up the symbol table because of the duplicate symbol {}",
                        label
                    );
                } else {
                    symbols_table.insert_label(label.to_string(), line_num);
                }
            }
        }
    }

    // Second pass to generate code, assume little endian
    let mut binary_code: Vec<u16> = Vec::new();

    for kind in &instructions {
        match kind {
            parser::Kind::Instruction(ins) => match ins {
                instruction::Instruction::A(a) => {
                    binary_code.push(a.resolve(&mut symbols_table));
                }
                instruction::Instruction::D(d) => {
                    binary_code.push(d.resolve());
                }
            },
            parser::Kind::Label(_) => {}
        }
    }

    let out_filename = {
        let s = arguments[1].strip_suffix(".asm").unwrap();
        String::from(s) + ".hack"
    };
    let mut writer = BufWriter::new(
        fs::File::options()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&out_filename)
            .with_context(|| format!("Failed to open {}", &out_filename))?,
    );

    binary_code.into_iter().for_each(|b| {
        writeln!(&mut writer, "{:016b}", b).unwrap();
    });

    Ok(())
}
