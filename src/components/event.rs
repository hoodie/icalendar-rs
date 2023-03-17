use chrono::NaiveDateTime;
use std::cmp::Ordering;

use super::*;
/// VEVENT [(RFC 5545, Section 3.6.1 )](https://tools.ietf.org/html/rfc5545#section-3.6.1)
#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Event {
    pub(super) inner: InnerComponent,
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let this = self.get_start();
        let that = other.get_start();
        let tup = (this, that);
        match tup {
            (Some(this_start), Some(that_start)) => {
                let this_start: NaiveDateTime = this_start.into();
                let that_start: NaiveDateTime = that_start.into();
                Some(this_start.cmp(&that_start))
            }
            (Some(_), None) => Some(Ordering::Greater),
            (None, Some(_)) => Some(Ordering::Less),
            (_, _) => Some(Ordering::Equal),
        }
    }
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

    /// Defines the overall status or confirmation
    pub fn status(&mut self, status: EventStatus) -> &mut Self {
        self.append_property(status)
    }

    /// Gets the overall status or confirmation.
    pub fn get_status(&self) -> Option<EventStatus> {
        EventStatus::from_str(self.property_value("STATUS")?)
    }

    //pub fn repeats<R:Repeater+?Sized>(&mut self, repeat: R) -> &mut Self {
    //    unimplemented!()
    //}
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveTime;

    #[test]
    fn get_properties_unset() {
        let event = Event::new();
        assert_eq!(event.get_status(), None);
    }

    #[test]
    fn get_properties_set() {
        let event = Event::new().status(EventStatus::Tentative).done();
        assert_eq!(event.get_status(), Some(EventStatus::Tentative));
    }
    #[test]
    fn event_ord_test() {
        let mut event = Event::new();
        let mut event2 = Event::new();

        assert_eq!(event.cmp(&event2), Ordering::Equal);

        // NaiveDate on Jan 1st 2023
        let nd = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();

        // 10:00:00 am
        let nt = NaiveTime::from_hms_opt(10, 0, 0).unwrap();

        event.starts(nd);
        event2.starts(nd.and_time(nt));

        assert_eq!(event.cmp(&event2), Ordering::Less);

        let nt = NaiveTime::from_hms_opt(11, 0, 0).unwrap();
        event.starts(nd.and_time(nt));
        assert_eq!(event.cmp(&event2), Ordering::Greater);
    }
}
