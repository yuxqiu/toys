use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;

use compact_str::CompactString;
use generator::HackGenerator;
use parser::parse;
use parser::Kind;

mod generator;
mod parser;
mod segment;

fn main() {
    let arguments: Vec<CompactString> = env::args_os()
        .map(|arg| arg.to_str().unwrap().into())
        .collect();
    assert!(
        arguments.len() == 2,
        "Usage: {} file",
        arguments.get(0).unwrap_or(&"hvm".into())
    );

    let is_single_file = arguments[1].ends_with(".vm");
    let path = Path::new(&arguments[1]);
    assert!(
        !(!is_single_file && !path.is_dir()),
        "Input file must be .vm file or a directory. Got {} instead",
        arguments[1]
    );

    if is_single_file {
        if let Some(c) = path.file_name() {
            if !c
                .to_str()
                .unwrap()
                .chars()
                .next()
                .unwrap()
                .is_ascii_uppercase()
            {
                panic!(
                    "Input filename must start with an uppercase character. Got {} instead",
                    arguments[1]
                );
            }
        } else {
            panic!("Invalid input file path");
        }
    }

    let instructions: HashMap<CompactString, Vec<Kind>> = {
        if is_single_file {
            let contents = fs::read_to_string(path).unwrap();
            HashMap::from([(
                path.file_stem().unwrap().to_str().unwrap().into(),
                contents.lines().filter_map(parse).collect(),
            )])
        } else {
            fs::read_dir(path)
                .unwrap()
                .map(Result::unwrap)
                .map(|entry| entry.path())
                .filter(|path| !path.is_dir() && path.extension().is_some_and(|s| s == "vm"))
                .map(|filepath| {
                    let filename = filepath.file_stem().unwrap().to_str().unwrap();
                    assert!(
                        filename.chars().next().unwrap().is_ascii_uppercase(),
                        "Input filename must start with an uppercase character. Got {} instead",
                        arguments[1]
                    );
                    let contents = fs::read_to_string(&filepath).unwrap();
                    (
                        filename.into(),
                        contents.lines().filter_map(parse).collect::<Vec<Kind>>(),
                    )
                })
                .collect()
        }
    };

    let path = {
        let mut pathbuf = path.to_path_buf();
        if !is_single_file {
            pathbuf.push(path.file_name().unwrap());
        }
        pathbuf.set_extension("asm");
        pathbuf
    };

    let mut writer = BufWriter::new(
        fs::File::options()
            .write(true)
            .truncate(true)
            .create(true)
            .open(path)
            .unwrap(),
    );

    let mut generator = HackGenerator::new();
    writeln!(&mut writer, "{}", HackGenerator::bootstrap()).unwrap();
    for (filename, instructions) in instructions {
        generator.set_filename(filename);
        for ins in instructions {
            writeln!(&mut writer, "// {ins}").unwrap();
            writeln!(&mut writer, "{}", generator.generate(ins).join("\n")).unwrap();
        }
    }
}
