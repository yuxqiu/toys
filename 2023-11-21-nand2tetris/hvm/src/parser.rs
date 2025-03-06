use std::str::FromStr;

use compact_str::CompactString;
use strum_macros::{EnumString, IntoStaticStr};

use crate::segment::Segment;

#[derive(EnumString, IntoStaticStr)]
pub enum Kind {
    #[strum(serialize = "add")]
    Add,
    #[strum(serialize = "sub")]
    Sub,
    #[strum(serialize = "neg")]
    Neg,
    #[strum(serialize = "eq")]
    Eq,
    #[strum(serialize = "gt")]
    Gt,
    #[strum(serialize = "lt")]
    Lt,
    #[strum(serialize = "and")]
    And,
    #[strum(serialize = "or")]
    Or,
    #[strum(serialize = "not")]
    Not,

    #[strum(disabled)]
    Push(Segment, u16),
    #[strum(disabled)]
    Pop(Segment, u16),

    #[strum(disabled)]
    Label(CompactString),
    #[strum(disabled)]
    Goto(CompactString),
    #[strum(disabled)]
    IfGoto(CompactString),

    #[strum(disabled)]
    Function(CompactString, u16),
    #[strum(disabled)]
    Call(CompactString, u16),
    #[strum(serialize = "return")]
    Return,
}

impl std::fmt::Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Kind::Push(segment, index) => write!(f, "push {segment} {index}"),
            Kind::Pop(segment, index) => write!(f, "pop {segment} {index}"),

            Kind::Label(label) => write!(f, "label {label}"),
            Kind::Goto(label) => write!(f, "goto {label}"),
            Kind::IfGoto(label) => write!(f, "if-goto {label}"),

            Kind::Function(func, nlcls) => write!(f, "function {func} {nlcls}"),
            Kind::Call(func, nargs) => write!(f, "call {func} {nargs}"),
            Kind::Return => write!(f, "return"),

            kind => <&Kind as Into<&'static str>>::into(kind).fmt(f),
        }
    }
}

// Assume line is a valid VM program
//
// No whitespace: logic, arithmetic, return
// 1 whitespace: label, goto, if-goto
// 2 whitespaces: push, pop, function, call
pub fn parse(line: &str) -> Option<Kind> {
    let comment_start = line.find("//").unwrap_or(line.len());
    let line = line[..comment_start].trim();
    if line.is_empty() {
        return None;
    }

    // check 1/2 whitespaces
    if let Some((op, other)) = line.split_once(' ') {
        let other = other.trim_start();

        // 2 whitespaces
        if let Some((left, right)) = other.split_once(' ') {
            let right = right.trim_start();
            return Some(match op {
                "push" | "pop" => {
                    // always assume this is a valid index after parsing
                    // let assembler handle this error
                    let index = right.parse::<u16>().unwrap_or_else(|_| {
                        panic!("Invalid index for push/pop instruction. Got {line}")
                    });

                    match op {
                        "push" => Kind::Push(parse_segment(left), index),
                        "pop" => Kind::Pop(parse_segment(left), index),
                        _ => unreachable!(),
                    }
                }
                "function" => {
                    let nlcls = right.parse::<u16>().unwrap_or_else(|_| {
                        panic!(
                            "Invalid number of local variables for function instruction. Got {line}"
                        )
                    });
                    Kind::Function(left.into(), nlcls)
                }
                "call" => {
                    let nargs = right.parse::<u16>().unwrap_or_else(|_| {
                        panic!("Invalid number of arguments for call instruction. Got {line}")
                    });
                    Kind::Call(left.into(), nargs)
                }
                _ => unreachable!(),
            });
        }

        // 1 whitespace
        return Some(match op {
            "label" => Kind::Label(other.into()),
            "goto" => Kind::Goto(other.into()),
            "if-goto" => Kind::IfGoto(other.into()),
            _ => unreachable!(),
        });
    }

    // arithmetic, return
    Kind::from_str(line).ok()
}

fn parse_segment(segment: &str) -> Segment {
    Segment::from_str(segment).expect("Failed to parse segment")
}
