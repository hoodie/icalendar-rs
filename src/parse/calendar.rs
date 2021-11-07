use super::Component;
use core::fmt::{self, Write};

/// Helpertype for reserialization
#[derive(Clone, Debug)]
pub struct Calendar<'a> {
    pub components: Vec<Component<'a>>,
}

impl Calendar<'_> {
    /// Writes `Component` into a `Writer` using `std::fmt`.
    pub fn fmt_write<W: Write>(&self, out: &mut W) -> Result<(), fmt::Error> {
        for component in &self.components {
            component.fmt_write(out)?;
        }
        Ok(())
    }
    /// Prints to stdout
    pub fn print(&self) -> Result<(), fmt::Error> {
        print_crlf!("{}", self);
        Ok(())
    }
}

impl fmt::Display for Calendar<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_write(f)
    }
}
