use chrono::*;

use crate::{Property, ValueType};

const NAIVE_DATE_TIME_FORMAT: &str = "%Y%m%dT%H%M%S";
const UTC_DATE_TIME_FORMAT: &str = "%Y%m%dT%H%M%SZ";
const NAIVE_DATE_FORMAT: &str = "%Y%m%d";

pub(crate) fn parse_utc_date_time(s: &str) -> Option<DateTime<Utc>> {
    Utc.datetime_from_str(s, UTC_DATE_TIME_FORMAT).ok()
}

pub(crate) fn format_utc_date_time(utc_dt: DateTime<Utc>) -> String {
    utc_dt.format(UTC_DATE_TIME_FORMAT).to_string()
}

pub(crate) fn naive_date_to_property(date: NaiveDate, key: &str) -> Property {
    Property::new(key, &date.format(NAIVE_DATE_FORMAT).to_string())
        .append_parameter(ValueType::Date)
        .done()
}

/// Representation of various forms of `DATE-TIME` per
/// [RFC 5545, Section 3.3.5](https://tools.ietf.org/html/rfc5545#section-3.3.5)
///
/// Conversions from [chrono] types are provided in form of [From] implementations, see
/// documentation of individual variants.
///
/// In addition to readily implemented `FORM #1` and `FORM #2`, the RFC also specifies
/// `FORM #3: DATE WITH LOCAL TIME AND TIME ZONE REFERENCE`. This variant is not yet implemented.
/// Adding it will require adding support for `VTIMEZONE` and referencing it using `TZID`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CalendarDateTime {
    /// `FORM #1: DATE WITH LOCAL TIME`: floating, follows current time-zone of the attendee.
    ///
    /// Conversion from [`chrono::NaiveDateTime`] results in this variant.
    Floating(NaiveDateTime),
    /// `FORM #2: DATE WITH UTC TIME`: rendered with Z suffix character.
    ///
    /// Conversion from [`chrono::DateTime<Utc>`](DateTime) results in this variant. Use
    /// `date_time.with_timezone(&Utc)` to convert `date_time` from arbitrary time zone to UTC.
    Utc(DateTime<Utc>),
    /// `FORM #3: DATE WITH LOCAL TIME AND TIME ZONE REFERENCE`: refers to a time zone definition.
    WithTimezone {
        /// The date and time in the given time zone.
        date_time: NaiveDateTime,
        /// The ID of the time zone definition in a VTIMEZONE calendar component.
        tzid: String,
    },
}

impl CalendarDateTime {
    pub(crate) fn from_property(property: &Property) -> Option<Self> {
        let value = property.value();
        if let Some(tzid) = property.params().get("TZID") {
            Some(Self::WithTimezone {
                date_time: NaiveDateTime::parse_from_str(value, NAIVE_DATE_TIME_FORMAT).ok()?,
                tzid: tzid.value().to_owned(),
            })
        } else if let Ok(naive_date_time) =
            NaiveDateTime::parse_from_str(value, NAIVE_DATE_TIME_FORMAT)
        {
            Some(naive_date_time.into())
        } else {
            parse_utc_date_time(value).map(Into::into)
        }
    }

    pub(crate) fn to_property(&self, key: &str) -> Property {
        match self {
            CalendarDateTime::Floating(naive_dt) => {
                Property::new(key, &naive_dt.format(NAIVE_DATE_TIME_FORMAT).to_string())
            }
            CalendarDateTime::Utc(utc_dt) => Property::new(key, &format_utc_date_time(*utc_dt)),
            CalendarDateTime::WithTimezone { date_time, tzid } => {
                Property::new(key, &date_time.format(NAIVE_DATE_TIME_FORMAT).to_string())
                    .add_parameter("TZID", tzid)
                    .done()
            }
        }
    }
}

/// Converts from time zone-aware UTC date-time to [`CalendarDateTime::Utc`].
impl From<DateTime<Utc>> for CalendarDateTime {
    fn from(dt: DateTime<Utc>) -> Self {
        Self::Utc(dt)
    }
}

/// Converts from time zone-less date-time to [`CalendarDateTime::Floating`].
impl From<NaiveDateTime> for CalendarDateTime {
    fn from(dt: NaiveDateTime) -> Self {
        Self::Floating(dt)
    }
}

/// Either a `DATE-TIME` or a `DATE`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DatePerhapsTime {
    /// A `DATE-TIME` property.
    DateTime(CalendarDateTime),
    /// A `DATE` property.
    Date(NaiveDate),
}

impl DatePerhapsTime {
    pub(crate) fn from_property(property: &Property) -> Option<Self> {
        if property.value_type() == Some(ValueType::Date) {
            Some(
                NaiveDate::parse_from_str(property.value(), NAIVE_DATE_FORMAT)
                    .ok()?
                    .into(),
            )
        } else {
            Some(CalendarDateTime::from_property(property)?.into())
        }
    }

    pub(crate) fn to_property(&self, key: &str) -> Property {
        match self {
            Self::DateTime(date_time) => date_time.to_property(key),
            Self::Date(date) => naive_date_to_property(*date, key),
        }
    }
}

impl From<CalendarDateTime> for DatePerhapsTime {
    fn from(dt: CalendarDateTime) -> Self {
        Self::DateTime(dt)
    }
}

impl From<DateTime<Utc>> for DatePerhapsTime {
    fn from(dt: DateTime<Utc>) -> Self {
        Self::DateTime(dt.into())
    }
}

impl From<NaiveDateTime> for DatePerhapsTime {
    fn from(dt: NaiveDateTime) -> Self {
        Self::DateTime(dt.into())
    }
}

impl From<NaiveDate> for DatePerhapsTime {
    fn from(date: NaiveDate) -> Self {
        Self::Date(date)
    }
}
