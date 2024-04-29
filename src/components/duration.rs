use std::{fmt::{self, Display}, ops::Neg, str::FromStr};

use crate::Property;


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Duration {
    positive: bool,
    weeks: u64,
    days: u64,
    hours: u64,
    minutes: u64,
    seconds: u64
}

impl Default for Duration {
    fn default() -> Self {
        Self {
            positive: true,
            weeks:0,
            days:0,
            hours:0,
            minutes:0,
            seconds:0
        }
    }
}

impl Duration {
    pub fn new(positive: bool, weeks: u64, days: u64, hours: u64, minutes: u64, seconds: u64) -> Self {
        Self{positive, weeks, days, hours, minutes, seconds}
    }
    
    pub fn weeks(weeks: u64) -> Self {
        Self {
            weeks,
            ..Self::default()
        }
    }

    pub fn days(days: u64) -> Self {
        Self {
            days,
            ..Self::default()
        }
    }

    pub fn hours(hours:u64) -> Self {
        Self {
            hours,
            ..Self::default()
        }
    }

    pub fn minutes(minutes: u64) -> Self {
        Self {
            minutes,
            ..Self::default()
        }
    }

    pub fn seconds(seconds: u64) -> Self {
        Self {
            seconds,
            ..Self::default()
        }
    }

    pub fn and_weeks(mut self, weeks: u64) -> Self {
        self.weeks = weeks;
        self
    }

    pub fn and_days(mut self, days: u64) -> Self {
        self.days = days;
        self
    }

    pub fn and_hours(mut self, hours: u64) -> Self {
        self.hours = hours;
        self
    }

    pub fn and_minutes(mut self, minutes: u64) -> Self {
        self.minutes = minutes;
        self
    }

    pub fn and_seconds(mut self, seconds: u64) -> Self {
        self.seconds = seconds;
        self
    }
}

impl Neg for Duration {
    type Output = Duration;

    fn neg(self) -> Self::Output {
        Self {
            positive: !self.positive,
            ..self
        }
    }
}

impl FromStr for Duration {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(())
        }
        let mut positiv:bool = true;

        let mut it = s.chars();
        let first = it.next();
        if let Some(first) = first {
            if first == '-' {
                positiv = false;
            } 
        }
        
        let parsed = if positiv {
            iso8601::duration(s)
        } else {
            iso8601::duration(it.as_str())
        };

        if let Ok(parsed) = parsed {
            let parsed = Duration::try_from(parsed)?;

            if !positiv {
                Ok(-parsed)
            } else {
                Ok(parsed)
            }
            
        } else {
            Err(())
        }
    }
    
}

impl TryFrom<iso8601::Duration> for Duration {
    type Error = ();

    fn try_from(value: iso8601::Duration) -> Result<Self, Self::Error> {
        // What happens if the format is P1W1D? 
        // I think this didnt work before either
        match value {
            iso8601::Duration::YMDHMS { year, month, day, hour, minute, second, millisecond: _ } => {
                if (year | month ) > 0 {
                    // Its not allowed to have year or month specifiers for ics files 
                    // https://icalendar.org/iCalendar-RFC-5545/3-3-6-duration.html
                    return Err(())
                } 
                Ok(Self::new(true, 0, day as u64, hour as u64, minute as u64, second as u64))
            },
            iso8601::Duration::Weeks(weeks) => Ok(Self::weeks(weeks as u64)),
        }          
    }
}

impl From<Duration> for Property {
    fn from(value: Duration) -> Self {
        Property::new_pre_alloc("DURATION".into(), value.to_string())
    }
}


impl Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.positive {
            f.write_str("-")?;
        }
        f.write_str("P")?;

        if self.weeks > 0 {
            f.write_fmt(format_args!("{}W", self.weeks))?;
        }
        if self.days > 0 {
            f.write_fmt(format_args!("{}D", self.days))?;
        }
        if (self.hours | self.minutes | self.seconds) > 0 {
            f.write_str("T")?;
            if self.hours > 0 {
                f.write_fmt(format_args!("{}H", self.hours))?;
            }
            if self.minutes > 0 {
                f.write_fmt(format_args!("{}M", self.minutes))?;
            }
            if self.seconds > 0 {
                f.write_fmt(format_args!("{}S", self.seconds))?;
            }
        }

        Ok(())
    }
}

#[test]
fn test_duration_parse_full_positiv() {
    let dur_str = "P1DT1H1M1S";
    
    pretty_assertions::assert_eq!(
        Duration::from_str(dur_str).unwrap().to_string(),
        dur_str
    );
}

#[test]
fn test_duration_parse_full_negativ() {
    let dur_str = "-P1DT1H1M1S";

    pretty_assertions::assert_eq!(
        Duration::from_str(dur_str).unwrap().to_string(),
        dur_str
    );
}

#[test]
fn test_duration_create_and_weeks() {
    let dur = Duration::days(1).and_weeks(1);
    
    pretty_assertions::assert_eq!(
        dur.to_string(), "P1W1D"
    );
}