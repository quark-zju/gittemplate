use crate::impl_object;
use std::fmt;

/// Path using `/` as separator.
pub struct PosixPathObject {
    path: String,
}

impl PosixPathObject {
    /// Create a new path object.
    pub fn new(path: String) -> Self {
        Self { path }
    }

    /// Split into dirname, basename
    fn split(&self) -> (&str, &str) {
        let path = &self.path;
        let mut split = path.rsplitn(2, '/');
        match split.next() {
            None => ("", ""),
            Some(name) => match split.next() {
                None => ("", name),
                Some(basename) => (name, basename),
            },
        }
    }
}

impl fmt::Display for PosixPathObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.path.fmt(f)
    }
}

impl_object! {
    impl PosixPathObject {
        pub fn basename(&self) -> Self {
            Self { path: self.split().1.to_string() }
        }

        pub fn dirname(&self) -> Self {
            Self { path: self.split().0.to_string() }
        }
    }
}
