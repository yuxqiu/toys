use crate::{
    instruction::{Instruction, A, D},
    label::Label,
};

pub enum Kind<'a> {
    Instruction(Instruction<'a>),
    Label(Label<'a>),
}

// Assume each line is valid
pub fn parse(line: &str) -> Option<Kind> {
    // Process inline comment
    let comment_start = line.find("//").unwrap_or(line.len());
    let line = line[..comment_start].trim();

    if line.is_empty() {
        return None;
    }

    if line.starts_with('@') {
        return Some(Kind::Instruction(Instruction::A(A::from(line))));
    }

    if line.starts_with('(') && line.ends_with(')') {
        return Some(Kind::Label(Label::from(line)));
    }

    Some(Kind::Instruction(Instruction::D(D::from(line))))
}
