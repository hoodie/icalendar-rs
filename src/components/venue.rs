use super::*;

/// VVENUE  [(ical-venue)](https://tools.ietf.org/html/draft-norris-ical-venue-01)
#[derive(Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Clone))]
pub struct Venue {
    pub(crate) inner: InnerComponent,
}
impl Venue {
    /// Creates a new Venue.
    pub fn new() -> Self {
        Default::default()
    }

    /// End of builder pattern.
    /// copies over everything
    pub fn done(&mut self) -> Self {
        Venue {
            inner: self.inner.done(),
        }
    }

    /// Set the STREET-ADDRESS `Property`
    ///
    /// This specifies the street address of a location. If the location requires a multiple-line
    /// address, they may be separated by an encoded newline "\n".
    pub fn street_address(&mut self, address: &str) -> &mut Self {
        self.add_property("STREET-ADDRESS", address)
    }

    /// Set the EXTENDED-ADDRESS `Property`
    ///
    /// This property provides the opportunity to include extended address information for a
    /// location. This property may be used to give additional information about an address that is
    /// not usually considered part of the street address. If the location requires a multiple-line
    /// address, they may be separated by an encoded newline "\n".
    pub fn extended_address(&mut self, address: &str) -> &mut Self {
        self.add_property("EXTENDED-ADDRESS", address)
    }

    /// Set the LOCALITY `Property`
    ///
    /// This specifies the city or locality of a venue.
    pub fn locality(&mut self, locality: &str) -> &mut Self {
        self.add_property("LOCALITY", locality)
    }

    /// Set the REGION `Property`
    ///
    /// This specifies the region (state, province, canton, etc.) of a location.
    pub fn region(&mut self, region: &str) -> &mut Self {
        self.add_property("REGION", region)
    }

    /// Set the COUNTRY `Property`
    ///
    /// This specifies the country of a location.
    pub fn country(&mut self, country: &str) -> &mut Self {
        self.add_property("COUNTRY", country)
    }

    /// Set the POSTAL-CODE `Property`
    ///
    /// This specifies the postal code of a location.
    pub fn postal_code(&mut self, postal_code: &str) -> &mut Self {
        self.add_property("POSTAL-CODE", postal_code)
    }
}
