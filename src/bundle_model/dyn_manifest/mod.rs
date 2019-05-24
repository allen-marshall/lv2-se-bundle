//! Representation of LV2 dynamic manifest generator.

use crate::rdf_util::Iri;

/// Representation of an LV2 dynamic manifest generator.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DynManifestInfo {
    /// IRI identifying the plugin.
    iri: Option<Iri>,

    /// URI pointing to the shared library that implements the dynamic manifest generator.
    binary: Iri,
}

impl DynManifestInfo {
    /// Gets the IRI that identifies the dynamic manifest generator. Returns
    /// [`None`](std::option::Option::None) if no IRI is specified for the generator.
    pub fn iri(&self) -> Option<&Iri> {
        self.iri.as_ref()
    }

    /// Gets a URI pointing to the shared library that implements the dynamic manifest generator. If
    /// the URI is relative, it should be interpreted relative to the bundle path.
    pub fn binary(&self) -> &Iri {
        &self.binary
    }
}