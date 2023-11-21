use compact_str::{format_compact, CompactString};
use smallvec::{smallvec, SmallVec};

use crate::{parser::Kind, segment};

// Pack two CompactString together can generally guarantee we don't malloc
// even in 32-bit architecture

// Push data in D to Mem[Mem[SP]] and set Mem[SP]=Mem[SP]+1
macro_rules! push_start {
    ($($x:expr,)*) => (smallvec![$($x),*,
    "@SP\nAM=M+1".into(),
    "A=A-1\nM=D".into(),])
}

/// 1. Put data in D to R13
/// 2. Load Mem[Mem[SP]-1] to D and Set Mem[SP]-=1
/// 3. Load Mem[R13] to A
/// 4. Put D to Mem[A]
macro_rules! pop_start {
    ($($x:expr,)*) => (smallvec![$($x),*,
    "@R13\nM=D".into(),
    "@SP\nAM=M-1".into(),
    "D=M\n@R13".into(),
    "A=M\nM=D".into(),
    ])
}

pub struct HackGenerator {
    current_filename: CompactString,
    current_function: CompactString,
    counter: u16, // ROM is only 32K == 15-bit address
}

impl HackGenerator {
    pub fn new() -> HackGenerator {
        HackGenerator {
            current_filename: "".into(),
            current_function: "".into(), // top-level instruction is in an unnamed function
            counter: 0,
        }
    }

    pub fn set_filename(&mut self, s: CompactString) {
        self.current_filename = s;
    }

    pub fn bootstrap() -> &'static str {
        // Do a proper call:
        // Set SP = 256, and then push 5 useless stuff to it
        "@261
D=A
@SP
M=D
@LCL
M=D
@256
D=A
@ARG
M=D
@Sys.init
0;JMP"
    }

    pub fn generate(&mut self, k: Kind) -> SmallVec<[CompactString; 20]> {
        self.counter += 1;

        match k {
            Kind::Add => smallvec!["@SP\nAM=M-1".into(), "D=M\nA=A-1".into(), "M=D+M".into(),],
            Kind::Sub => smallvec!["@SP\nAM=M-1".into(), "D=-M\nA=A-1".into(), "M=D+M".into(),],
            Kind::Neg => smallvec!["@SP\nA=M-1".into(), "M=-M".into()],
            Kind::Eq => smallvec![
                "@SP\nAM=M-1".into(),
                "D=-M\nA=A-1".into(),
                "D=D+M".into(),
                format_compact!("@{}$EQs{}", self.current_function, self.counter),
                "D;JEQ\n@SP".into(),
                "A=M-1\nM=0".into(),
                format_compact!("@{}$EQe{}", self.current_function, self.counter),
                "0;JMP".into(),
                format_compact!("({}$EQs{})", self.current_function, self.counter),
                "@SP\nA=M-1".into(),
                "M=-1".into(),
                format_compact!("({}$EQe{})", self.current_function, self.counter),
            ],
            // haven't considered overflow yet
            Kind::Gt => smallvec![
                "@SP\nAM=M-1".into(),
                "D=-M\nA=A-1".into(),
                "D=D+M".into(),
                format_compact!("@{}$GTs{}", self.current_function, self.counter),
                "D;JGT\n@SP".into(),
                "A=M-1\nM=0".into(),
                format_compact!("@{}$GTe{}", self.current_function, self.counter),
                "0;JMP".into(),
                format_compact!("({}$GTs{})", self.current_function, self.counter),
                "@SP\nA=M-1".into(),
                "M=-1".into(),
                format_compact!("({}$GTe{})", self.current_function, self.counter),
            ],
            // haven't considered overflow yet
            Kind::Lt => smallvec![
                "@SP\nAM=M-1".into(),
                "D=-M\nA=A-1".into(),
                "D=D+M".into(),
                format_compact!("@{}$LTs{}", self.current_function, self.counter),
                "D;JLT\n@SP".into(),
                "A=M-1\nM=0".into(),
                format_compact!("@{}$LTe{}", self.current_function, self.counter),
                "0;JMP".into(),
                format_compact!("({}$LTs{})", self.current_function, self.counter),
                "@SP\nA=M-1".into(),
                "M=-1".into(),
                format_compact!("({}$LTe{})", self.current_function, self.counter),
            ],
            Kind::And => smallvec!["@SP\nAM=M-1".into(), "D=M\nA=A-1".into(), "M=D&M".into(),],
            Kind::Or => smallvec!["@SP\nAM=M-1".into(), "D=M\nA=A-1".into(), "M=D|M".into(),],
            Kind::Not => smallvec!["@SP\nA=M-1".into(), "M=!M".into()],

            Kind::Push(segment, index) => match segment {
                segment::Segment::Argument => push_start![
                    format_compact!("@{}", index),
                    "D=A\n@ARG".into(),
                    "A=D+M\nD=M".into(),
                ],
                segment::Segment::Local => push_start![
                    format_compact!("@{}", index),
                    "D=A\n@LCL".into(),
                    "A=D+M\nD=M".into(),
                ],
                segment::Segment::Static => {
                    push_start![
                        format_compact!("@{}.{}", self.current_filename, index),
                        "D=M".into(),
                    ]
                }
                segment::Segment::Constant => {
                    push_start![format_compact!("@{}", index), "D=A".into(),]
                }
                segment::Segment::This => push_start![
                    format_compact!("@{}", index),
                    "D=A\n@THIS".into(),
                    "A=D+M\nD=M".into(),
                ],
                segment::Segment::That => push_start![
                    format_compact!("@{}", index),
                    "D=A\n@THAT".into(),
                    "A=D+M\nD=M".into(),
                ],
                segment::Segment::Pointer => {
                    push_start![format_compact!("@{}", 3 + index), "D=M".into(),]
                }
                segment::Segment::Temp => {
                    push_start![format_compact!("@{}", 5 + index), "D=M".into(),]
                }
            },
            Kind::Pop(segment, index) => match segment {
                segment::Segment::Argument => {
                    pop_start![
                        format_compact!("@{}", index),
                        "D=A\n@ARG".into(),
                        "D=D+M".into(),
                    ]
                }
                segment::Segment::Local => {
                    pop_start![
                        format_compact!("@{}", index),
                        "D=A\n@LCL".into(),
                        "D=D+M".into(),
                    ]
                }
                segment::Segment::Static => smallvec![
                    "@SP\nA=M-1".into(),
                    "D=M".into(),
                    format_compact!("@{}.{}", self.current_filename, index),
                    "M=D\n@SP".into(),
                    "M=M-1".into(),
                ],
                segment::Segment::This => {
                    pop_start![
                        format_compact!("@{}", index),
                        "D=A\n@THIS".into(),
                        "D=D+M".into(),
                    ]
                }
                segment::Segment::That => {
                    pop_start![
                        format_compact!("@{}", index),
                        "D=A\n@THAT".into(),
                        "D=D+M".into(),
                    ]
                }
                segment::Segment::Pointer => smallvec![
                    "@SP\nAM=M-1".into(),
                    "D=M".into(),
                    format_compact!("@{}", 3 + index),
                    "M=D".into(),
                ],
                segment::Segment::Temp => smallvec![
                    "@SP\nAM=M-1".into(),
                    "D=M".into(),
                    format_compact!("@{}", 5 + index),
                    "M=D".into(),
                ],
                _ => unreachable!(),
            },

            Kind::Label(label) => {
                smallvec![format_compact!("({}.{})", self.current_function, label)]
            }
            Kind::Goto(label) => smallvec![
                format_compact!("@{}.{}", self.current_function, label),
                "0;JMP".into()
            ],
            Kind::IfGoto(label) => smallvec![
                "@SP\nAM=M-1".into(),
                "D=M".into(),
                format_compact!("@{}.{}", self.current_function, label),
                "D;JNE".into()
            ],

            // Can create special cases for other small nlcls
            Kind::Function(func, nlcls) => {
                self.current_function = func;

                if nlcls == 0 {
                    return smallvec![format_compact!("({})", self.current_function),];
                }

                smallvec![
                    format_compact!("({})", self.current_function),
                    format_compact!("@{}", nlcls - 1),
                    "D=A".into(),
                    format_compact!("({}$init)", self.current_function),
                    "@SP\nAM=M+1".into(),
                    "A=A-1\nM=0".into(),
                    "D=D-1".into(),
                    format_compact!("@{}$init", self.current_function),
                    "D;JGE".into(),
                ]
            }
            Kind::Call(func, nargs) => {
                // Push return_addr, LCL, ARG, THIS, THAT
                smallvec![
                    format_compact!("@{}$ret{}", self.current_function, self.counter),
                    "D=A\n@SP".into(),
                    "A=M\nM=D".into(),
                    "@LCL\nD=M".into(),
                    "@SP\nAM=M+1".into(),
                    "M=D\n@ARG".into(),
                    "D=M\n@SP".into(),
                    "AM=M+1\nM=D".into(),
                    "@THIS\nD=M".into(),
                    "@SP\nAM=M+1".into(),
                    "M=D\n@THAT".into(),
                    "D=M\n@SP".into(),
                    "AM=M+1\nM=D".into(),
                    "@SP\nMD=M+1".into(), // must use MD=M+1 bc of a silly bug in CPUSimulator
                    // === set LCL first
                    "@LCL\nM=D".into(),
                    // === set ARG
                    format_compact!("@{}\nD=D-A", 5 + nargs), // maxlen = 12
                    "@ARG\nM=D".into(),
                    format_compact!("@{}", func),
                    "0;JMP".into(),
                    format_compact!("({}$ret{})", self.current_function, self.counter),
                ]
            }
            Kind::Return => {
                smallvec![
                    "@5\nD=-A".into(),
                    "@LCL\nA=D+M".into(),
                    "D=M\n@R13".into(),
                    // save return_addr to Mem[R13]
                    // must be done here because *ARG = pop() might overwrite
                    // return_address if the arguments of the function is 0
                    "M=D\n@SP".into(),
                    "A=M-1\nD=M".into(),
                    "@ARG\nA=M".into(),
                    "M=D\nD=A".into(),   // set *ARG = pop()
                    "@SP\nM=D+1".into(), // set SP = ARG + 1
                    "@LCL\nAM=M-1".into(),
                    "D=M\n@THAT".into(),
                    "M=D\n@LCL".into(), // set THAT
                    "AM=M-1\nD=M".into(),
                    "@THIS\nM=D".into(), // set THIS
                    "@LCL\nAM=M-1".into(),
                    "D=M\n@ARG".into(),
                    "M=D\n@LCL".into(), // set ARG
                    "A=M-1\nD=M".into(),
                    "@LCL\nM=D".into(), // set LCL
                    "@R13\nA=M".into(),
                    "0;JMP".into(), // ret
                ]
            }
        }
    }
}
