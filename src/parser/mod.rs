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
    parse_internal(s, true)
}

fn parse_internal(s: &str, ignore_errors: bool) -> Result<Expr, ParseError<usize, Token, &str>> {
    grammar::TopExprParser::new().parse(ignore_errors, s)
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
