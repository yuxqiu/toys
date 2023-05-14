## BrainFuck Interpreter

A tiny brain fuck interpreter written in Rust.

## Usage

`cargo run [filename]`

- If the filename is provided, it will treat the contents of the file as a Brain Fuck program and execute it.
    - You can comment your code with any character other than the eight characters used by the Brain Fuck language.
- If no filename is provided, the interpreter will run in interactive mode, waiting for user input.
    - In this mode, each line is treated as a separate brain fuck program, which means that no state is shared between each line.

## Examples

There are a few examples in the `examples` folder:
- `7.bf`: a program that outputs 7
- `cat.bf`: a program that mimics `cat` program
- `helloworld.bf`: a program that outputs `Hello World!\n`
- `helloworld-small.bf`: a small program that outputs `Hello, World!`

## Limitations

- Only utf-8 Brain Fuck source code is supported
- Diagnostic messages are not friendly enough
- Poor performance if Brain Fuck programs rely a lot on jumps ('[' or ']')