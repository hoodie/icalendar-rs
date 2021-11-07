use std::{
    collections::HashMap,
    fmt::{self, Write},
    mem,
};

#[derive(Debug)]
/// key-value pairs inside of `Property`s
pub struct Parameter {
    key: String,
    value: String,
}

impl Parameter {
    /// Creates a new `Parameter`
    pub fn new(key: &str, val: &str) -> Self {
        Parameter {
            key: key.to_owned(),
            value: val.to_owned(),
        }
    }
}

//type EntryParameters = Vec<Parameter>;
pub type EntryParameters = HashMap<String, Parameter>;

#[derive(Debug)]
/// key-value pairs inside of `Component`s
pub struct Property {
    key: String,
    value: String,
    parameters: EntryParameters,
}

impl Property {
    /// Guess what this does :D
    pub fn new(key: &str, val: &str) -> Self {
        Property {
            key: key.to_owned(),
            value: val.replace('\n', "\\n"),
            parameters: HashMap::new(),
        }
    }

    /// Clones the key field.
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Clones the key field.
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Appends a new parameter.
    pub fn append_parameter<I: Into<Parameter>>(&mut self, into_parameter: I) -> &mut Self {
        let parameter = into_parameter.into();
        self.parameters.insert(parameter.key.clone(), parameter);
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
            value: mem::take(&mut self.value),
            parameters: mem::take(&mut self.parameters),
        }
    }

    /// Writes this Property to `out`
    pub fn fmt_write<W: Write>(&self, out: &mut W) -> Result<(), fmt::Error> {
        // A nice starting capacity for the majority of content lines
        let mut line = String::with_capacity(150);

        write!(line, "{}", self.key)?;
        for &Parameter { ref key, ref value } in self.parameters.values() {
            write!(line, ";{}={}", key, value)?;
        }
        write!(line, ":{}", self.value)?;
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
            value: String::from(match val {
                Class::Public => "PUBLIC",
                Class::Private => "PRIVATE",
                Class::Confidential => "CONFIDENTIAL",
            }),
            parameters: HashMap::new(),
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
            value: String::from(match val {
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

impl From<EventStatus> for Property {
    fn from(val: EventStatus) -> Self {
        Property {
            key: String::from("STATUS"),
            value: String::from(match val {
                EventStatus::Tentative => "TENTATIVE",
                EventStatus::Confirmed => "CONFIRMED",
                EventStatus::Cancelled => "CANCELLED",
            }),
            parameters: HashMap::new(),
        }
    }
}

impl From<TodoStatus> for Property {
    fn from(val: TodoStatus) -> Self {
        Property {
            key: String::from("STATUS"),
            value: String::from(match val {
                TodoStatus::NeedsAction => "NEEDS-ACTION",
                TodoStatus::Completed => "COMPLETED",
                TodoStatus::InProcess => "IN-PROCESS",
                TodoStatus::Cancelled => "CANCELLED",
                //TodoStatus::Custom(s)   => "CU",
            }),
            parameters: HashMap::new(),
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
