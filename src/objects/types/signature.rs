use super::protocol::SerdeValue;
use super::time::TimestampObject;
use crate::impl_object;
use crate::Result;
use std::fmt;

#[derive(Clone)]
pub struct SignatureObject {
    name: String,
    email: String,
    when: TimestampObject,
}

impl From<git2::Signature<'_>> for SignatureObject {
    fn from(value: git2::Signature) -> Self {
        Self {
            name: String::from_utf8_lossy(value.name_bytes()).to_string(),
            email: String::from_utf8_lossy(value.email_bytes()).to_string(),
            when: value.when().into(),
        }
    }
}

impl fmt::Display for SignatureObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} <{}>", &self.name, &self.email)
    }
}

impl_object! {
    impl SignatureObject {
        fn date(&self) -> TimestampObject {
            self.when
        }

        fn name(&self) -> String {
            self.name.clone()
        }

        fn email(&self) -> String {
            self.email.clone()
        }

        fn to_serde_value(&self) -> Result<SerdeValue> {
            let value = serde_json::json!({
                "date": self.date().to_serde_value()?,
                "name": self.name(),
                "email": self.email(),
            });
            Ok(value.into())
        }
    }
}
