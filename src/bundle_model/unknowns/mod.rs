//! Representation of LV2 bundle data that isn't understood by this crate, but is deemed important
//! enough to represent.

// TODO: Maybe disallow construction of UnknownX when the IRI *is* understood by the crate?

use crate::bundle_model::IdentifiedBy;
use crate::rdf_util::Iri;

/// Represents a type of LV2 extension data interface that this crate doesn't understand.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UnknownExtensionData {
    /// Extension data is typically represented by an IRI, so this type is just a wrapper around
    /// [`Iri`](crate::rdf_util::Iri).
    iri: Iri
}

impl UnknownExtensionData {
    /// Constructs a representation of an unknown extension data interface.
    ///
    /// # Parameters
    /// - `iri`: IRI identifying the extension data interface.
    pub fn new(iri: Iri) -> UnknownExtensionData {
        UnknownExtensionData {
            iri
        }
    }
}

impl IdentifiedBy<Iri> for UnknownExtensionData {
    fn id(&self) -> &Iri {
        &self.iri
    }
}

/// Represents an LV2 host feature that this crate doesn't understand.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UnknownHostFeature {
    /// Host features are typically represented by an IRI, so this type is just a wrapper around
    /// [`Iri`](crate::rdf_util::Iri).
    iri: Iri
}

impl UnknownHostFeature {
    /// Constructs a representation of an unknown host feature.
    ///
    /// # Parameters
    /// - `iri`: IRI identifying the host feature.
    pub fn new(iri: Iri) -> UnknownHostFeature {
        UnknownHostFeature {
            iri
        }
    }
}

impl IdentifiedBy<Iri> for UnknownHostFeature {
    fn id(&self) -> &Iri {
        &self.iri
    }
}

/// Represents an LV2 option that this crate doesn't understand.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UnknownOption {
    /// LV2 options are typically represented by an IRI, so this type is just a wrapper around
    /// [`Iri`](crate::rdf_util::Iri).
    iri: Iri
}

impl UnknownOption {
    /// Constructs a representation of an unknown LV2 option.
    ///
    /// # Parameters
    /// - `iri`: IRI identifying the option.
    pub fn new(iri: Iri) -> UnknownOption {
        UnknownOption {
            iri
        }
    }
}

impl IdentifiedBy<Iri> for UnknownOption {
    fn id(&self) -> &Iri {
        &self.iri
    }
}

/// Represents an LV2 plugin type that this crate doesn't understand.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UnknownPluginType {
    /// LV2 plugin types are typically represented by an IRI, so this type is just a wrapper around
    /// [`Iri`](crate::rdf_util::Iri).
    iri: Iri
}

impl UnknownPluginType {
    /// Constructs a representation of an unknown LV2 plugin type.
    ///
    /// # Parameters
    /// - `iri`: IRI identifying the plugin type.
    pub fn new(iri: Iri) -> UnknownPluginType {
        UnknownPluginType {
            iri
        }
    }
}

impl IdentifiedBy<Iri> for UnknownPluginType {
    fn id(&self) -> &Iri {
        &self.iri
    }
}

/// Represents an LV2 port type that this crate doesn't understand.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UnknownPortType {
    /// LV2 port types are typically represented by an IRI, so this type is just a wrapper around
    /// [`Iri`](crate::rdf_util::Iri).
    iri: Iri
}

impl UnknownPortType {
    /// Constructs a representation of an unknown LV2 port type.
    ///
    /// # Parameters
    /// - `iri`: IRI identifying the port type.
    pub fn new(iri: Iri) -> UnknownPortType {
        UnknownPortType {
            iri
        }
    }
}

impl IdentifiedBy<Iri> for UnknownPortType {
    fn id(&self) -> &Iri {
        &self.iri
    }
}

/// Represents an LV2 port designation that this crate doesn't understand.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UnknownPortDesignation {
    /// LV2 port designations are typically represented by an IRI, so this type is just a wrapper
    /// around [`Iri`](crate::rdf_util::Iri).
    iri: Iri
}

impl UnknownPortDesignation {
    /// Constructs a representation of an unknown LV2 port designation.
    ///
    /// # Parameters
    /// - `iri`: IRI identifying the port designation.
    pub fn new(iri: Iri) -> UnknownPortDesignation {
        UnknownPortDesignation {
            iri
        }
    }
}

impl IdentifiedBy<Iri> for UnknownPortDesignation {
    fn id(&self) -> &Iri {
        &self.iri
    }
}