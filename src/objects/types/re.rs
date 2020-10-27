use super::protocol::FromObjectRef;
use super::protocol::IntoObject;
use super::protocol::Object;
use super::protocol::ObjectProtocol;
use super::protocol::SerdeValue;
use super::LambdaObject;
use super::StringObject;
use crate::impl_object;
use crate::Result;
use regex::Regex;
use std::fmt;

pub struct RegexObject(Regex);

macro_rules! with_replacer {
    ($obj:expr, $func:expr) => {
        if let Ok(rep) = StringObject::from_object($obj) {
            ($func)(rep.as_ref())
        } else if let Ok(lambda) = LambdaObject::from_object($obj) {
            let lambda = lambda.clone();
            let replace_func = move |captures: &regex::Captures| -> String {
                if let Some(m) = captures.get(0) {
                    let s = m.as_str();
                    if let Ok(r) = lambda.apply(s.to_string().into_object()) {
                        // Errors are ignored.
                        r.to_plain_string().unwrap_or_else(|s| s.to_string())
                    } else {
                        s.to_string()
                    }
                } else {
                    // Shouldn't happen.
                    Default::default()
                }
            };
            ($func)(replace_func)
        } else {
            return Err(crate::Error::IncompatibleTypes(format!(
                "sub requires Lambda or String, got {}",
                $obj.type_name()
            )));
        }
    };
}

impl RegexObject {
    pub fn compile(s: &str) -> Result<Self> {
        let regex = Regex::new(s)?;
        Ok(Self(regex))
    }

    pub fn from_literal(s: &str) -> Result<Self> {
        Self::compile(&regex::escape(s))
    }

    pub fn is_match(&self, s: &str) -> bool {
        self.0.is_match(s)
    }
}

impl fmt::Display for RegexObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "re({:?})", self.0.as_str())
    }
}

impl_object! {
    impl RegexObject {
        pub fn matches(&self, text: &StringObject) -> Result<Option<Vec<Option<String>>>> {
            let result = match self.0.captures(text.as_ref()) {
                Some(captures) => {
                    let items: Vec<Option<String>> = captures
                        .iter()
                        .map(|i| match i {
                            None => None,
                            Some(i) => Some(i.as_str().to_string()),
                        })
                        .collect();
                    Some(items)
                }
                None => None,
            };
            Ok(result)
        }


        pub fn sub(&self, text: &StringObject, rep: Object) -> Result<String> {
            let result = with_replacer!(&rep, |r| self.0.replace(text.as_ref(), r).to_string());
            Ok(result)
        }

        pub fn gsub(&self, text: &StringObject, rep: Object) -> Result<String> {
            let result = with_replacer!(&rep, |r| self.0.replace_all(text.as_ref(), r).to_string());
            Ok(result)
        }

        fn to_serde_value(&self) -> Result<SerdeValue> {
            Err(crate::Error::IncompatibleTypes("Regex cannot be converted to JSON".to_string()))
        }
    }
}
