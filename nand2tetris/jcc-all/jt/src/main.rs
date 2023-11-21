use std::{env, fs, io::BufWriter, path::Path};

use anyhow::{bail, Context, Ok, Result};
use compact_str::CompactString;
use jcc::tokenizer::tokenize;
use xml::to_xml;

mod xml;

fn main() -> Result<()> {
    let arguments: Vec<CompactString> = env::args_os()
        .map(|arg| arg.to_str().unwrap().into())
        .collect();
    if arguments.len() != 2 {
        panic!("Usage: {} file", arguments.get(0).unwrap_or(&"jc".into()));
    }

    let is_single_file = arguments[1].ends_with(".jack");
    let path = Path::new(&arguments[1]);
    if !is_single_file && !path.is_dir() {
        bail!(
            "Input file must be .jack file or a directory. Got {} instead",
            arguments[1]
        );
    }

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
                bail!(
                    "Input filename must start with an uppercase character. Got {} instead",
                    arguments[1]
                );
            }
        } else {
            bail!("Invalid characters in given file path");
        }
    }

    if is_single_file {
        xml_single_file(path)?;
    } else {
        for filepath in fs::read_dir(path)
            .unwrap()
            .map(Result::unwrap)
            .map(|entry| entry.path())
            .filter(|path| {
                !path.is_dir()
                    && path.extension().is_some_and(|s| s == "jack")
                    && path.file_name().is_some_and(|s| {
                        s.to_str()
                            .is_some_and(|s| s.chars().next().unwrap().is_ascii_uppercase())
                    })
            })
        {
            xml_single_file(&filepath)?;
        }
    }

    Ok(())
}

fn xml_single_file(path: &Path) -> Result<()> {
    let mut tokens = tokenize(fs::read_to_string(path)?.chars().collect())
        .with_context(|| format!("Failed to tokenize {}", &path.to_str().unwrap()))?;
    let path = {
        let mut path = path.to_path_buf();
        let mut filename = path.file_stem().unwrap().to_owned();
        filename.push("TT.xml");
        path.set_file_name(filename);
        path
    };
    let mut writer = BufWriter::new(
        fs::File::options()
            .write(true)
            .truncate(true)
            .create(true)
            .open(path)?,
    );
    to_xml(&mut writer, &mut tokens);

    Ok(())
}
