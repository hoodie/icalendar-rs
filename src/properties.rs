use std::fmt;
use std::mem;

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

type EntryParameters = Vec<Parameter>;
// type EntryParameters = HashMap<EPKey,String>;

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
            parameters: Vec::new(),
        }
    }

    /// Clones the key field
    pub fn key(&self) -> String {
        self.key.clone()
    }

    /// Builder method, adds a new `Parameter`
    pub fn parameter(&mut self, key: &str, val: &str) -> &mut Self {
        self.parameters.push(Parameter::new(key, val));
        self
    }

    /// End of Builder Pattern
    pub fn done(&mut self) -> Self {
        Property {
            key: mem::replace(&mut self.key, String::new()),
            value: mem::replace(&mut self.value, String::new()),
            parameters: mem::replace(&mut self.parameters, Vec::new()),
        }
    }

    /// Writes this Property to `out`
    pub fn fmt_write<W: fmt::Write>(&self, out: &mut W) -> Result<(), fmt::Error> {
        try!(write!(out, "{}", self.key));
        for &Parameter { ref key, ref value } in &self.parameters {
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

use std::convert::Into;
impl Into<Property> for Class {
    fn into(self) -> Property {
        Property {
            key: String::from("CLASS"),
            value: String::from(match self {
                Class::Public => "PUBLIC",
                Class::Private => "PRIVATE",
                Class::Confidential => "CONFIDENTIAL",
            }),
            parameters: Vec::new()
        }
    }
}

