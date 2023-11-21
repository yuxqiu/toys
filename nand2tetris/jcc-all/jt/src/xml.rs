use std::io::Write;

use compact_str::CompactString;
use jcc::tokenizer::Tokens;

fn escape_symbol(s: &str) -> &str {
    match s {
        "<" => "&lt;",
        ">" => "&gt;",
        "\"" => "&quot;",
        "&" => "&amp;",
        _ => s,
    }
}

fn escape_string(s: &CompactString) -> String {
    // Hope this is efficient
    s.replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('\"', "&quot;")
        .replace('&', "&amp;")
}

pub fn to_xml<W>(writer: &mut W, tokens: &mut Tokens)
where
    W: Write,
{
    writeln!(writer, "<tokens>").unwrap();

    while let Some(token) = tokens.pop().map(|t| &t.inner) {
        match token {
            jcc::tokenizer::Token::Keywords(keyword) => {
                writeln!(writer, "<keyword> {} </keyword>", keyword).unwrap();
            }
            jcc::tokenizer::Token::Symbols(symbol) => {
                writeln!(
                    writer,
                    "<symbol> {} </symbol>",
                    escape_symbol(symbol.as_ref())
                )
                .unwrap();
            }
            jcc::tokenizer::Token::Integer(integer) => {
                writeln!(writer, "<integerConstant> {} </integerConstant>", integer).unwrap();
            }
            jcc::tokenizer::Token::String(string) => {
                writeln!(
                    writer,
                    "<stringConstant> {} </stringConstant>",
                    escape_string(string)
                )
                .unwrap();
            }
            jcc::tokenizer::Token::Identifier(identifier) => {
                writeln!(writer, "<identifier> {} </identifier>", identifier).unwrap();
            }
        }
    }

    writeln!(writer, "</tokens>").unwrap();
}
