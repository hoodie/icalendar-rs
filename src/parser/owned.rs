/// Owning version of [`crate::parser::components::Component`]
#[derive(PartialEq, Debug, Clone, Default)]
pub struct Calendar {
    pub components: Vec<Component>,
}

impl<'a> From<&'a Calendar> for crate::parser::Calendar<'a> {
    fn from(that: &'a Calendar) -> Self {
        crate::parser::Calendar {
            components: that.components.iter().map(Into::into).collect(),
        }
    }
}

impl From<&crate::parser::Calendar<'_>> for Calendar {
    fn from(that: &crate::parser::Calendar<'_>) -> Self {
        Calendar {
            components: that.components.iter().map(Into::into).collect(),
        }
    }
}

/// Owning version of [`crate::parser::components::Component`]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Component {
    pub name: String,
    pub properties: Vec<Property>,
    pub components: Vec<Component>,
}

impl<'a> From<&'a Component> for crate::parser::Component<'a> {
    fn from(that: &'a Component) -> Self {
        crate::parser::Component {
            name: that.name.as_str(),
            properties: that.properties.iter().map(Into::into).collect(),
            components: that.components.iter().map(Into::into).collect(),
        }
    }
}

impl From<&crate::parser::Component<'_>> for Component {
    fn from(that: &crate::parser::Component<'_>) -> Self {
        Component {
            name: that.name.to_owned(),
            properties: that.properties.iter().map(Into::into).collect(),
            components: that.components.iter().map(Into::into).collect(),
        }
    }
}

/// Owning version of [`crate::parser::properties::Property`]
#[derive(PartialEq, Debug, Clone)]
pub struct Property {
    pub key: String,
    pub val: String,
    pub params: Vec<Parameter>,
}

impl<'a> From<&'a Property> for crate::parser::Property<'a> {
    fn from(that: &'a Property) -> Self {
        crate::parser::Property {
            key: that.key.as_str(),
            val: that.val.as_str(),
            params: that.params.iter().map(Into::into).collect(),
        }
    }
}

impl From<&crate::parser::Property<'_>> for Property {
    fn from(that: &crate::parser::Property<'_>) -> Self {
        Property {
            key: that.key.to_owned(),
            val: that.val.to_owned(),
            params: that.params.iter().map(Into::into).collect(),
        }
    }
}

/// Owning version of [`crate::parameters::Parameter`]
#[derive(PartialEq, Debug, Clone)]
pub struct Parameter {
    pub key: String,
    pub val: Option<String>,
}

impl<'a> From<&'a Parameter> for crate::parser::Parameter<'a> {
    fn from(that: &'a Parameter) -> Self {
        crate::parser::Parameter {
            key: that.key.as_str(),
            val: that.val.as_deref(),
        }
    }
}

impl From<&crate::parser::Parameter<'_>> for Parameter {
    fn from(that: &crate::parser::Parameter<'_>) -> Self {
        Parameter {
            key: that.key.to_owned(),
            val: that.val.map(ToOwned::to_owned),
        }
    }
}
