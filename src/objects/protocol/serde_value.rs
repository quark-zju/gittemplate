use serde::ser::SerializeSeq;
use serde_json::Value;
use std::ops::Deref;
use std::sync::Arc;

type LazySeq = Arc<dyn Fn() -> Box<dyn Iterator<Item = crate::Result<SerdeValue>>>>;
/// Represent a dynamic value that can be serialized via serde.
#[derive(Clone)]
pub enum SerdeValue {
    /// Non-lazy serde-json value.
    Value(Arc<Value>),

    /// Lazy sequence.
    LazySeq(LazySeq),
}

impl serde::Serialize for SerdeValue {
    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            SerdeValue::Value(v) => v.deref().serialize(serializer),
            SerdeValue::LazySeq(get_iter) => {
                let iter = get_iter();
                let mut s = serializer.serialize_seq(None)?;
                for item in iter {
                    let item = item.map_err(to_ser_error::<S>)?;
                    s.serialize_element(&item)?;
                }
                s.end()
            }
        }
    }
}

fn to_ser_error<S: serde::Serializer>(e: crate::Error) -> S::Error {
    serde::ser::Error::custom(e)
}

macro_rules! impl_from_t {
    ($($t:ty),*) => {
        $(
            impl From<$t> for SerdeValue {
                fn from(t: $t) -> Self {
                    SerdeValue::Value(Arc::new(Value::from(t)))
                }
            }
        )*
    }
}

impl_from_t!(Value, String, i64, bool, ());

impl From<LazySeq> for SerdeValue {
    fn from(get_iter: LazySeq) -> Self {
        Self::LazySeq(get_iter)
    }
}
