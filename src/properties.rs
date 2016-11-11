use std::fmt;
use std::mem;
use std::collections::HashMap;
use std::convert::Into;

#[derive(Debug)]
/// key-value pairs inside of `Property`s
pub struct Parameter {
    pub key: String,
    pub value: String,
}

impl Parameter {
    pub fn new(key: &str, val: &str) -> Self {
        Parameter {
            key: key.to_owned(),
            value: val.to_owned(),
        }
    }
}

//type EntryParameters = Vec<Parameter>;
type EntryParameters = HashMap<String,Parameter>;

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

    /// Clones the key field
    pub fn key(&self) -> String {
        self.key.clone()
    }

    pub fn append_parameter<I:Into<Parameter>>(&mut self, into_parameter: I) -> &mut Self {
        let parameter = into_parameter.into();
        self.parameters.insert(parameter.key.to_owned(), parameter);
        self
    }

    pub fn add_parameter(&mut self, key: &str, val: &str) -> &mut Self {
        self.append_parameter(Parameter::new(key, val));
        self
    }

    /// End of Builder Pattern
    pub fn done(&mut self) -> Self {
        Property {
            key: mem::replace(&mut self.key, String::new()),
            value: mem::replace(&mut self.value, String::new()),
            parameters: mem::replace(&mut self.parameters, HashMap::new()),
        }
    }

    /// Writes this Property to `out`
    pub fn fmt_write<W: fmt::Write>(&self, out: &mut W) -> Result<(), fmt::Error> {
        try!(write!(out, "{}", self.key));
        for (_key, &Parameter { ref key, ref value }) in &self.parameters {
            try!(write!(out, ";{}={}", key, value));
        }
        try!(writeln!(out, ":{}", self.value));
        Ok(())
    }
}

/// Defines: `Public`, `Private`, `Confidential`
pub enum Class {
    Public, Private, Confidential
}

impl Into<Property> for Class {
    fn into(self) -> Property {
        Property {
            key: String::from("CLASS"),
            value: String::from(match self {
                Class::Public => "PUBLIC",
                Class::Private => "PRIVATE",
                Class::Confidential => "CONFIDENTIAL",
            }),
            parameters: HashMap::new()
        }
    }
}

pub enum ValueType{
    Binary,
    Boolean,
    CalAddress,
    Date,
    DateTime,
    Duration,
    Float,
    Integer,
    Period,
    Recur,
    Text,
    Time,
    Uri,
    UtcOffset,
}


impl Into<Parameter> for ValueType {
    fn into(self) -> Parameter {
        Parameter {
            key: String::from("VALUE"),
            value: String::from(match self {
                ValueType::Binary     => "BINARY",
                ValueType::Boolean    => "BOOLEAN",
                ValueType::CalAddress => "CAL-ADDRESS",
                ValueType::Date       => "DATE",
                ValueType::DateTime   => "DATE-TIME",
                ValueType::Duration   => "DURATION",
                ValueType::Float      => "FLOAT",
                ValueType::Integer    => "INTEGER",
                ValueType::Period     => "PERIOD",
                ValueType::Recur      => "RECUR",
                ValueType::Text       => "TEXT",
                ValueType::Time       => "TIME",
                ValueType::Uri        => "URI",
                ValueType::UtcOffset  => "UTC-OFFSET"
            })
        }
    }
}
