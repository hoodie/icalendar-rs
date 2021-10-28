use std::collections::HashMap;
use std::fmt::{self, Write};
use std::iter::{self, Once};

#[derive(Debug)]
/// key-value pairs inside of `Property`s
pub struct Parameter(String);

impl<S> From<S> for Parameter
where
    S: ToString,
{
    fn from(value: S) -> Self {
        Parameter(value.to_string())
    }
}

//type EntryParameters = Vec<Parameter>;
type EntryParameters = HashMap<String, Parameter>;

#[derive(Debug)]
/// key-value pairs inside of `Component`s
pub struct Property {
    value: String,
    parameters: EntryParameters,
}

impl IntoIterator for Property {
    type Item = Self;

    type IntoIter = Once<Self>;

    fn into_iter(self) -> Self::IntoIter {
        iter::once(self)
    }
}

impl<S> From<S> for Property
where
    S: ToString,
{
    fn from(value: S) -> Self {
        Property {
            value: value.to_string().replace('\n', "\\n"),
            parameters: HashMap::new(),
        }
    }
}

impl Property {
    /// The value of the Property
    pub fn value(&self) -> &str {
        let Property { value, .. } = self;
        value
    }

    /// Creates and appends a parameter.
    pub fn parameter<K, V>(self, key: K, val: V) -> Self
    where
        K: ToString,
        V: Into<Parameter>,
    {
        self.with_parameter_set((key.to_string(), val.into()))
    }

    /// Sets parameter from a tuple of (Key, Value)
    pub fn with_parameter_set<T>(mut self, parameter: T) -> Self
    where
        T: Into<(String, Parameter)>,
    {
        let Property { parameters, .. } = &mut self;
        let (key, value) = parameter.into();
        parameters.insert(key, value);
        self
    }

    /// Writes this Property to `out`
    pub fn fmt_write<W: Write>(&self, key: &str, out: &mut W) -> Result<(), fmt::Error> {
        // A nice starting capacity for the majority of content lines
        let mut line = String::with_capacity(150);

        let Property { value, parameters } = self;

        write!(line, "{}", key)?;
        for (key, Parameter(value)) in parameters {
            write!(line, ";{}={}", key, value)?;
        }
        write!(line, ":{}", value)?;
        write_crlf!(out, "{}", fold_line(&line))?;
        Ok(())
    }
}

/// Defines: `Public`, `Private`, `Confidential`
#[derive(Copy, Clone, Debug)]
pub enum Class {
    /// Public
    Public,
    /// Private
    Private,
    /// Confidential
    Confidential,
}

impl From<Class> for (String, Property) {
    fn from(val: Class) -> Self {
        (
            String::from("CLASS"),
            match val {
                Class::Public => "PUBLIC",
                Class::Private => "PRIVATE",
                Class::Confidential => "CONFIDENTIAL",
            }
            .into(),
        )
    }
}

/// see 8.3.4. [Value Data Types Registry](https://tools.ietf.org/html/rfc5545#section-8.3.4)
#[derive(Copy, Clone, Debug)]
pub enum ValueType {
    /// Binary
    Binary,
    /// Boolean
    Boolean,
    /// CalAddress
    CalAddress,
    /// Date
    Date,
    /// DateTime
    DateTime,
    /// Duration
    Duration,
    /// Float
    Float,
    /// Integer
    Integer,
    /// Period
    Period,
    /// Recur
    Recur,
    /// Text
    Text,
    /// Time
    Time,
    /// Uri
    Uri,
    /// UtcOffset
    UtcOffset,
}

impl From<ValueType> for (String, Parameter) {
    fn from(val: ValueType) -> Self {
        (
            String::from("VALUE"),
            match val {
                ValueType::Binary => "BINARY",
                ValueType::Boolean => "BOOLEAN",
                ValueType::CalAddress => "CAL-ADDRESS",
                ValueType::Date => "DATE",
                ValueType::DateTime => "DATE-TIME",
                ValueType::Duration => "DURATION",
                ValueType::Float => "FLOAT",
                ValueType::Integer => "INTEGER",
                ValueType::Period => "PERIOD",
                ValueType::Recur => "RECUR",
                ValueType::Text => "TEXT",
                ValueType::Time => "TIME",
                ValueType::Uri => "URI",
                ValueType::UtcOffset => "UTC-OFFSET",
            }
            .into(),
        )
    }
}

#[derive(Copy, Clone, Debug)]
/// Encodes the status of an `Event`
pub enum EventStatus {
    /// Indicates event is tentative.
    Tentative,
    /// Indicates event is definite.
    Confirmed,
    /// Indicates event was cancelled.
    Cancelled,
    //Custom(&str)
}

#[derive(Copy, Clone, Debug)]
/// Encodes the status of a `Todo`
pub enum TodoStatus {
    /// Indicates to-do needs action.
    NeedsAction,
    /// Indicates to-do is completed.
    Completed,
    /// Indicates to-do is in process.
    InProcess,
    /// Indicates to-do was cancelled.
    Cancelled,
    //Custom(&str)
}

//pub enum JournalStatuw{
//    Draft,
//    Final,
//    Cancelled,
//    Custom(&str)
//}

impl From<EventStatus> for (String, Property) {
    fn from(val: EventStatus) -> Self {
        (
            String::from("STATUS"),
            Property {
                value: String::from(match val {
                    EventStatus::Tentative => "TENTATIVE",
                    EventStatus::Confirmed => "CONFIRMED",
                    EventStatus::Cancelled => "CANCELLED",
                }),
                parameters: HashMap::new(),
            },
        )
    }
}

impl From<TodoStatus> for (String, Property) {
    fn from(val: TodoStatus) -> Self {
        (
            String::from("STATUS"),
            match val {
                TodoStatus::NeedsAction => "NEEDS-ACTION",
                TodoStatus::Completed => "COMPLETED",
                TodoStatus::InProcess => "IN-PROCESS",
                TodoStatus::Cancelled => "CANCELLED",
                //TodoStatus::Custom(s)   => "CU",
            }
            .into(),
        )
    }
}

//pub enum AttendeeRole {
//    /// CHAIR           (RFC 5545, Section 3.2.16)
//    Chair,
//
//    /// REQ-PARTICIPANT (RFC 5545, Section 3.2.16)
//    ReqParticipant,
//
//    /// OPT-PARTICIPANT (RFC 5545, Section 3.2.16)
//    OptParticipant,
//
//    /// NON-PARTICIPANT (RFC 5545, Section 3.2.16)
//    NonParticipant
//}
//
//pub struct Attendee {
//    cn: String,
//    role: AttendeeRole,
//    delegated_from: String,
//    partstat: String,
//    sent_by: String,
//    dir: String,
//}
//
//impl Into<Property> for Attendee {
//}

// Fold a content line as described in RFC 5545, Section 3.1
fn fold_line(line: &str) -> String {
    let limit = 75;
    let len = line.len();
    let mut ret = String::with_capacity(len + (len / limit * 3));
    let mut bytes_remaining = len;

    let mut pos = 0;
    let mut next_pos = limit;
    while bytes_remaining > limit {
        while !line.is_char_boundary(next_pos) {
            next_pos -= 1;
        }
        ret.push_str(&line[pos..next_pos]);
        ret.push_str("\r\n ");

        bytes_remaining -= next_pos - pos;
        pos = next_pos;
        next_pos += limit;
    }

    ret.push_str(&line[len - bytes_remaining..]);
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::string::String;

    #[test]
    fn fold_line_short() {
        let line = String::from("This is a short line");
        assert_eq!(line, fold_line(&line));
    }

    #[test]
    fn fold_line_folds_on_char_boundary() {
        let line = String::from(
            "Content lines shouldn't be folded in the middle \
             of a UTF-8 character. 老虎.",
        );
        let expected = String::from(
            "Content lines shouldn't be folded in the middle \
             of a UTF-8 character. 老\r\n 虎.",
        );
        assert_eq!(expected, fold_line(&line));
    }
}
