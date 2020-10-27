use super::protocol::IntoObject;
use super::protocol::Object;
use super::protocol::ObjectProtocol;
use super::protocol::SerdeValue;
use crate::impl_object;
use crate::Result;
use std::fmt;

#[derive(Copy, Clone)]
pub struct NilObject;

impl<T: IntoObject> IntoObject for Option<T> {
    fn into_object(self) -> Object {
        match self {
            None => NilObject.to_object(),
            Some(t) => t.into_object(),
        }
    }
}

impl fmt::Display for NilObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let _ = f;
        Ok(())
    }
}

impl_object! {
    impl NilObject {
        fn is_true(&self) -> Result<bool> {
            Ok(false)
        }

        fn to_serde_value(&self) -> Result<SerdeValue> {
            Ok(().into())
        }
    }
}
