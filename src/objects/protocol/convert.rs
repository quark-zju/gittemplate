use super::Object;
use super::ObjectProtocol;
use crate::Error;
use crate::Result;
use std::any::Any;
use std::ops::Deref;

/// Converts into an object.
///
/// Works for more types than `ObjectProtocol`. For example, `bool` implements
/// `IntoObject` but not `ObjectProtocol`.
pub trait IntoObject {
    fn into_object(self) -> Object;
}

impl IntoObject for Object {
    fn into_object(self) -> Object {
        self
    }
}

/// Convert from an object.
pub trait FromObject {
    fn from_object(object: &Object) -> Result<Self>
    where
        Self: Sized;
}

/// Convert from an object, by reference.
pub trait FromObjectRef {
    fn from_object(object: &Object) -> Result<&Self>
    where
        Self: Sized;
}

impl FromObject for Object {
    fn from_object(object: &Object) -> Result<Self> {
        Ok(object.clone())
    }
}

pub fn downcast<T: ObjectProtocol + Any + Sized>(this: &dyn ObjectProtocol) -> Result<&T> {
    if let Some(any) = this.as_any() {
        if let Some(value) = any.downcast_ref() {
            return Ok(value);
        }
    }
    Err(Error::MismatchedType(
        format!("{} ({})", this.to_plain_string()?, this.type_name()),
        T::static_type_name().to_string(),
    ))
}

impl<T: Any + ObjectProtocol + Sized> FromObjectRef for T {
    fn from_object(object: &Object) -> Result<&T> {
        downcast(object.deref())
    }
}

// Common types defined by ObjectProtocol

impl FromObject for bool {
    fn from_object(object: &Object) -> Result<Self> {
        object.is_true()
    }
}

impl FromObject for String {
    fn from_object(object: &Object) -> Result<Self> {
        Ok(object.to_plain_string()?)
    }
}
