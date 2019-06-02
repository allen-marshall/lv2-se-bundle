//! Representation of LV2 dynamic manifest generator.

use crate::rdf_util::Iri;
use crate::bundle_model::{Loadable, OptionallyIdentifiedBy, HasRelatedSet, RequiresRelation, OptionallySupportsRelation};
use crate::bundle_model::symbol::Symbol;
use enumset::EnumSetIter;
use crate::bundle_model::constants::{HostFeature, Lv2Option};
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

impl<'a> HasRelatedSet<'a, RequiresRelation, HostFeature> for DynManifestInfo {
    type BorrowedElt = HostFeature;
    type SetIter = IterBridge<EnumSetIter<HostFeature>>;

    fn set_iter(&'a self) -> Self::SetIter {
        self.host_feature_requirer.required_host_features.knowns_iter()
    }
}

impl<'a> HasRelatedSet<'a, RequiresRelation, UnknownHostFeature> for DynManifestInfo {
    type BorrowedElt = &'a UnknownHostFeature;
    type SetIter = <BTreeSet<UnknownHostFeature> as IntoParallelRefIterator<'a>>::Iter;

    fn set_iter(&'a self) -> Self::SetIter {
        self.host_feature_requirer.required_host_features.unknowns_iter()
    }
}

impl<'a> HasRelatedSet<'a, RequiresRelation, Lv2Option> for DynManifestInfo {
    type BorrowedElt = Lv2Option;
    type SetIter = IterBridge<EnumSetIter<Lv2Option>>;

    fn set_iter(&'a self) -> Self::SetIter {
        self.host_feature_requirer.required_options.knowns_iter()
    }
}

impl<'a> HasRelatedSet<'a, RequiresRelation, UnknownOption> for DynManifestInfo {
    type BorrowedElt = &'a UnknownOption;
    type SetIter = <BTreeSet<UnknownOption> as IntoParallelRefIterator<'a>>::Iter;

    fn set_iter(&'a self) -> Self::SetIter {
        self.host_feature_requirer.required_options.unknowns_iter()
    }
}

impl<'a> HasRelatedSet<'a, OptionallySupportsRelation, HostFeature> for DynManifestInfo {
    type BorrowedElt = HostFeature;
    type SetIter = IterBridge<EnumSetIter<HostFeature>>;

    fn set_iter(&'a self) -> Self::SetIter {
        self.host_feature_requirer.optional_host_features.knowns_iter()
    }
}

impl<'a> HasRelatedSet<'a, OptionallySupportsRelation, UnknownHostFeature> for DynManifestInfo {
    type BorrowedElt = &'a UnknownHostFeature;
    type SetIter = <BTreeSet<UnknownHostFeature> as IntoParallelRefIterator<'a>>::Iter;

    fn set_iter(&'a self) -> Self::SetIter {
        self.host_feature_requirer.optional_host_features.unknowns_iter()
    }
}

impl<'a> HasRelatedSet<'a, OptionallySupportsRelation, Lv2Option> for DynManifestInfo {
    type BorrowedElt = Lv2Option;
    type SetIter = IterBridge<EnumSetIter<Lv2Option>>;

    fn set_iter(&'a self) -> Self::SetIter {
        self.host_feature_requirer.optional_options.knowns_iter()
    }
}

impl<'a> HasRelatedSet<'a, OptionallySupportsRelation, UnknownOption> for DynManifestInfo {
    type BorrowedElt = &'a UnknownOption;
    type SetIter = <BTreeSet<UnknownOption> as IntoParallelRefIterator<'a>>::Iter;

    fn set_iter(&'a self) -> Self::SetIter {
        self.host_feature_requirer.optional_options.unknowns_iter()
    }
}