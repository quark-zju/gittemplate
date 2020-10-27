use std::fmt::Write as _;
use std::io;

pub type Result = crate::Result<()>;

pub struct Formatter<'a>(Inner<'a>);
enum Inner<'a> {
    /// Str-based.
    Str(&'a mut dyn std::fmt::Write),
    /// Bytes-based.
    Bytes(&'a mut dyn std::io::Write),
}

/// Similar to `std::fmt::Display`. But is fallible.
pub trait Fmt {
    fn fmt(&self, f: &mut Formatter) -> Result;
}

impl<T: std::fmt::Display> Fmt for T {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self)?;
        Ok(())
    }
}

impl<'a> Formatter<'a> {
    pub fn write_str(&mut self, text: &str) -> Result {
        match self.0 {
            Inner::Str(ref mut s) => s.write_str(text)?,
            Inner::Bytes(ref mut w) => w.write_all(text.as_bytes())?,
        }
        Ok(())
    }

    pub fn from_fmt_write(w: &'a mut dyn std::fmt::Write) -> Self {
        Self(Inner::Str(w))
    }

    pub fn from_io_write(w: &'a mut dyn io::Write) -> Self {
        Self(Inner::Bytes(w))
    }
}

// Make `write!(f, ...)` work in `fmt()` implementation.
// Used above.
impl std::fmt::Write for Formatter<'_> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        Formatter::write_str(self, s).map_err(|_| std::fmt::Error)
    }
}

// Used by `serde_json::to_writer` in `JsonObject`.
impl std::io::Write for Formatter<'_> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self.0 {
            Inner::Bytes(ref mut w) => w.write(buf),
            Inner::Str(ref mut w) => {
                w.write_str(&String::from_utf8_lossy(buf))
                    .map_err(map_fmt_error)?;
                Ok(buf.len())
            }
        }
    }

    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        match self.0 {
            Inner::Bytes(ref mut w) => w.write_all(buf),
            Inner::Str(ref mut w) => {
                w.write_str(&String::from_utf8_lossy(buf))
                    .map_err(map_fmt_error)?;
                Ok(())
            }
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match self.0 {
            Inner::Bytes(ref mut w) => w.flush(),
            Inner::Str(_) => Ok(()),
        }
    }
}

fn map_fmt_error(e: std::fmt::Error) -> io::Error {
    io::Error::new(io::ErrorKind::Other, e)
}
