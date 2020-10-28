use crate::objects::protocol::IntoObject;
use crate::objects::protocol::Object;
use crate::Error;
use crate::Result;
use std::borrow::Cow;
use std::fmt;

/// A node in the parsed AST.
#[derive(Clone)]
pub enum Expr {
    /// Unresolved symbol name.
    Symbol(Symbol),

    /// A function call.
    Fn(Symbol, Vec<Expr>),

    /// Pre-calculated value.
    Inlined(Object),
}

/// Symbol name.
#[derive(Clone)]
pub enum Symbol {
    /// Specified name.
    Name(Cow<'static, str>),

    /// Placeholder.
    /// Useful for parsing incomplete input.
    Missing,
}

impl From<()> for Symbol {
    fn from(_: ()) -> Self {
        Self::Missing
    }
}

impl From<&'static str> for Symbol {
    fn from(s: &'static str) -> Self {
        Self::Name(s.into())
    }
}

impl From<String> for Symbol {
    fn from(s: String) -> Self {
        Self::Name(s.into())
    }
}

impl Symbol {
    pub fn as_str(&self) -> &str {
        match self {
            Symbol::Name(ref s) => s.as_ref(),
            Symbol::Missing => &"<missing>",
        }
    }
}

impl fmt::Debug for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Symbol(s) => f.write_str(s.as_str())?,
            Expr::Fn(name, args) => {
                let name = name.as_str();
                if args.is_empty() {
                    f.write_str(name)?;
                    f.write_str("()")?;
                } else {
                    let mut list = f.debug_tuple(&name);
                    for arg in args {
                        list.field(arg);
                    }
                    list.finish()?;
                }
            }
            Expr::Inlined(value) => {
                let s = value.to_ast_fmt_string();
                f.write_str(&s)?;
            }
        }
        Ok(())
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

impl Expr {
    /// Parse AST from a string.
    pub fn parse(s: &str) -> Result<Self> {
        crate::parser::parse(s).map_err(|e| Error::ParseError(s.to_string(), e.to_string()))
    }

    /// Parse AST from a potentially incomplete string.
    /// Evaluating the AST might cause `MissingSymbol` errors.
    pub fn parse_incomplete(s: &str) -> Result<Self> {
        crate::parser::parse_incomplete(s)
            .map_err(|e| Error::ParseError(s.to_string(), e.to_string()))
    }

    /// Replace a name (ex. `x`) to another name.
    /// Useful to implement lambda.
    pub(crate) fn replace(&mut self, from: &str, to: &Expr) {
        match self {
            Expr::Inlined(_) => {}
            Expr::Symbol(s) => {
                if s.as_str() == from {
                    *self = to.clone();
                }
            }
            Expr::Fn(name, args) => {
                // Special case: hold the first argument of "lambda" unchanged.
                if name.as_str() == "lambda" && args.len() > 1 {
                    if let Expr::Symbol(ref inner_name) = args[0] {
                        if inner_name.as_str() != from {
                            for arg in &mut args[1..] {
                                arg.replace(from, to);
                            }
                        }
                    }
                } else {
                    for arg in args {
                        arg.replace(from, to);
                    }
                }
            }
        }
    }

    pub(crate) fn into_gitrevset_expr(self) -> Result<gitrevset::Expr> {
        type RevsetExpr = gitrevset::Expr;
        let expr = match self {
            Expr::Symbol(Symbol::Name(s)) => RevsetExpr::Name(s.to_string()),
            Expr::Fn(Symbol::Name(name), args) => {
                let args = args
                    .into_iter()
                    .map(|a| a.into_gitrevset_expr())
                    .collect::<Result<Vec<_>>>()?;
                RevsetExpr::Fn(name, args)
            }
            // Untranslatable.
            _ => RevsetExpr::Name("_unknown_".into()),
        };
        Ok(expr)
    }
}

/// Convert to `Expr` by parsing.
pub trait ParseToExpr {
    /// Convert to `Expr` by parsing.
    fn parse_to_expr(self) -> Result<Expr>;
}

impl ParseToExpr for &str {
    fn parse_to_expr(self) -> Result<Expr> {
        Expr::parse(self)
    }
}

impl ParseToExpr for Expr {
    fn parse_to_expr(self) -> Result<Expr> {
        Ok(self)
    }
}

// unquote("'foo\\n'") => "foo\n"
pub(crate) fn unquote(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut prev = '_';
    for ch in s[1..s.len() - 1].chars() {
        match (prev, ch) {
            ('\\', 'n') => result.push('\n'),
            ('\\', 't') => result.push('\t'),
            ('\\', _) => result.push(ch),
            (_, '\\') => (),
            (_, _) => result.push(ch),
        }
        prev = ch;
    }
    result
}

impl From<String> for Expr {
    fn from(s: String) -> Expr {
        Expr::Inlined(s.into_object())
    }
}

impl From<&str> for Expr {
    fn from(s: &str) -> Expr {
        Expr::Inlined(s.to_string().into_object())
    }
}

impl From<Object> for Expr {
    fn from(o: Object) -> Expr {
        Expr::Inlined(o)
    }
}

impl From<u64> for Expr {
    fn from(i: u64) -> Expr {
        Expr::Symbol(i.to_string().into())
    }
}

/// Statically construct [`Expr`].
#[macro_export]
macro_rules! ast {
    ($lit:literal) => { $crate::Expr::from($lit) };
    ($fn_name:ident ( $($arg:tt $( ( $($subargs:tt)* ) )? ),* )) => {{
        let args = vec![ $(ast!($arg $( ( $($subargs)* ) )?),)* ];
        $crate::Expr::Fn(stringify!($fn_name).into(), args)
    }};
    ($sym:ident) => { $crate::Expr::Symbol(stringify!($sym).to_string()) };
    {$v:expr} => { $crate::Expr::from($v) };
}
