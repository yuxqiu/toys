use anyhow::{bail, Result};
use compact_str::CompactString;
use strum_macros::{AsRefStr, Display, EnumString};

#[derive(Display, EnumString, Clone, Copy)]
#[strum(serialize_all = "lowercase")]
pub enum Keywords {
    Class,
    Constructor,
    Function,
    Method,
    Field,
    Static,
    Var,
    Int,
    Char,
    Boolean,
    Void,
    True,
    False,
    Null,
    This,
    Let,
    Do,
    If,
    While,
    Else,
    Return,
}

#[derive(EnumString, AsRefStr, Clone, Copy)]
pub enum Symbols {
    #[strum(serialize = "{")]
    LeftCurly,
    #[strum(serialize = "}")]
    RightCurly,
    #[strum(serialize = "[")]
    LeftSquare,
    #[strum(serialize = "]")]
    RightSquare,
    #[strum(serialize = "(")]
    LeftParen,
    #[strum(serialize = ")")]
    RightParen,
    #[strum(serialize = ".")]
    Dot,
    #[strum(serialize = ",")]
    Comma,
    #[strum(serialize = ";")]
    SemiColon,
    #[strum(serialize = "+")]
    Plus,
    #[strum(serialize = "-")]
    Minus,
    #[strum(serialize = "*")]
    Mult,
    #[strum(serialize = "/")]
    Div,
    #[strum(serialize = "&")]
    And,
    #[strum(serialize = "|")]
    Or,
    #[strum(serialize = "<")]
    Lt,
    #[strum(serialize = ">")]
    Gt,
    #[strum(serialize = "=")]
    Equal,
    #[strum(serialize = "~")]
    Not,
}

pub enum Token {
    Keywords(Keywords),
    Symbols(Symbols),
    Integer(u16),
    String(CompactString),
    Identifier(CompactString),
}

pub struct Line<T, S = usize> {
    pub inner: T,
    pub line: S,
}

impl<T, S> Line<T, S> {
    fn new(inner: T, line: S) -> Line<T, S> {
        Line { inner, line }
    }
}

pub struct Tokens {
    // Next Time when impl Tokens
    // 1. Preserve the source information for the token: (line, col, start_index, end_index)
    // `char_indices` might be a good function for us to gather index info.
    tokens: Vec<Line<Token>>,
    idx: usize,
}

impl Tokens {
    fn new(tokens: Vec<Line<Token>>) -> Tokens {
        Tokens { tokens, idx: 0 }
    }

    pub fn pop(&mut self) -> Option<&Line<Token>> {
        let token_with_line = self.tokens.get(self.idx);
        self.idx += 1;
        token_with_line
    }

    #[must_use]
    pub fn peek(&self, ith: usize) -> Option<&Line<Token>> {
        self.tokens.get(self.idx + ith)
    }
}

// (adv, val)
//
// Post Condition
//  1. val is in [0, 32767]
fn process_ascii_digit(chars: &[char]) -> Result<(usize, u16)> {
    let count = chars.iter().take_while(|c| c.is_ascii_digit()).count();
    if let Ok(num) = chars[..count]
        .iter()
        .collect::<CompactString>()
        .parse::<u16>()
    {
        if num <= 32767 {
            return Ok((count, num));
        }
    }

    bail!("number too large to fit in 0..32767")
}

// adv
fn process_single_line_comment(chars: &[char]) -> usize {
    chars.iter().take_while(|c| **c != '\n').count()
}

// (adv, line)
fn process_multi_line_comment(chars: &[char]) -> Result<(usize, usize)> {
    let mut line = 0;
    let count = chars[2..]
        .windows(2)
        .take_while(|w| {
            if w[0] == '\n' {
                line += 1;
            }
            w != &['*', '/']
        })
        .count();

    if !chars.get(2 + count).is_some_and(|c| *c == '*')
        || !chars.get(3 + count).is_some_and(|c| *c == '/')
    {
        bail!("unterminated comment");
    }

    Ok((4 + count, line))
}

// (adv, string literal)
fn process_strings(chars: &[char]) -> Result<(usize, CompactString)> {
    let count = chars[1..]
        .iter()
        .take_while(|c| **c != '\n' && **c != '"')
        .count();

    if !chars.get(count + 1).is_some_and(|c| *c == '"') {
        bail!("string does not end with `\"`");
    }

    Ok((2 + count, chars[1..=count].iter().collect()))
}

// (adv, identifier/keyword)
fn process_identifier(chars: &[char]) -> (usize, CompactString) {
    let count = chars
        .iter()
        .take_while(|c| c.is_ascii_alphanumeric() || **c == '_')
        .count();
    (count, chars[..count].iter().collect())
}

pub fn tokenize(chars: &[char]) -> Result<Tokens> {
    let mut tokens: Vec<Line<Token>> = Vec::new();

    // I didn't track column num because I didn't want to deal with grapheme
    let mut line = 1;
    let mut idx = 0;

    while let Some(c) = chars.get(idx) {
        if c.is_whitespace() {
            idx += 1;
            // enough to handle newlines: \r\n and \n
            // \r is rare, isn't it?
            if *c == '\n' {
                line += 1;
            }
            continue;
        }

        if c.is_ascii_digit() {
            let num = process_ascii_digit(&chars[idx..]);
            if let Err(e) = num {
                bail!("Line {}: {}", line, e);
            }
            let (adv, val) = num.unwrap();
            tokens.push(Line::new(Token::Integer(val), line));
            idx += adv;
            continue;
        }

        // comment, must before symbol parsing because we need to handle `/`
        if *c == '/' {
            if let Some(nc) = chars.get(idx + 1) {
                if *nc == '/' {
                    idx += process_single_line_comment(&chars[idx..]);
                    continue;
                } else if *nc == '*' {
                    let comment = process_multi_line_comment(&chars[idx..]);
                    if let Err(e) = comment {
                        bail!("Line {}: {}", line, e);
                    }
                    let (adv, nline) = comment.unwrap();
                    idx += adv;
                    line += nline;
                    continue;
                }
            }
        }

        // string
        if *c == '"' {
            let s = process_strings(&chars[idx..]);
            if let Err(e) = s {
                bail!("Line {}: {}", line, e);
            }
            let (adv, s) = s.unwrap();
            tokens.push(Line::new(Token::String(s), line));
            idx += adv;
            continue;
        }

        if let Ok(symbols) = {
            let mut s = [0; 4];
            TryInto::<Symbols>::try_into(c.encode_utf8(&mut s) as &str)
        } {
            tokens.push(Line::new(Token::Symbols(symbols), line));
            idx += 1;
            continue;
        }

        let (adv, identifier) = process_identifier(&chars[idx..]);
        idx += adv;

        if let Ok(keyword) = TryInto::<Keywords>::try_into(identifier.as_str()) {
            tokens.push(Line::new(Token::Keywords(keyword), line));
            continue;
        }
        tokens.push(Line::new(Token::Identifier(identifier), line));
    }

    Ok(Tokens::new(tokens))
}
