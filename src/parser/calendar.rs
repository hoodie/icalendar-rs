use crate::calendar::CalendarComponent;

use super::{components::LikeComponent, read_calendar, unfold, Component, Property};
use core::{fmt, str::FromStr};

/// Helper-type for reserialization
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Calendar<'a> {
    pub properties: Vec<Property<'a>>,
    pub components: Vec<Component<'a>>,
}

impl Calendar<'_> {
    /// Prints to stdout
    pub fn print(&self) -> Result<(), fmt::Error> {
        print_crlf!("{}", self);
        Ok(())
    }
}
impl<'a> LikeComponent<'a> for Calendar<'a> {
    fn name(&self) -> &str {
        const CALENDAR_NAME: &str = "VCALENDAR";
        CALENDAR_NAME
    }

    fn properties(&self) -> &[Property<'a>] {
        &self.properties
    }

    fn components(&self) -> &[Component<'a>] {
        &self.components
    }
}

impl fmt::Display for Calendar<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_write(f)
    }
}

impl From<Calendar<'_>> for crate::Calendar {
    fn from(parsed: Calendar) -> Self {
        parsed.components.into()
    }
}

impl<'a> From<Vec<Component<'a>>> for crate::Calendar {
    fn from(mut components: Vec<Component<'a>>) -> Self {
        let root_is_calendar = components
            .get(0)
            .map(|first_root| first_root.name == "VCALENDAR")
            .unwrap_or(false);

        let components: Vec<Component<'a>> = if root_is_calendar {
            components.swap_remove(0).components
        } else {
            components
        };
        components
            .into_iter()
            .map(|c: Component<'a>| {
                let elem: CalendarComponent = c.into();
                elem
            })
            .collect()
    }
}

impl FromStr for crate::Calendar {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let from_parsed = crate::Calendar::from(read_calendar(&unfold(s))?);
        Ok(from_parsed)
    }
}
