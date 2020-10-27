use super::protocol::IntoObject;
use super::protocol::Object;
use super::protocol::ObjectProtocol;
use super::protocol::SerdeValue;
use crate::impl_object;
use crate::Result;
use std::fmt;

#[derive(Copy, Clone)]
pub struct IntegerObject(i64);

impl IntegerObject {
    pub fn to_i64(&self) -> i64 {
        self.0
    }

    pub fn to_usize(&self) -> Result<usize> {
        Ok(self.0.max(0) as usize)
    }
}

impl From<i64> for IntegerObject {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

impl IntoObject for i64 {
    fn into_object(self) -> Object {
        IntegerObject::from(self).to_object()
    }
}

impl fmt::Display for IntegerObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl_object! {
    impl IntegerObject {
        pub fn negative(&self) -> Self {
            Self(-self.0)
        }

        pub fn neg(&self) -> Self {
            self.negative()
        }

        pub fn add(values: &[&Self]) -> Self {
            Self(values.iter().map(|v| v.0).sum())
        }

        pub fn lt(&self, rhs: &Self) -> bool {
            self.to_i64() < rhs.to_i64()
        }

        pub fn mul(&self, rhs: &Self) -> i64 {
            self.to_i64() * rhs.to_i64()
        }

        fn to_serde_value(&self) -> Result<SerdeValue> {
            Ok(self.to_i64().into())
        }
    }
}
