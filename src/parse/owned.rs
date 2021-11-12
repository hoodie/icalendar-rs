/// Owning version of [`crate::parse::components::Component`]
#[derive(PartialEq, Debug, Clone, Default)]
pub struct Calendar {
    pub components: Vec<Component>,
}

impl<'a> From<&'a Calendar> for crate::parse::Calendar<'a> {
    fn from(that: &'a Calendar) -> Self {
        crate::parse::Calendar {
            components: that.components.iter().map(Into::into).collect(),
        }
    }
}

impl From<&crate::parse::Calendar<'_>> for Calendar {
    fn from(that: &crate::parse::Calendar<'_>) -> Self {
        Calendar {
            components: that.components.iter().map(Into::into).collect(),
        }
    }
}

/// Owning version of [`crate::parse::components::Component`]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Component {
    pub name: String,
    pub properties: Vec<Property>,
    pub components: Vec<Component>,
}

impl<'a> From<&'a Component> for crate::parse::Component<'a> {
    fn from(that: &'a Component) -> Self {
        crate::parse::Component {
            name: that.name.as_str(),
            properties: that.properties.iter().map(Into::into).collect(),
            components: that.components.iter().map(Into::into).collect(),
        }
    }
}

impl From<&crate::parse::Component<'_>> for Component {
    fn from(that: &crate::parse::Component<'_>) -> Self {
        Component {
            name: that.name.to_owned(),
            properties: that.properties.iter().map(Into::into).collect(),
            components: that.components.iter().map(Into::into).collect(),
        }
    }
}

/// Owning version of [`crate::parse::properties::Property`]
#[derive(PartialEq, Debug, Clone)]
pub struct Property {
    pub key: String,
    pub val: String,
    pub params: Vec<Parameter>,
}

impl<'a> From<&'a Property> for crate::parse::Property<'a> {
    fn from(that: &'a Property) -> Self {
        crate::parse::Property {
            key: that.key.as_str(),
            val: that.val.as_str(),
            params: that.params.iter().map(Into::into).collect(),
        }
    }
}

impl From<&crate::parse::Property<'_>> for Property {
    fn from(that: &crate::parse::Property<'_>) -> Self {
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

impl<'a> From<&'a Parameter> for crate::parse::Parameter<'a> {
    fn from(that: &'a Parameter) -> Self {
        crate::parse::Parameter {
            key: that.key.as_str(),
            val: that.val.as_deref(),
        }
    }
}

impl From<&crate::parse::Parameter<'_>> for Parameter {
    fn from(that: &crate::parse::Parameter<'_>) -> Self {
        Parameter {
            key: that.key.to_owned(),
            val: that.val.map(ToOwned::to_owned),
        }
    }
}
