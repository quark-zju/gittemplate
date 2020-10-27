use super::protocol::fmt;
use super::protocol::IntoObject;
use super::protocol::Object;
use super::protocol::ObjectProtocol;
use super::protocol::SerdeValue;
use super::IntegerObject;
use super::LambdaObject;
use super::StringObject;
use crate::impl_object;
use crate::Result;
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Clone)]
pub struct ListObject {
    inner: Arc<Mutex<Inner>>,
    separator: String,
}

pub struct ListIter {
    inner: Arc<Mutex<Inner>>,
    index: usize,
}

struct Inner {
    items: Vec<Object>,
    remaining: Option<Box<dyn Iterator<Item = Result<Object>>>>,
}

impl Iterator for ListIter {
    type Item = Result<Object>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut inner = self.inner.lock().unwrap();
        while inner.items.len() <= self.index {
            let remaining = match &mut inner.remaining {
                Some(iter) => iter,
                None => return None,
            };
            match remaining.next() {
                Some(Ok(obj)) => {
                    inner.items.push(obj);
                }
                Some(Err(e)) => {
                    inner.remaining = None;
                    return Some(Err(e));
                }
                None => {
                    inner.remaining = None;
                }
            }
        }
        let result = inner.items.get(self.index).cloned().map(Ok);
        self.index += 1;
        result
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.index += n;
        self.next()
    }
}

impl fmt::Fmt for ListObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let separator = &self.separator;
        let mut first = true;
        for i in self.iter() {
            if first {
                first = false;
            } else {
                f.write_str(separator)?;
            }
            // This is why std::fmt::Display cannot be used.
            // std::fmt::Display has no way to report the actual error.
            i?.fmt(f)?;
        }
        Ok(())
    }
}

impl ListObject {
    pub fn from_vec(items: Vec<Object>) -> Self {
        let inner = Inner {
            items: items,
            remaining: None,
        };
        Self {
            inner: Arc::new(Mutex::new(inner)),
            separator: " ".to_string(),
        }
    }

    pub fn from_iter(iter: impl Iterator<Item = Result<Object>> + 'static) -> Self {
        let inner = Inner {
            items: Vec::new(),
            remaining: Some(Box::new(iter)),
        };
        Self {
            inner: Arc::new(Mutex::new(inner)),
            separator: " ".to_string(),
        }
    }

    pub fn with_separator(mut self, separator: String) -> Self {
        self.separator = separator;
        self
    }

    pub fn iter(&self) -> ListIter {
        ListIter {
            inner: self.inner.clone(),
            index: 0,
        }
    }
}

impl<T: IntoObject> IntoObject for Vec<T> {
    fn into_object(self) -> Object {
        ListObject::from_vec(self.into_iter().map(|v| v.into_object()).collect()).to_object()
    }
}

impl_object! {
    impl ListObject {
        pub fn add(lists: &[&Self]) -> Result<Self> {
            let mut iter: Box<dyn Iterator<Item = Result<Object>>> = Box::new(std::iter::empty());
            let mut sep: Option<String> = None;
            for list in lists {
                iter = Box::new(iter.chain(list.iter()));
                match sep {
                    None => sep = Some(list.separator.clone()),
                    Some(ref sep) => {
                        if &list.separator != sep {
                            let msg = "add() got lists with different separators".to_string();
                            return Err(crate::Error::IncompatibleTypes(msg));
                        }
                    }
                }
            }
            let sep = sep.unwrap_or_else(|| " ".to_string());
            let new_list = Self::from_iter(iter).with_separator(sep);
            Ok(new_list)
        }

        pub fn map(&self, lambda: &LambdaObject) -> Result<Self> {
            let lambda = lambda.clone();
            let iter = self.iter().map(move |item| lambda.apply(item?));
            let new_list = Self::from_iter(iter).with_separator(self.separator.clone());
            Ok(new_list)
        }

        pub fn filter(&self, lambda: &LambdaObject) -> Result<Self> {
            let lambda = lambda.clone();
            let iter = self.iter().filter(move |item| match item {
                Err(_) => true,
                Ok(item) => match lambda.apply(item.clone()) {
                    Err(_) => true,
                    Ok(value) => match value.is_true() {
                        Err(_) => true,
                        Ok(b) => b,
                    },
                },
            });
            let new_list = Self::from_iter(iter).with_separator(self.separator.clone());
            Ok(new_list)
        }

        pub fn skip(&self, n: &IntegerObject) -> Result<Self> {
            let n = n.to_usize()?;
            let iter = self.iter().skip(n);
            let new_list = Self::from_iter(iter).with_separator(self.separator.clone());
            Ok(new_list)
        }

        pub fn take(&self, n: &IntegerObject) -> Result<Self> {
            let n = n.to_usize()?;
            let iter = self.iter().take(n);
            let new_list = Self::from_iter(iter).with_separator(self.separator.clone());
            Ok(new_list)
        }

        pub fn contains(&self, v: Object) -> Result<bool> {
            let found = self.iter().find(|i| match i {
                Err(_) => true,
                Ok(i) => i.is_eq(&v).ok().unwrap_or(false),
            });
            match found {
                Some(Err(e)) => Err(e),
                Some(Ok(_)) => Ok(true),
                None => Ok(false),
            }
        }

        pub fn join(&self, separator: Option<&StringObject>) -> Result<Self> {
            let separator = match separator {
                Some(sep) => sep.as_ref().to_string(),
                None => "".to_string(),
            };
            Ok(self.clone().with_separator(separator))
        }

        pub fn reverse(&self) -> Result<Self> {
            let mut items: Vec<Object> = self.iter().collect::<Result<Vec<_>>>()?;
            items.reverse();
            Ok(Self::from_vec(items).with_separator(self.separator.clone()))
        }

        pub fn sort(&self) -> Result<Self> {
            let mut items: Vec<Object> = self.iter().collect::<Result<Vec<_>>>()?;
            items.sort_by_cached_key(|v| v.to_plain_string().unwrap_or_default());
            Ok(Self::from_vec(items).with_separator(self.separator.clone()))
        }

        pub fn count(&self) -> Result<i64> {
            let mut count = 0;
            for item in self.iter() {
                let _ = item?;
                count += 1;
            }
            Ok(count)
        }

        pub fn nth(&self, index: &IntegerObject) -> Result<Option<Object>> {
            let index = index.to_i64() as usize;
            self.iter().nth(index).transpose()
        }

        pub fn first(&self) -> Result<Option<Object>> {
            self.iter().next().transpose()
        }

        fn to_serde_value(&self) -> Result<SerdeValue> {
            // Converts to a lazy list.
            let this = self.clone();
            let get_iter = move || {
                let this = this.clone();
                let iter = this.iter().map(|o| o.and_then(|o| o.to_serde_value()));
                let iter: Box<dyn Iterator<Item = crate::Result<SerdeValue>>> = Box::new(iter);
                iter
            };
            let get_iter: Arc<dyn Fn() -> Box<dyn Iterator<Item = crate::Result<SerdeValue>>>> =
                Arc::new(get_iter);
            Ok(SerdeValue::from(get_iter))
        }
    }
}
