## nand2tetris

- `asm` contains the implementation of the nand2tetris assembler.
- `hvm` contains the implementation of the nand2tetris virtual machine.
- `jcc-all` contains
    - `jt`: contains a tokenizer that outputs tokens in xml format.
    - `jc`: contains a compiler that compiles Jack lang to instructions supported by the VM.
    - `jcc`: contains shared libraries used by `jt` and `jc`, including tokenizer, parser and code generator.