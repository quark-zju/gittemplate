use crate::ast::Expr;
use lalrpop_util::lexer::Token;
use lalrpop_util::ParseError;

#[rustfmt::skip]
mod grammar;

/// Parse a string into an AST.
pub fn parse(s: &str) -> Result<Expr, ParseError<usize, Token, &str>> {
    parse_internal(s, false)
}

/// Parse a string that might be incomplete (ex. "a+", "a.").
/// `Expr::Symbol::Missing` will be used as placeholders for missing symbols.
pub fn parse_incomplete(s: &str) -> Result<Expr, ParseError<usize, Token, &str>> {
    parse_internal(s, false)
}

fn parse_internal(s: &str, ignore_errors: bool) -> Result<Expr, ParseError<usize, Token, &str>> {
    grammar::TopExprParser::new().parse(ignore_errors, s)
}
