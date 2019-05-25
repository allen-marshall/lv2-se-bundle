//! Representation of LV2 dynamic manifest generator.

use crate::rdf_util::Iri;
use crate::bundle_model::{Loadable, Requirer, Identified};
use crate::bundle_model::symbol::Symbol;
use enumset::{EnumSet, EnumSetIter};
use crate::bundle_model::constants::HostFeature;
use rayon::iter::{IterBridge, ParallelBridge};

/// Representation of an LV2 dynamic manifest generator.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DynManifestInfo {
    /// IRI identifying the plugin.
    iri: Option<Iri>,

    /// URI pointing to the shared library that implements the dynamic manifest generator.
    binary: Iri,

    /// LV2 symbol identifying the dynamic manifest generator.
    symbol: Option<Symbol>,

    /// Host features required by the dynamic manifest generator. This set should not intersect with
    /// [`optional_host_features`](self::DynManifestInfo::optional_host_features).
    required_host_features: EnumSet<HostFeature>,

    /// Host features optionally supported by the dynamic manifest generator. This set should not
    /// intersect with [`required_host_features`](self::DynManifestInfo::required_host_features).
    optional_host_features: EnumSet<HostFeature>
}

impl Identified for DynManifestInfo {
    fn iri(&self) -> Option<&Iri> {
        self.iri.as_ref()
    }

    fn symbol(&self) -> Option<&Symbol> {
        self.symbol.as_ref()
    }
}

impl Loadable for DynManifestInfo {
    fn binary(&self) -> Option<&Iri> {
        Some(&self.binary)
    }
}

impl Requirer<HostFeature> for DynManifestInfo {
    type RequiredIter = IterBridge<EnumSetIter<HostFeature>>;
    type OptionallySupportedIter = IterBridge<EnumSetIter<HostFeature>>;

    fn required_iter(&self) -> Self::RequiredIter {
        self.required_host_features.iter().par_bridge()
    }

    fn optionally_supported_iter(&self) -> Self::OptionallySupportedIter {
        self.optional_host_features.iter().par_bridge()
    }
}