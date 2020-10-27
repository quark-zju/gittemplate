use super::protocol::IntoObject;
use super::protocol::Object;
use super::protocol::ObjectProtocol;
use super::protocol::SerdeValue;
use crate::impl_object;
use crate::Result;
use std::fmt;

#[derive(Copy, Clone)]
pub struct BoolObject(bool);

impl From<bool> for BoolObject {
    fn from(value: bool) -> Self {
        Self(value)
    }
}

impl IntoObject for bool {
    fn into_object(self) -> Object {
        BoolObject::from(self).to_object()
    }
}

impl fmt::Display for BoolObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl BoolObject {
    pub fn as_bool(&self) -> bool {
        self.0
    }
}

impl_object! {
    impl BoolObject {
        fn is_true(&self) -> Result<bool> {
            Ok(self.0)
        }

        fn to_serde_value(&self) -> Result<SerdeValue> {
            Ok(self.as_bool().into())
        }
    }
}
