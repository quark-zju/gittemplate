use crate::ast::Expr;
use lalrpop_util::lexer::Token;
use lalrpop_util::ParseError;

#[rustfmt::skip]
mod grammar;

pub const MISSING: &'static str = "__missing__";

/// Parse a string into an AST.
pub fn parse(s: &str) -> crate::Result<Expr> {
    parse_internal(s, false)
}

/// Parse a string that might be incomplete (ex. "a+", "a.").
/// `Expr::Symbol::Missing` will be used as placeholders for missing symbols.
pub fn parse_incomplete(s: &str) -> crate::Result<Expr> {
    parse_internal(s, true)
}

fn parse_internal(s: &str, ignore_errors: bool) -> crate::Result<Expr> {
    let parser = grammar::TopExprParser::new();
    let result = parser.parse(ignore_errors, &s);

    loop {
        if result.is_ok() || !ignore_errors {
            break;
        }
        let missing = eof_missing(s, &result);
        let mut s = s.to_string();
        match missing {
            Some(m) => s.push_str(m),
            None => break,
        }

        // Try appending "__missing__", ")" at the end, and re-parse.
        // This enables parsing "a(b." as "a(b.<missing>)".
        loop {
            let result = parser.parse(ignore_errors, &s);
            if result.is_ok() {
                return result.map_err(|e| map_err(&s, e));
            } else {
                match eof_missing(&s, &result) {
                    Some(m) => s.push_str(m),
                    None => {
                        // Other error - cannot fix the expression by appending
                        // missing items.
                        break;
                    }
                }
            }
        }
    }
    result.map_err(|e| map_err(s, e))
}

fn eof_missing(
    s: &str,
    e: &std::result::Result<Expr, ParseError<usize, Token, &str>>,
) -> Option<&'static str> {
    if let Err(ParseError::UnrecognizedEOF { location, expected }) = e {
        if *location == s.len() {
            if expected.iter().find(|s| s.as_str() == "\")\"").is_some() {
                return Some(")");
            }
            return Some(MISSING);
        }
    }
    None
}

fn map_err(s: &str, e: ParseError<usize, Token, &str>) -> crate::Error {
    crate::Error::ParseError(s.to_string(), e.to_string())
}

pub(crate) fn desugar_concat(x: Expr, xs: Vec<(&str, Expr)>, func_name: &str) -> Expr {
    if xs.is_empty() {
        x
    } else {
        // x SEP y SEP z: desugar to func_name(x, y, z)
        let args: Vec<Expr> = std::iter::once(x)
            .chain(xs.into_iter().map(|(_, e)| e))
            .collect();
        Expr::Fn(func_name.to_string().into(), args)
    }
}
