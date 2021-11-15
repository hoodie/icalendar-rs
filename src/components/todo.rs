use chrono::*;

use super::*;

/// VTODO  [(RFC 5545, Section 3.6.2 )](https://tools.ietf.org/html/rfc5545#section-3.6.2)
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Todo {
    pub(super) inner: InnerComponent,
}

impl Todo {
    /// Creates a new Todo.
    pub fn new() -> Self {
        Default::default()
    }

    /// End of builder pattern.
    /// copies over everything
    pub fn done(&mut self) -> Self {
        Todo {
            inner: self.inner.done(),
        }
    }

    /// Set the `PERCENT-COMPLETE` property
    ///
    /// Ranges between 0 - 100
    pub fn percent_complete(&mut self, percent: u8) -> &mut Self {
        self.add_property("PERCENT-COMPLETE", &percent.to_string());
        self
    }

    /// Set the `DUE` property
    ///
    /// See [`CalendarDateTime`] for info how are different [`chrono`] types converted automatically.
    pub fn due<T: Into<CalendarDateTime>>(&mut self, dt: T) -> &mut Self {
        let calendar_dt: CalendarDateTime = dt.into();
        self.add_property("DUE", &calendar_dt.to_string());
        self
    }

    /// Set the `COMPLETED` property
    ///
    /// Per [RFC 5545, Section 3.8.2.1](https://tools.ietf.org/html/rfc5545#section-3.8.2.1), this
    /// must be a date-time in UTC format.
    pub fn completed(&mut self, dt: DateTime<Utc>) -> &mut Self {
        self.add_property("COMPLETED", &CalendarDateTime::Utc(dt).to_string());
        self
    }

    ///  Defines the overall status or confirmation
    ///
    pub fn status(&mut self, status: TodoStatus) -> &mut Self {
        self.append_property(status.into());
        self
    }

    //pub fn repeats<R:Repeater+?Sized>(&mut self, repeat: R) -> &mut Self {
    //    unimplemented!()
    //}
}
