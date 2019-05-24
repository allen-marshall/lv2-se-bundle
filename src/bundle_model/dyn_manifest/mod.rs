//! Representation of LV2 dynamic manifest generator.

use crate::rdf_util::Iri;
use crate::bundle_model::{LoadableEntity, HostFeatureSupporter};
use crate::bundle_model::symbol::Symbol;
use enumset::{EnumSet, EnumSetIter};
use crate::bundle_model::constants::HostFeature;

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

impl HostFeatureSupporter for DynManifestInfo {
    type RequiredHostFeaturesIter = EnumSetIter<HostFeature>;

    type OptionalHostFeaturesIter = EnumSetIter<HostFeature>;

    fn required_host_features_iter(&self) -> Self::RequiredHostFeaturesIter {
        self.required_host_features.iter()
    }

    fn optional_host_features_iter(&self) -> Self::OptionalHostFeaturesIter {
        self.optional_host_features.iter()
    }
}