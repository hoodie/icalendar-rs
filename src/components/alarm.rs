use super::*;

/// VALARM [(RFC 5545, Section 3.6.6 )](https://tools.ietf.org/html/rfc5545#section-3.6.6)
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Alarm {
    pub(crate) inner: InnerComponent,
    // pub(crate) action: Option<Action>,
}

impl Alarm {
    /// Creates a new Event.
    pub fn new() -> Self {
        Default::default()
    }

    /// add the `ACTION` property
    pub fn action(&mut self, action: Action) -> &mut Self {
        // self.add_property("ACTION", action.as_str());
        self.append_property(action.into());
        self
    }

    /// add the `ACTION` property
    pub fn trigger(&mut self, trigger: Trigger) -> &mut Self {
        self.append_property(trigger.into());
        self
    }

    /// Set the [`ACKNOWLEDGED`]() property
    pub fn acknowledged(&mut self, dt: DateTime<Utc>) -> &mut Self {
        self.add_property("ACKNOWLEDGED", &CalendarDateTime::Utc(dt).to_string());
        self
    }

    /// End of builder pattern.
    /// copies over everything
    pub fn done(&mut self) -> Self {
        Alarm {
            inner: self.inner.done(),
            // TODO: add default action = None
        }
    }

    //pub fn repeats<R:Repeater+?Sized>(&mut self, repeat: R) -> &mut Self {
    //    unimplemented!()
    //}
}

// impl std::Str
