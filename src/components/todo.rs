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

    /// Set the [`PERCENT-COMPLETE`](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.1.8) property
    ///
    /// Ranges between 0 - 100
    pub fn percent_complete(&mut self, percent: u8) -> &mut Self {
        self.add_property("PERCENT-COMPLETE", &percent.to_string());
        self
    }

    /// Gets the [`PERCENT-COMPLETE`](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.1.8) property.
    ///
    /// Ranges between 0 - 100.
    pub fn get_percent_complete(&self) -> Option<u8> {
        self.property_value("PERCENT-COMPLETE")?.parse().ok()
    }

    /// Set the [`DUE`](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.2.3) property
    ///
    /// See [`CalendarDateTime`] for info how are different [`chrono`] types converted automatically.
    pub fn due<T: Into<CalendarDateTime>>(&mut self, dt: T) -> &mut Self {
        let calendar_dt: CalendarDateTime = dt.into();
        self.add_property("DUE", &calendar_dt.to_string());
        self
    }

    /// Set the [`COMPLETED`](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.2.1) property
    ///
    /// Per [RFC 5545, Section 3.8.2.1](https://tools.ietf.org/html/rfc5545#section-3.8.2.1), this
    /// must be a date-time in UTC format.
    pub fn completed(&mut self, dt: DateTime<Utc>) -> &mut Self {
        self.add_property("COMPLETED", &CalendarDateTime::Utc(dt).to_string());
        self
    }

    /// Defines the overall status or confirmation
    pub fn status(&mut self, status: TodoStatus) -> &mut Self {
        self.append_property(status.into());
        self
    }

    /// Gets the overall status.
    pub fn get_status(&self) -> Option<TodoStatus> {
        TodoStatus::from_str(self.property_value("STATUS")?)
    }

    //pub fn repeats<R:Repeater+?Sized>(&mut self, repeat: R) -> &mut Self {
    //    unimplemented!()
    //}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_properties_unset() {
        let todo = Todo::new();
        assert_eq!(todo.get_percent_complete(), None);
        assert_eq!(todo.get_status(), None);
    }

    #[test]
    fn get_properties_set() {
        let todo = Todo::new()
            .percent_complete(42)
            .status(TodoStatus::NeedsAction)
            .done();
        assert_eq!(todo.get_percent_complete(), Some(42));
        assert_eq!(todo.get_status(), Some(TodoStatus::NeedsAction));
    }
}
