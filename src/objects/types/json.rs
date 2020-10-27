use super::protocol::fmt;
use super::protocol::SerdeValue;
use super::BoolObject;
use crate::impl_object;
use crate::Result;

#[derive(Clone)]
pub struct JsonObject {
    value: SerdeValue,
    pretty: bool,
}

impl From<SerdeValue> for JsonObject {
    fn from(value: SerdeValue) -> Self {
        Self {
            value,
            pretty: false,
        }
    }
}

impl fmt::Fmt for JsonObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.pretty {
            serde_json::to_writer_pretty(f, &self.value)?;
        } else {
            serde_json::to_writer(f, &self.value)?;
        }
        Ok(())
    }
}

impl JsonObject {
    pub fn as_serde_value(&self) -> &SerdeValue {
        &self.value
    }
}

impl_object! {
    impl JsonObject {
        pub fn pretty(&self, value: Option<&BoolObject>) -> Result<Self> {
            let mut other = self.clone();
            let pretty = value.map(|v| v.as_bool()).unwrap_or(true);
            other.pretty = pretty;
            Ok(other)
        }

        fn to_serde_value(&self) -> Result<SerdeValue> {
            Ok(self.value.clone())
        }
    }
}
