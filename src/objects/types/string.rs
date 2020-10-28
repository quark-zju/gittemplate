use super::protocol::fmt;
use super::protocol::FromObjectRef;
use super::protocol::IntoObject;
use super::protocol::Object;
use super::protocol::ObjectProtocol;
use super::IntegerObject;
use super::ListObject;
use super::RegexObject;
use crate::ast::Expr;
use crate::eval;
use crate::impl_object;
use crate::Result;
use ansi_term::Color;
use ansi_term::Style;

#[derive(Clone)]
pub struct StringObject(String);

impl From<String> for StringObject {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl IntoObject for String {
    fn into_object(self) -> Object {
        StringObject::from(self).to_object()
    }
}

impl AsRef<str> for StringObject {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl fmt::Fmt for StringObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl StringObject {}

impl_object! {
    impl StringObject {
        pub fn add(strings: &[&StringObject]) -> String {
            // `strings: &[&StringObject]` will perform typecheck.
            // `strings: &[String]` converts arguments to strings.
            let strings: Vec<&str> = strings.iter().map(|s| s.0.as_ref()).collect();
            strings.concat()
        }

        pub fn repeat(&self, n: &IntegerObject) -> Result<String> {
            Ok(str::repeat(self.as_ref(), n.to_usize()?))
        }

        pub fn mul(&self, n: &IntegerObject) -> Result<String> {
            self.repeat(n)
        }

        pub fn upper(&self) -> String {
            self.0.to_uppercase()
        }

        pub fn lower(&self) -> String {
            self.0.to_lowercase()
        }

        pub fn len(&self) -> i64 {
            self.0.len() as i64
        }

        pub fn lines(&self) -> ListObject {
            struct Lines {
                text: String,
                pos: usize,
            }
            impl Iterator for Lines {
                type Item = String;
                fn next(&mut self) -> Option<Self::Item> {
                    let remaining = &self.text[self.pos..];
                    if remaining.is_empty() {
                        None
                    } else {
                        match remaining.find('\n') {
                            Some(n) => {
                                let result = remaining[..n].to_string();
                                self.pos += n + 1;
                                Some(result)
                            }
                            None => {
                                self.pos = self.text.len();
                                Some(remaining.to_string())
                            }
                        }
                    }
                }
            }
            let text = self.0.clone();
            let iter = Lines { text, pos: 0 };
            ListObject::from_iter(iter.map(move |s| Ok(s.to_string().into_object())))
                .with_separator("\n".to_string())
        }

        pub fn contains(&self, rhs: Object) -> Result<bool> {
            if let Ok(re) = RegexObject::from_object(&rhs) {
                Ok(re.is_match(self.as_ref()))
            } else if let Ok(s) = Self::from_object(&rhs) {
                Ok(self.as_ref().contains(s.as_ref()))
            } else {
                Err(crate::Error::MismatchedType(
                    format!("{} ({})", self.as_ref(), self.type_name()),
                    "String or Regex".to_string(),
                ))
            }
        }

        pub fn matches(&self, pat: &RegexObject) -> Result<Option<Vec<Option<String>>>> {
            pat.matches(self)
        }

        pub fn gsub(&self, from: Object, to: Object) -> Result<String> {
            if let (Ok(from), Ok(to)) = (Self::from_object(&from), Self::from_object(&to)) {
                Ok(self.as_ref().replace(from.as_ref(), to.as_ref()))
            } else if let (Ok(from), to) = (RegexObject::from_object(&from), to) {
                from.gsub(self, to)
            } else {
                Err(crate::Error::MismatchedType(
                    format!("{} ({})", self.as_ref(), self.type_name()),
                    "String | Regex".to_string(),
                ))

            }
        }

        pub fn label(context: &eval::EvalContext, args: &[Expr]) -> Result<Object> {
            label(context, args)
        }

        pub fn black(&self) -> String {
            Color::Black.paint(&self.0).to_string()
        }

        pub fn red(&self) -> String {
            Color::Red.paint(&self.0).to_string()
        }

        pub fn green(&self) -> String {
            Color::Green.paint(&self.0).to_string()
        }

        pub fn yellow(&self) -> String {
            Color::Yellow.paint(&self.0).to_string()
        }

        pub fn blue(&self) -> String {
            Color::Blue.paint(&self.0).to_string()
        }

        pub fn purple(&self) -> String {
            Color::Purple.paint(&self.0).to_string()
        }

        pub fn cyan(&self) -> String {
            Color::Cyan.paint(&self.0).to_string()
        }

        pub fn white(&self) -> String {
            Color::White.paint(&self.0).to_string()
        }

        pub fn bold(&self) -> String {
            Style::default().bold().paint(&self.0).to_string()
        }

        fn to_ast_fmt_string(&self) -> String {
            format!("{:?}", self.as_ref())
        }
    }
}

fn label(context: &eval::EvalContext, args: &[Expr]) -> Result<Object> {
    // Use `context` to decide whether to use color.
    eval::ensure_arg_count("label", args, 2..)?;
    let text = context.eval_expr(&args[0])?.to_plain_string()?;
    let labels = args[1..]
        .iter()
        .map(|a| Ok(context.eval_expr(a)?.to_plain_string()?))
        .collect::<Result<Vec<String>>>()?
        .into_iter()
        .flat_map(|s| {
            s.split_whitespace()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<String>>();
    let mut style = Style::default();
    for label in labels {
        match label.as_ref() {
            "black" => style = style.fg(Color::Black),
            "red" => style = style.fg(Color::Red),
            "green" => style = style.fg(Color::Green),
            "yellow" => style = style.fg(Color::Yellow),
            "blue" => style = style.fg(Color::Blue),
            "purple" => style = style.fg(Color::Purple),
            "cyan" => style = style.fg(Color::Cyan),
            "white" => style = style.fg(Color::White),
            "black_bg" => style = style.on(Color::Black),
            "red_bg" => style = style.on(Color::Red),
            "green_bg" => style = style.on(Color::Green),
            "yellow_bg" => style = style.on(Color::Yellow),
            "blue_bg" => style = style.on(Color::Blue),
            "purple_bg" => style = style.on(Color::Purple),
            "cyan_bg" => style = style.on(Color::Cyan),
            "white_bg" => style = style.on(Color::White),
            "bold" => style = style.bold(),
            "dim" => style = style.dimmed(),
            "italic" => style = style.italic(),
            "underline" => style = style.underline(),
            "strikethrough" => style = style.strikethrough(),
            "blink" => style = style.blink(),
            // TODO: 256 or real colors.
            // TODO: Color aliases in git config.
            _ => {}
        }
    }

    let text = if style.is_plain() {
        text
    } else {
        style.paint(text).to_string()
    };
    Ok(text.into_object())
}
