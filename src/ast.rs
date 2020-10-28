use crate::objects::protocol::IntoObject;
use crate::objects::protocol::Object;
use crate::Error;
use crate::Result;
use serde::ser::Error as _;
use serde_json::json;
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
pub type Symbol = Cow<'static, str>;

impl fmt::Debug for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Symbol(s) => f.write_str(s.as_ref())?,
            Expr::Fn(name, args) => {
                let name = name.as_ref();
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
                if s.as_ref() == from {
                    *self = to.clone();
                }
            }
            Expr::Fn(name, args) => {
                // Special case: hold the first argument of "lambda" unchanged.
                if name.as_ref() == "lambda" && args.len() > 1 {
                    if let Expr::Symbol(ref inner_name) = args[0] {
                        if inner_name.as_ref() != from {
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
            Expr::Symbol(s) => RevsetExpr::Name(s.to_string()),
            Expr::Fn(name, args) => {
                let args = args
                    .into_iter()
                    .map(|a| a.into_gitrevset_expr())
                    .collect::<Result<Vec<_>>>()?;
                RevsetExpr::Fn(name, args)
            }
            Expr::Inlined(s) => RevsetExpr::Name(s.to_plain_string()?.into()),
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

impl From<i64> for Expr {
    fn from(i: i64) -> Expr {
        Expr::Inlined(i.into_object())
    }
}

impl serde::Serialize for Expr {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Expr::Inlined(object) => {
                let serde_value = object.to_serde_value().map_err(|e| S::Error::custom(e))?;
                json!(["inlined", serde_value]).serialize(serializer)
            }
            Expr::Symbol(symbol) => json!(["symbol", symbol]).serialize(serializer),
            Expr::Fn(fn_name, fn_args) => json!(["fn", fn_name, fn_args]).serialize(serializer),
        }
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
