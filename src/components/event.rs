use super::*;
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
}

/// Things that have a start and and end
pub trait EventLike {
    /// Set the [`DTSTART`](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.2.4) [`Property`]
    ///
    /// See [`CalendarDateTime`] for info how are different [`chrono`] types converted automatically.
    fn starts<T: Into<CalendarDateTime>>(&mut self, dt: T) -> &mut Self;

    /// Set the [`DTEND`](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.2.2) [`Property`]
    ///
    /// See [`CalendarDateTime`] for info how are different [`chrono`] types converted automatically.
    fn ends<T: Into<CalendarDateTime>>(&mut self, dt: T) -> &mut Self;

    /// Set the [`DTSTART`](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.2.4) [`Property`], date only
    fn start_date<TZ: TimeZone>(&mut self, date: Date<TZ>) -> &mut Self
    where
        TZ::Offset: fmt::Display;

    /// Set the [`DTEND`](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.2.2) [`Property`], date only
    fn end_date<TZ: TimeZone>(&mut self, date: Date<TZ>) -> &mut Self
    where
        TZ::Offset: fmt::Display;

    /// Set the [`DTSTART`](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.2.4) [`Property`]
    /// and [`DTEND`](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.2.2) [`Property`],
    /// date only
    fn all_day<TZ: TimeZone>(&mut self, date: Date<TZ>) -> &mut Self
    where
        TZ::Offset: fmt::Display;
}

