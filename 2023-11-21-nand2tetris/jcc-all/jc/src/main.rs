use std::{
    env, fs,
    io::{BufWriter, Write},
    path::Path,
};

use anyhow::{bail, Context, Result};
use compact_str::CompactString;
use jcc::{generator::generate, parser::parse, tokenizer::tokenize};

fn main() -> Result<()> {
    let arguments: Vec<CompactString> = env::args_os()
        .map(|arg| arg.to_str().unwrap().into())
        .collect();
    assert!(
        arguments.len() == 2,
        "Usage: {} file",
        arguments.get(0).unwrap_or(&"jc".into())
    );

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
        codegen_single_file(path)?;
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
            codegen_single_file(&filepath)?;
        }
    }

    Ok(())
}

fn codegen_single_file(path: &Path) -> Result<()> {
    let tokens = tokenize(&fs::read_to_string(path)?.chars().collect::<Vec<_>>())
        .with_context(|| format!("Failed to tokenize {}", &path.to_str().unwrap()))?;
    let tree =
        parse(tokens).with_context(|| format!("Failed to tokenize {}", &path.to_str().unwrap()))?;

    let path = {
        let mut path = path.to_path_buf();
        let mut filename = path.file_stem().unwrap().to_owned();
        filename.push(".vm");
        path.set_file_name(filename);
        path
    };
    let mut writer = BufWriter::new(
        fs::File::options()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&path)?,
    );
    write!(&mut writer, "{}", generate(&tree)?.join("\n")).with_context(|| {
        format!(
            "Failed to write generated code to {}",
            path.to_str().unwrap()
        )
    })?;

    Ok(())
}
