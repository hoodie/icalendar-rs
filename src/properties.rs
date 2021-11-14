use std::{
    collections::HashMap,
    fmt::{self, Write},
    mem,
};

#[derive(Debug, PartialEq, Eq)]
/// key-value pairs inside of `Property`s
pub struct Parameter {
    key: String,
    val: String,
}

impl Parameter {
    /// Creates a new `Parameter`
    pub fn new(key: &str, val: &str) -> Self {
        Parameter {
            key: key.to_owned(),
            val: val.to_owned(),
        }
    }
}

//type EntryParameters = Vec<Parameter>;
pub type EntryParameters = HashMap<String, Parameter>;

#[derive(Debug, PartialEq, Eq)]
/// key-value pairs inside of `Component`s
pub struct Property {
    key: String,
    val: String,
    params: EntryParameters,
}

impl Property {
    /// Guess what this does :D
    pub fn new(key: &str, val: &str) -> Self {
        Property {
            key: key.to_owned(),
            val: val.replace('\n', "\\n"),
            params: HashMap::new(),
        }
    }

    /// Clones the key field.
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Clones the key field.
    pub fn value(&self) -> &str {
        &self.val
    }

    /// Appends a new parameter.
    pub fn append_parameter<I: Into<Parameter>>(&mut self, into_parameter: I) -> &mut Self {
        let parameter = into_parameter.into();
        self.params.insert(parameter.key.clone(), parameter);
        self
    }

    /// Creates and appends a parameter.
    pub fn add_parameter(&mut self, key: &str, val: &str) -> &mut Self {
        self.append_parameter(Parameter::new(key, val));
        self
    }

    /// End of Builder Pattern.
    pub fn done(&mut self) -> Self {
        Property {
            key: mem::take(&mut self.key),
            val: mem::take(&mut self.val),
            params: mem::take(&mut self.params),
        }
    }

    /// Writes this Property to `out`
    pub fn fmt_write<W: Write>(&self, out: &mut W) -> Result<(), fmt::Error> {
        // A nice starting capacity for the majority of content lines
        let mut line = String::with_capacity(150);

        write!(line, "{}", self.key)?;
        for &Parameter {
            ref key,
            val: ref value,
        } in self.params.values()
        {
            write!(line, ";{}={}", key, value)?;
        }
        write!(line, ":{}", self.val)?;
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

impl From<Class> for Property {
    fn from(val: Class) -> Self {
        Property {
            key: String::from("CLASS"),
            val: String::from(match val {
                Class::Public => "PUBLIC",
                Class::Private => "PRIVATE",
                Class::Confidential => "CONFIDENTIAL",
            }),
            params: HashMap::new(),
        }
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

impl From<ValueType> for Parameter {
    fn from(val: ValueType) -> Self {
        Parameter {
            key: String::from("VALUE"),
            val: String::from(match val {
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
            }),
        }
    }
}

#[derive(Copy, Clone, Debug)]
/// Encodes the status of an `Event`
/// <https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.1.11>
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
/// <https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.1.11>
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

impl From<EventStatus> for Property {
    fn from(val: EventStatus) -> Self {
        Property {
            key: String::from("STATUS"),
            val: String::from(match val {
                EventStatus::Tentative => "TENTATIVE",
                EventStatus::Confirmed => "CONFIRMED",
                EventStatus::Cancelled => "CANCELLED",
            }),
            params: HashMap::new(),
        }
    }
}

impl From<TodoStatus> for Property {
    fn from(val: TodoStatus) -> Self {
        Property {
            key: String::from("STATUS"),
            val: String::from(match val {
                TodoStatus::NeedsAction => "NEEDS-ACTION",
                TodoStatus::Completed => "COMPLETED",
                TodoStatus::InProcess => "IN-PROCESS",
                TodoStatus::Cancelled => "CANCELLED",
                //TodoStatus::Custom(s)   => "CU",
            }),
            params: HashMap::new(),
        }
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
pub(crate) fn fold_line(line: &str) -> String {
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
