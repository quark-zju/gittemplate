use super::protocol::IntoObject;
use super::protocol::Object;
use super::protocol::ObjectProtocol;
use super::protocol::SerdeValue;
use crate::ast::Expr;
use crate::impl_object;
use crate::Result;
use std::fmt;

#[derive(Clone)]
pub struct ExprObject(Expr);

impl From<Expr> for ExprObject {
    fn from(value: Expr) -> Self {
        Self(value)
    }
}

impl IntoObject for Expr {
    fn into_object(self) -> Object {
        ExprObject::from(self).to_object()
    }
}

impl fmt::Display for ExprObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl ExprObject {
    pub fn as_expr(&self) -> &Expr {
        &self.0
    }
}

impl_object! {
    impl ExprObject {
        fn to_serde_value(&self) -> Result<SerdeValue> {
            let json: serde_json::Value = serde_json::to_value(&self.0)?;
            Ok(json.into())
        }
    }
}
