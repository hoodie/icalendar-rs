use chrono::*;
use std::fmt;

use crate::{Property, ValueType};

const NAIVE_DATE_TIME_FORMAT: &str = "%Y%m%dT%H%M%S";
pub(crate) const UTC_DATE_TIME_FORMAT: &str = "%Y%m%dT%H%M%SZ";
pub(crate) const NAIVE_DATE_FORMAT: &str = "%Y%m%d";

/// Representation of various forms of `DATE-TIME` per
/// [RFC 5545, Section 3.3.5](https://tools.ietf.org/html/rfc5545#section-3.3.5)
///
/// Conversions from [chrono] types are provided in form of [From] implementations, see
/// documentation of individual variants.
///
/// In addition to readily implemented `FORM #1` and `FORM #2`, the RFC also specifies
/// `FORM #3: DATE WITH LOCAL TIME AND TIME ZONE REFERENCE`. This variant is not yet implemented.
/// Adding it will require adding support for `VTIMEZONE` and referencing it using `TZID`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
}

impl CalendarDateTime {
    pub(crate) fn from_str(s: &str) -> Option<Self> {
        if let Ok(naive_date_time) = NaiveDateTime::parse_from_str(s, NAIVE_DATE_TIME_FORMAT) {
            Some(naive_date_time.into())
        } else if let Ok(utc) = Utc.datetime_from_str(s, UTC_DATE_TIME_FORMAT) {
            Some(utc.into())
        } else {
            None
        }
    }
}

impl fmt::Display for CalendarDateTime {
    /// Format date-time in RFC 5545 compliant manner.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            CalendarDateTime::Floating(naive_dt) => naive_dt.format(NAIVE_DATE_TIME_FORMAT).fmt(f),
            CalendarDateTime::Utc(utc_dt) => utc_dt.format(UTC_DATE_TIME_FORMAT).fmt(f),
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DatePerhapsTime {
    DateTime(CalendarDateTime),
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
            Some(CalendarDateTime::from_str(property.value())?.into())
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
