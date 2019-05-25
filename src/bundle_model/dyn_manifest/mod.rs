//! Representation of LV2 dynamic manifest generator.

use crate::rdf_util::Iri;
use crate::bundle_model::{Loadable, Requirer, OptionallyIdentifiedBy};
use crate::bundle_model::symbol::Symbol;
use enumset::EnumSetIter;
use crate::bundle_model::constants::HostFeature;
use crate::bundle_model::unknowns::{UnknownHostFeature, UnknownOption};
use crate::bundle_model::impl_util::HostFeatureRequirer;
use rayon::iter::{IterBridge, IntoParallelRefIterator};
use std::collections::BTreeSet;

/// Representation of an LV2 dynamic manifest generator.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DynManifestInfo {
    /// IRI identifying the plugin.
    iri: Option<Iri>,

    /// LV2 symbol identifying the dynamic manifest generator.
    symbol: Option<Symbol>,

    /// URI pointing to the shared library that implements the dynamic manifest generator.
    binary: Iri,

    /// Information about required (and optional) host features and LV2 options.
    host_feature_requirer: HostFeatureRequirer
}

impl OptionallyIdentifiedBy<Iri> for DynManifestInfo {
    fn id(&self) -> Option<&Iri> {
        self.iri.as_ref()
    }
}

impl OptionallyIdentifiedBy<Symbol> for DynManifestInfo {
    fn id(&self) -> Option<&Symbol> {
        self.symbol.as_ref()
    }
}

impl Loadable for DynManifestInfo {
    fn binary(&self) -> Option<&Iri> {
        Some(&self.binary)
    }
}

impl<'a> Requirer<'a, HostFeature> for DynManifestInfo {
    type BorrowedElt = HostFeature;
    type RequiredIter = IterBridge<EnumSetIter<HostFeature>>;
    type OptionallySupportedIter = IterBridge<EnumSetIter<HostFeature>>;

    fn required_iter(&'a self) -> Self::RequiredIter {
        <HostFeatureRequirer as Requirer<'a, HostFeature>>::required_iter(&self.host_feature_requirer)
    }

    fn optionally_supported_iter(&'a self) -> Self::OptionallySupportedIter {
        <HostFeatureRequirer as Requirer<'a, HostFeature>>::optionally_supported_iter(&self.host_feature_requirer)
    }
}

impl<'a> Requirer<'a, UnknownHostFeature> for DynManifestInfo {
    type BorrowedElt = &'a UnknownHostFeature;
    type RequiredIter = <BTreeSet<UnknownHostFeature> as IntoParallelRefIterator<'a>>::Iter;
    type OptionallySupportedIter = <BTreeSet<UnknownHostFeature> as IntoParallelRefIterator<'a>>::Iter;

    fn required_iter(&'a self) -> Self::RequiredIter {
        <HostFeatureRequirer as Requirer<'a, UnknownHostFeature>>::required_iter(&self.host_feature_requirer)
    }

    fn optionally_supported_iter(&'a self) -> Self::OptionallySupportedIter {
        <HostFeatureRequirer as Requirer<'a, UnknownHostFeature>>::optionally_supported_iter(&self.host_feature_requirer)
    }
}

impl<'a> Requirer<'a, UnknownOption> for DynManifestInfo {
    type BorrowedElt = &'a UnknownOption;
    type RequiredIter = <BTreeSet<UnknownOption> as IntoParallelRefIterator<'a>>::Iter;
    type OptionallySupportedIter = <BTreeSet<UnknownOption> as IntoParallelRefIterator<'a>>::Iter;

    fn required_iter(&'a self) -> Self::RequiredIter {
        <HostFeatureRequirer as Requirer<'a, UnknownOption>>::required_iter(&self.host_feature_requirer)
    }

    fn optionally_supported_iter(&'a self) -> Self::OptionallySupportedIter {
        <HostFeatureRequirer as Requirer<'a, UnknownOption>>::optionally_supported_iter(&self.host_feature_requirer)
    }
}