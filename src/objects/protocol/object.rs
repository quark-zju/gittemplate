use super::fmt;
use super::Attribute;
use crate::Result;
use std::any::Any;
use std::borrow::Cow;
use std::ops::Deref;
use std::sync::Arc;

/// Context used by template renderer.
pub trait ObjectProtocol: fmt::Fmt {
    /// Resolve the given attribute name.
    fn get_attr(&self, name: &str) -> Result<Attribute> {
        let _ = name;
        Ok(Attribute::Missing)
    }

    /// List attributes.
    fn static_list_attrs() -> &'static [&'static str]
    where
        Self: Sized,
    {
        &[]
    }

    /// List attributes.
    fn list_attrs(&self) -> &'static [&'static str];

    /// Auto converting to another type based on accessed attribute name.
    fn deref_object(&self, name: &str) -> Result<Option<Object>> {
        let _ = name;
        Ok(None)
    }

    /// Test whether this value is considered "true".
    fn is_true(&self) -> Result<bool> {
        Ok(true)
    }

    /// Test whether two values are equal.
    fn is_eq(&self, other: &dyn ObjectProtocol) -> Result<bool> {
        Ok(self.type_name() == other.type_name()
            && self.to_plain_string()? == other.to_plain_string()?)
    }

    /// For downcasting.
    fn as_any(&self) -> Option<&dyn Any> {
        None
    }

    /// Obtain the type name (for dyn Trait).
    fn type_name(&self) -> Cow<'static, str>;

    /// Obtain the type name statically.
    fn static_type_name() -> Cow<'static, str>
    where
        Self: Sized;

    /// Convert to `Object`.
    fn to_object(self) -> Object
    where
        Self: Sized + 'static,
    {
        Object(Arc::new(self))
    }

    /// Convert to `SerdeValue`.
    fn to_serde_value(&self) -> Result<super::SerdeValue> {
        let text = self.to_plain_string()?;
        Ok(text.into())
    }

    /// Convert to a plain string.
    fn to_plain_string(&self) -> crate::Result<String> {
        let mut s = String::new();
        let mut f = fmt::Formatter::from_fmt_write(&mut s);
        self.fmt(&mut f)?;
        Ok(s)
    }
}

impl ObjectProtocol for Object {
    fn get_attr(&self, name: &str) -> Result<Attribute> {
        self.0.deref().get_attr(name)
    }

    fn list_attrs(&self) -> &'static [&'static str] {
        self.0.deref().list_attrs()
    }

    fn deref_object(&self, name: &str) -> Result<Option<Object>> {
        self.0.deref().deref_object(name)
    }

    fn is_true(&self) -> Result<bool> {
        self.0.deref().is_true()
    }

    fn is_eq(&self, other: &dyn ObjectProtocol) -> Result<bool> {
        self.0.deref().is_eq(other)
    }

    fn as_any(&self) -> Option<&dyn Any> {
        self.0.deref().as_any()
    }

    fn type_name(&self) -> Cow<'static, str> {
        self.0.deref().type_name()
    }

    fn static_type_name() -> Cow<'static, str> {
        "Object".into()
    }

    fn to_object(self) -> Object {
        Self(self.0.clone())
    }

    fn to_serde_value(&self) -> Result<super::SerdeValue> {
        self.0.deref().to_serde_value()
    }
}

impl Object {
    // fn list_self_attrs(&self) -> &'static [&'static str] {
    //     self.0.deref()
    // }

    /// Write to `w`. Unlike `to_plain_string()`, this function does not
    /// buffer. Unlike `fmt::Display`, this function reports errors.
    pub fn write_to(&self, mut w: impl std::io::Write) -> Result<()> {
        let w = &mut w as &mut dyn std::io::Write;
        let mut f = fmt::Formatter::from_io_write(w);
        self.fmt(&mut f)
    }
}

#[derive(Clone)]
pub struct Object(pub Arc<dyn ObjectProtocol>);

impl fmt::Fmt for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.deref().fmt(f)
    }
}

impl std::ops::Deref for Object {
    type Target = dyn ObjectProtocol;
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}
