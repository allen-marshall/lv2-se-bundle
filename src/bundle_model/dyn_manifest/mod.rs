//! Representation of LV2 dynamic manifest generator.

use crate::rdf_util::Iri;
use crate::bundle_model::LoadableEntity;
use crate::bundle_model::symbol::Symbol;

/// Representation of an LV2 dynamic manifest generator.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DynManifestInfo {
    /// IRI identifying the plugin.
    iri: Option<Iri>,

    /// URI pointing to the shared library that implements the dynamic manifest generator.
    binary: Iri,

    /// LV2 symbol identifying the dynamic manifest generator.
    symbol: Option<Symbol>
}

impl LoadableEntity for DynManifestInfo {
    fn iri(&self) -> Option<&Iri> {
        self.iri.as_ref()
    }

    fn binary(&self) -> Option<&Iri> {
        Some(&self.binary)
    }

    fn symbol(&self) -> Option<&Symbol> {
        self.symbol.as_ref()
    }
}