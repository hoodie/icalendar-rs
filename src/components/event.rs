use super::*;
use crate::query::ComponentQuery;
/// VEVENT [(RFC 5545, Section 3.6.1 )](https://tools.ietf.org/html/rfc5545#section-3.6.1)
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Event {
    pub(super) inner: InnerComponent,
}

impl Event {
    /// Creates a new Event.
    pub fn new() -> Self {
        Default::default()
    }

    /// End of builder pattern.
    /// copies over everything
    pub fn done(&mut self) -> Self {
        Event {
            inner: self.inner.done(),
        }
    }

    ///  Defines the overall status or confirmation
    pub fn status(&mut self, status: EventStatus) -> &mut Self {
        self.append_property(status.into());
        self
    }

    //pub fn repeats<R:Repeater+?Sized>(&mut self, repeat: R) -> &mut Self {
    //    unimplemented!()
    //}

    /// Returns a queryable object
    #[cfg(feature = "query")]
    pub fn query(&self) -> ComponentQuery<'_, Self> {
        ComponentQuery::from(self)
    }
}

// impl std::Str
