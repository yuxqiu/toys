use std::env;
use std::fs::File;
use std::io::Read;
use std::ops::IndexMut;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        repl();
    } else if args.len() == 2 {
        script(&args[1]);
    } else {
        println!("Usage: ./{} filename", args[0]);
    }
}

fn eval(source_code: String) {
    #[derive(Default)]
    struct Memory {
        memory: Vec<u8>,
    }
    impl Memory {
        fn index_mut(&mut self, index: usize) -> &mut u8 {
            if index >= self.memory.len() {
                self.memory
                    .resize(std::cmp::max(index + 1, self.memory.len() * 2), 0);
            }
            self.memory.index_mut(index)
        }
    }

    const TOKENS: [u8; 8] = [b'>', b'<', b'+', b'-', b'.', b',', b'[', b']'];
    let code: Vec<u8> = source_code
        .bytes()
        .filter(|ch| TOKENS.contains(ch))
        .collect();

    let mut memory: Memory = Memory::default();
    let mut pc: usize = 0;
    let mut data: usize = 0;

    while pc < code.len() {
        let ch = code[pc];
        match ch {
            b'>' => data += 1,
            b'<' => {
                if data == 0 {
                    panic!("attempted to set data pointer to value < 0");
                }
                data -= 1;
            }
            b'+' => *memory.index_mut(data) = memory.index_mut(data).wrapping_add(1),
            b'-' => *memory.index_mut(data) = memory.index_mut(data).wrapping_sub(1),
            b'.' => {
                print!(
                    "{}",
                    char::from_u32(*memory.index_mut(data) as u32).unwrap()
                );

                // In REPL mode, if the flushed contents do not contain a newline character
                // they will be hidden by rustyline.
                // So, I'm choosing not to flush here, which means that the brain fuck program
                // should output a newline character at the end if they want their output to
                // be seen immediately.
                //
                // std::io::stdout().flush().expect("unable to flush stdout");
            }
            b',' => {
                // use a temporary here because when error happens
                // the content of the buffer is unspecified
                //
                // we don't want EOF to corrupt previous value here
                let mut byte = [0];

                // if ignoring the problem mentioned above, we can use
                // slice::from_mut(memory.index_mut(data)) to treat the
                // object as a slice of length 1
                if let Err(io_error) = std::io::stdin().read_exact(&mut byte) {
                    // permit EOF when waiting for input
                    if io_error.kind() != std::io::ErrorKind::UnexpectedEof {
                        panic!("failed to read 1 byte from stdin");
                    }
                } else {
                    *memory.index_mut(data) = byte[0];
                }
            }
            b'[' => {
                if *memory.index_mut(data) == 0 {
                    // can improve this by pre-calculating (one byte at a time or SIMD) or cacheing the corresponding ]
                    let mut counter: usize = 1;
                    while counter != 0 {
                        pc += 1;
                        if pc == code.len() {
                            // can improve the error msg by recording the original position of the character
                            panic!("un-matched square bracket '[' in the source code");
                        }
                        match code[pc] {
                            b'[' => counter += 1,
                            b']' => counter -= 1,
                            _ => {}
                        }
                    }
                }
            }
            b']' => {
                if *memory.index_mut(data) != 0 {
                    // can improve this by pre-calculating (one byte at a time or SIMD) or cacheing the corresponding [
                    let mut counter: usize = 1;
                    while counter != 0 {
                        if pc == 0 {
                            // can improve the error msg by recording the original position of the character
                            panic!("un-matched square bracket ']' in the source code");
                        }
                        pc -= 1;
                        match code[pc] {
                            b'[' => counter -= 1,
                            b']' => counter += 1,
                            _ => {}
                        }
                    }
                }
            }
            _ => unreachable!(),
        }
        pc += 1;
    }
}

fn script(filename: &str) {
    let mut file = File::open(filename).unwrap();
    let mut code = String::new();
    file.read_to_string(&mut code).unwrap();
    eval(code);
}

// A simple REPL
// Each line is treated as an independent BrainFuck program
fn repl() {
    let mut rl = rustyline::DefaultEditor::new().expect("failed to start REPL mode");
    loop {
        let readline = rl.readline(">>> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str()).unwrap();
                eval(line);
            }
            Err(
                rustyline::error::ReadlineError::Eof | rustyline::error::ReadlineError::Interrupted,
            ) => break,
            e => {
                // panic if receiving other errors
                e.unwrap();
            }
        }
    }
}
