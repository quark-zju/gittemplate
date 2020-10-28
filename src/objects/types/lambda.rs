use super::protocol::Object;
use super::protocol::SerdeValue;
use crate::ast::Expr;
use crate::eval;
use crate::impl_object;
use crate::Result;
use std::borrow::Cow;
use std::fmt;
use std::fmt::Display;

#[derive(Clone)]
pub struct LambdaObject {
    pub context: eval::EvalContext,
    pub arg_name: Cow<'static, str>,
    pub body: Expr,
}

impl LambdaObject {
    pub fn new(
        context: eval::EvalContext,
        arg_name: impl Into<Cow<'static, str>>,
        body: Expr,
    ) -> Self {
        let arg_name = arg_name.into();
        Self {
            context,
            arg_name,
            body,
        }
    }
}

impl Display for LambdaObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} => {}", &self.arg_name, &self.body)
    }
}

impl_object! {
    impl LambdaObject {
        /// Apply values to an lambda expression. `apply(lambda, x)`.
        pub fn apply(&self, value: Object) -> Result<Object> {
            let mut body = self.body.clone();
            let value = Expr::Inlined(value);
            body.replace(&self.arg_name, &value);
            self.context.eval_expr(&body)
        }

        fn to_serde_value(&self) -> Result<SerdeValue> {
            Err(crate::Error::IncompatibleTypes("Lambda cannot be converted to JSON".to_string()))
        }
    }
}
