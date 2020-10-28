use crate::ast::Expr;
use crate::ast::ParseToExpr;
use crate::objects::protocol::Attribute;
use crate::objects::protocol::IntoObject;
use crate::objects::protocol::Object;
use crate::objects::protocol::ObjectProtocol;
use crate::objects::types::BoolObject;
use crate::objects::types::ExprObject;
use crate::objects::types::JsonObject;
use crate::objects::types::LambdaObject;
use crate::objects::types::ListObject;
use crate::objects::types::NilObject;
use crate::objects::types::RegexObject;
use crate::objects::types::RepoObject;
use crate::objects::types::TimestampObject;
use crate::parser::MISSING;
use crate::Error;
use crate::Result;
use std::fmt;
use std::ops;
use std::ops::Deref;
use std::sync::Arc;

/// Context for evaluating template expressions.
#[derive(Clone)]
pub struct EvalContext {
    /// Configuration.
    git_config: Option<Arc<git2::Config>>,

    /// Default context object.
    global_object: Object,

    /// Whether to enable ANSI colors or not.
    colors: bool,
}

impl EvalContext {
    /// Construct an empty [`EvalContext`].
    ///
    /// Colors are auto-detected.
    pub fn new() -> Self {
        Self {
            git_config: None,
            global_object: NilObject.to_object(),
            colors: false,
        }
    }

    /// Attach git config to the context.
    pub fn with_git_config(mut self, git_config: git2::Config) -> Self {
        self.git_config = Some(Arc::new(git_config));
        self
    }

    /// Set the global object.
    ///
    /// Attributes of the root object are accessible from the template
    /// language.
    pub fn with_global_object(mut self, obj: impl IntoObject) -> Self {
        self.global_object = obj.into_object();
        self
    }

    /// Set whether to enable ANSI colors or not.
    pub fn with_colors(mut self, colors: bool) -> Self {
        self.colors = colors;
        self
    }

    /// Decide colors by tty.
    pub fn with_colors_by_tty(mut self) -> Self {
        self.colors = atty::is(atty::Stream::Stdout);
        self
    }
}

impl EvalContext {
    /// Evaluate an expression.
    pub fn eval(&self, expr: impl ParseToExpr) -> Result<Object> {
        let expr = expr.parse_to_expr()?;
        self.eval_expr(&expr)
    }

    /// Evaluate the AST under the given context.
    pub(crate) fn eval_expr(&self, expr: &Expr) -> Result<Object> {
        let object = &self.global_object;
        let value: Object = match expr {
            Expr::Symbol(name) => match object.get_attr(&name)? {
                Attribute::Missing => match self.resolve_global_symbol(name) {
                    Some(value) => value,
                    None => {
                        if name == MISSING {
                            return Err(missing_symbol_hint(object));
                        }
                        return Err(Error::UnresolvedSymbol(name.to_string()));
                    }
                },
                Attribute::Value(value) => value,
                Attribute::Method(_method) => {
                    return Err(Error::UnresolvedSymbol(name.to_string()))
                }
            },
            Expr::Fn(name, args) => {
                match self.resolve_global_function(&name) {
                    None => {
                        // Try local attribute and method. Note: args[0] will be no longer lazy.
                        if let Some(this_expr) = args.get(0) {
                            let this = self.eval_expr(this_expr)?;
                            let typename = this.type_name();
                            if name == MISSING {
                                // Show attributes on `this`.
                                return Err(missing_symbol_hint(&this));
                            }
                            if let Some(value) = self.method_call(this, name, &args)? {
                                return Ok(value);
                            }
                            return Err(Error::UnresolvedSymbol(format!(
                                "{} (on type {})",
                                name, typename
                            )));
                        }
                        return Err(Error::UnresolvedSymbol(name.to_string()));
                    }
                    Some(func) => func(self, args)?,
                }
            }
            Expr::Inlined(value) => value.clone(),
        };
        Ok(value)
    }

    /// Partially execute an AST. Useful by lambda called repetitively, to avoid
    /// repetitive computation like constructing regex over and over.
    pub fn partial_eval(&self, mut expr: Expr) -> Expr {
        match expr {
            Expr::Inlined(_) => {}
            Expr::Symbol(ref s) => {
                if let Some(obj) = self.resolve_global_symbol(s.as_ref()) {
                    return Expr::Inlined(obj);
                }
            }
            Expr::Fn(fn_name, args) => {
                let args: Vec<Expr> = args.into_iter().map(|e| self.partial_eval(e)).collect();
                match fn_name.as_ref() {
                    "re" => {
                        if let [Expr::Inlined(_)] = &args[..] {
                            // Pre-compile the regex.
                            if let Ok(compiled) = re(self, &args) {
                                return Expr::Inlined(compiled);
                            }
                        }
                    }
                    _ => {}
                }
                expr = Expr::Fn(fn_name, args);
            }
        }
        expr
    }

    /// Try to call a function on `this`.
    fn method_call(&self, this: Object, fn_name: &str, args: &[Expr]) -> Result<Option<Object>> {
        match this.get_attr(fn_name)? {
            Attribute::Missing => {
                // Try auto cast to a dynamic "deref" object.
                if let Some(obj) = this.deref_object(fn_name)? {
                    return self.method_call(obj, fn_name, args);
                }
            }
            Attribute::Value(value) => {
                // Syntax sugar: foo(x) => x.foo
                ensure_arg_count(fn_name, args, 1..=1)?;
                return Ok(Some(value));
            }
            Attribute::Method(func) => {
                // Syntax sugar: foo(x, ...) => x.foo(...)
                let mut new_args = args.to_vec();
                // Do not evaluate args[0] again.
                new_args[0] = Expr::Inlined(this);
                return Ok(Some(func(self, &new_args)?));
            }
        }
        Ok(None)
    }

    fn resolve_global_symbol(&self, name: &str) -> Option<Object> {
        // Try global object.
        let object = &self.global_object;
        if let Ok(Attribute::Value(value)) = object.get_attr(&name) {
            return Some(value);
        }
        // Try hard-coded names.
        if let Some(value) = resolve_static_global_symbols(name) {
            return Some(value);
        }
        None
    }

    fn resolve_global_function(
        &self,
        name: &str,
    ) -> Option<Box<dyn Fn(&EvalContext, &[Expr]) -> Result<Object>>> {
        // Try functions on global object.
        let object = &self.global_object;
        if let Ok(Attribute::Method(func)) = object.get_attr(&name) {
            let object = object.clone();
            let wrapper = move |context: &EvalContext, args: &[Expr]| -> Result<Object> {
                let result = func(context, &args);
                match result {
                    Err(Error::MismatchedArguments(..)) | Err(Error::MismatchedType(..)) => {
                        // Retry by inserting the global object as args[0].
                        let args: Vec<_> = std::iter::once(Expr::Inlined(object.clone()))
                            .chain(args.iter().cloned())
                            .collect();
                        func(context, &args)
                    }
                    _ => result,
                }
            };
            return Some(Box::new(wrapper));
        }
        // Try hard-coded functions.
        let func = match name {
            // Control flow (must use Expr for laziness).
            "if" => r#if,
            "try" => r#try,
            "and" => and,
            "or" => or,
            // Methods on general objects.
            "str" => r#str,
            "concat" => concat,
            "eq" => eq,
            "not" => not,
            "ast" => ast,
            "json" => json,
            "typename" => typename,
            // Constructors.
            "lambda" => lambda,
            "vec" => vec,
            "re" => re,
            "repo" => repo,
            "now" => now,
            _ => return None,
        };
        Some(Box::new(func))
    }
}

fn resolve_static_global_symbols(name: &str) -> Option<Object> {
    match name {
        "true" => Some(true.into_object()),
        "false" => Some(false.into_object()),
        "nil" => Some(NilObject.to_object()),
        _ => name.parse::<i64>().ok().map(|i| i.into_object()),
    }
}

fn r#if(context: &EvalContext, args: &[Expr]) -> Result<Object> {
    ensure_arg_count("if", args, 2..=3)?;
    let cond = context.eval_expr(&args[0])?;
    if cond.is_true()? {
        context.eval_expr(&args[1])
    } else {
        if args.len() < 2 {
            Ok(NilObject.to_object())
        } else {
            context.eval_expr(&args[2])
        }
    }
}

fn and(context: &EvalContext, args: &[Expr]) -> Result<Object> {
    // Take `Expr` for laziness.
    let mut obj = NilObject.to_object();
    for expr in args {
        obj = context.eval_expr(expr)?;
        if !obj.is_true()? {
            return Ok(obj);
        }
    }
    Ok(obj)
}

fn or(context: &EvalContext, args: &[Expr]) -> Result<Object> {
    // Take `Expr` for laziness.
    let mut obj = NilObject.to_object();
    for expr in args {
        obj = context.eval_expr(expr)?;
        if obj.is_true()? {
            return Ok(obj);
        }
    }
    Ok(obj)
}

/// Test if 2 values are the same.
fn eq(context: &EvalContext, args: &[Expr]) -> Result<Object> {
    ensure_arg_count("eq", args, 2..=2)?;
    let lhs = context.eval_expr(&args[0])?;
    let rhs = context.eval_expr(&args[1])?;
    let result = BoolObject::from(lhs.is_eq(rhs.deref())?);
    Ok(result.to_object())
}

/// Converts to a non-lazy string.
fn r#str(context: &EvalContext, args: &[Expr]) -> Result<Object> {
    ensure_arg_count("str", args, 1..=1)?;
    let value = context.eval_expr(&args[0])?;
    let string = value.to_plain_string()?;
    Ok(string.into_object())
}

/// Logical not.
fn not(context: &EvalContext, args: &[Expr]) -> Result<Object> {
    ensure_arg_count("not", args, 1..=1)?;
    let value = context.eval_expr(&args[0])?;
    let not_value = !value.is_true()?;
    Ok(not_value.into_object())
}

/// Construct a list from items.
fn concat(context: &EvalContext, args: &[Expr]) -> Result<Object> {
    let context = context.clone();
    let args = args.to_vec();
    let iter = args.into_iter().map(move |a| context.eval_expr(&a));
    Ok(ListObject::from_iter(iter)
        .with_separator("".to_string())
        .to_object())
}

/// Construct a lambda expression. `lambda(x, foo(x))`.
fn lambda(context: &EvalContext, args: &[Expr]) -> Result<Object> {
    let _ = context;
    ensure_arg_count("lambda", args, 2..=2)?;
    let arg_name = if let Expr::Symbol(arg_name) = &args[0] {
        arg_name.clone()
    } else {
        return Err(Error::MismatchedType(
            args[0].to_string(),
            "symbol".to_string(),
        ));
    };
    let body = args[1].clone();
    let body = context.partial_eval(body);
    let context = context.clone();
    let lambda = LambdaObject::new(context, arg_name, body);
    Ok(lambda.to_object())
}

/// Construct a list.
fn vec(context: &EvalContext, args: &[Expr]) -> Result<Object> {
    let items = args
        .iter()
        .map(|a| context.eval_expr(a))
        .collect::<Result<Vec<_>>>()?;
    Ok(ListObject::from_vec(items).to_object())
}

/// Construct a regex.
fn re(context: &EvalContext, args: &[Expr]) -> Result<Object> {
    ensure_arg_count("re", args, 1..=1)?;
    let pat = context.eval_expr(&args[0])?.to_plain_string()?;
    RegexObject::compile(&pat).map(|r| r.to_object())
}

/// Construct a repo.
fn repo(context: &EvalContext, args: &[Expr]) -> Result<Object> {
    ensure_arg_count("repo", args, ..=1)?;
    let git_repo = if args.is_empty() {
        git2::Repository::open_from_env()?
    } else {
        let path = context.eval_expr(&args[0])?.to_plain_string()?;
        git2::Repository::open(&path)?
    };
    let repo = RepoObject::from(git_repo);
    Ok(repo.to_object())
}

/// Construct a timestamp.
fn now(context: &EvalContext, args: &[Expr]) -> Result<Object> {
    let _ = context;
    ensure_arg_count("now", args, 0..=0)?;
    let now = TimestampObject::now();
    Ok(now.to_object())
}

/// Silent missing symbol errors.
fn r#try(context: &EvalContext, args: &[Expr]) -> Result<Object> {
    ensure_arg_count("try", args, 1..=2)?;
    let expr = &args[0];
    let result = context.eval_expr(expr);
    match result {
        Err(_) => {
            let fallback = match args.get(1) {
                Some(expr) => context.eval_expr(expr)?,
                None => NilObject.to_object(),
            };
            Ok(fallback)
        }
        _ => result,
    }
}

/// Obtains the AST string.
fn ast(_context: &EvalContext, args: &[Expr]) -> Result<Object> {
    ensure_arg_count("ast", args, 1..=1)?;
    Ok(ExprObject::from(args[0].clone()).to_object())
}

/// Obtains the type name.
fn typename(context: &EvalContext, args: &[Expr]) -> Result<Object> {
    ensure_arg_count("typename", args, 1..=1)?;
    let value = context.eval_expr(&args[0])?;
    Ok(value.type_name().to_string().into_object())
}

/// Format to a JSON string.
fn json(context: &EvalContext, args: &[Expr]) -> Result<Object> {
    ensure_arg_count("json", args, 1..=1)?;
    let obj = context.eval_expr(&args[0])?;
    let json_value = obj.to_serde_value()?;
    let json_obj = JsonObject::from(json_value);
    Ok(json_obj.to_object())
}

/// Ensure the number of arguments.
pub(crate) fn ensure_arg_count(
    func_name: &str,
    args: &[Expr],
    required: impl ops::RangeBounds<usize> + fmt::Debug + 'static,
) -> Result<()> {
    if !required.contains(&args.len()) {
        Err(Error::MismatchedArguments(
            func_name.to_string(),
            Box::new(required),
            args.len(),
        ))
    } else {
        Ok(())
    }
}

/// Return MissingSymbol error with hints on attributes of the object.
fn missing_symbol_hint(obj: &Object) -> Error {
    let attrs = obj.list_attrs();
    let attrs = attrs.iter().map(|s| s.to_string()).collect();
    return Error::MissingingSymbol(attrs);
}
