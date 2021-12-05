use super::*;

#[derive(Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Clone))]
pub struct Other {
    pub(crate) name: String,
    pub(crate) inner: InnerComponent,
}

impl Component for Other {
    /// Tells you what kind of `Component` this is
    ///
    /// Might be `VEVENT`, `VTODO`, `VALARM` etc
    fn component_kind(&self) -> String {
        self.name.clone()
    }

    /// Read-only access to `properties`
    fn properties(&self) -> &BTreeMap<String, Property> {
        &self.inner.properties
    }

    /// Read-only access to `multi_properties`
    fn multi_properties(&self) -> &Vec<Property> {
        &self.inner.multi_properties
    }

    /// Adds a `Property`
    fn append_property(&mut self, property: Property) -> &mut Self {
        self.inner
            .properties
            .insert(property.key().to_owned(), property);
        self
    }

    /// Adds a `Property` of which there may be many
    fn append_multi_property(&mut self, property: Property) -> &mut Self {
        self.inner.multi_properties.push(property);
        self
    }
}

impl From<(String, InnerComponent)> for Other {
    fn from((name, inner): (String, InnerComponent)) -> Self {
        Self { name, inner }
    }
}
