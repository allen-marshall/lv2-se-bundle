//! Representation of LV2 dynamic manifest generator.

use crate::rdf_util::Iri;
use crate::bundle_model::{Loadable, Requirer, OptionallyIdentifiedBy};
use crate::bundle_model::symbol::Symbol;
use enumset::{EnumSet, EnumSetIter};
use crate::bundle_model::constants::HostFeature;
use crate::bundle_model::unknowns::{UnknownHostFeature, UnknownOption};
use rayon::iter::{IterBridge, ParallelBridge, IntoParallelRefIterator};
use std::collections::BTreeSet;

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
    optional_host_features: EnumSet<HostFeature>,

    /// Host features that are required by the dynamic manifest generator, but not understood by
    /// this crate. This set should not intersect with
    /// [`optional_unknown_host_features`](self::DynManifestInfo::optional_unknown_host_features)
    required_unknown_host_features: BTreeSet<UnknownHostFeature>,

    /// Host features that are optionally supported by the dynamic manifest generator, but not
    /// understood by this crate. This set should not intersect with
    /// [`required_unknown_host_features`](self::DynManifestInfo::required_unknown_host_features).
    optional_unknown_host_features: BTreeSet<UnknownHostFeature>,

    /// LV2 options that are required by the dynamic manifest generator, but not understood by this
    /// crate. This set should not intersect with
    /// [`optional_unknown_options`](self::DynManifestInfo::optional_unknown_options).
    required_unknown_options: BTreeSet<UnknownOption>,

    /// LV2 options that are optionally supported by the dynamic manifest generator, but not
    /// understood by this crate. This set should not intersect with
    /// [`required_unknown_options`](self::DynManifestInfo::required_unknown_options).
    optional_unknown_options: BTreeSet<UnknownOption>
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
        self.required_host_features.iter().par_bridge()
    }

    fn optionally_supported_iter(&'a self) -> Self::OptionallySupportedIter {
        self.optional_host_features.iter().par_bridge()
    }
}

impl<'a> Requirer<'a, UnknownHostFeature> for DynManifestInfo {
    type BorrowedElt = &'a UnknownHostFeature;
    type RequiredIter = <BTreeSet<UnknownHostFeature> as IntoParallelRefIterator<'a>>::Iter;
    type OptionallySupportedIter = <BTreeSet<UnknownHostFeature> as IntoParallelRefIterator<'a>>::Iter;

    fn required_iter(&'a self) -> Self::RequiredIter {
        self.required_unknown_host_features.par_iter()
    }

    fn optionally_supported_iter(&'a self) -> Self::OptionallySupportedIter {
        self.optional_unknown_host_features.par_iter()
    }
}

impl<'a> Requirer<'a, UnknownOption> for DynManifestInfo {
    type BorrowedElt = &'a UnknownOption;
    type RequiredIter = <BTreeSet<UnknownOption> as IntoParallelRefIterator<'a>>::Iter;
    type OptionallySupportedIter = <BTreeSet<UnknownOption> as IntoParallelRefIterator<'a>>::Iter;

    fn required_iter(&'a self) -> Self::RequiredIter {
        self.required_unknown_options.par_iter()
    }

    fn optionally_supported_iter(&'a self) -> Self::OptionallySupportedIter {
        self.optional_unknown_options.par_iter()
    }
}