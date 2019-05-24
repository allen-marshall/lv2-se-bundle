//! Defines data structures representing information available in an LV2 bundle's RDF data.

use num_bigint::BigUint;

pub mod constants;
pub mod port;
pub mod plugin;

/// Represents a version specification for an LV2 resource, such as a plugin. A resource version
/// consists of a minor version number and a micro version number. There is no major version number,
/// because LV2 uses the resource's identification IRI instead of a version number to track
/// non-backwards-compatible changes. A value of zero for the minor version indicates a pre-release.
/// An odd value for either the minor version or the micro version indicates a development release.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ResourceVersion {
    minor_version: BigUint,
    micro_version: BigUint
}

impl ResourceVersion {
    /// Constructs a resource version object.
    ///
    /// # Parameters
    /// - `minor_version`: The minor version number.
    /// - `micro_version`: The micro version number.
    pub fn new(minor_version: BigUint, micro_version: BigUint) -> Self {
        ResourceVersion {
            minor_version,
            micro_version
        }
    }

    /// Gets the minor version number.
    pub fn minor_version(&self) -> &BigUint {
        &self.minor_version
    }

    /// Gets the micro version number.
    pub fn micro_version(&self) -> &BigUint {
        &self.micro_version
    }

    /// Checks if this version represents a pre-release.
    pub fn is_pre_release(&self) -> bool {
        self.minor_version == 0u8.into()
    }

    /// Checks if this version represents a development release other than a pre-release.
    pub fn is_dev_release(&self) -> bool {
        // Check if the minor version or micro version is odd.
        &self.minor_version % &2u8 == 1u8.into() || &self.micro_version % &2u8 == 1u8.into()
    }
}