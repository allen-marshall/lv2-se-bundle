//! Utilities to help with implementation of traits and/or other functionality.

use enumset::{EnumSet, EnumSetIter};
use std::collections::BTreeSet;
use rayon::iter::{IterBridge, IntoParallelRefIterator, ParallelBridge};
use crate::rdf_util::Literal;
use crate::bundle_model::{Named, Documented, Provider, Requirer};
use crate::bundle_model::constants::{ExtensionData, HostFeature};
use crate::bundle_model::unknowns::{UnknownExtensionData, UnknownHostFeature, UnknownOption};

/// Base functionality for implementing the [`Named`](crate::bundle_model::Named) trait.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct NamedImpl {
    /// Human-readable names. Multiple language-tagged literals can be used.
    names: BTreeSet<Literal>,

    /// Short names, up to 16 Unicode grapheme clusters each. Multiple language-tagged literals can
    /// be used.
    short_names: BTreeSet<Literal>
}

impl<'a> Named<'a> for NamedImpl {
    type NamesIter = <BTreeSet<Literal> as IntoParallelRefIterator<'a>>::Iter;
    type ShortNamesIter = <BTreeSet<Literal> as IntoParallelRefIterator<'a>>::Iter;

    fn names_iter(&'a self) -> Self::NamesIter {
        self.names.par_iter()
    }

    fn short_names_iter(&'a self) -> Self::ShortNamesIter {
        self.short_names.par_iter()
    }
}

/// Base functionality for implementing the [`Documented`](crate::bundle_model::Documented) trait.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct DocumentedImpl {
    /// Documentation embedded in the bundle. Multiple language-tagged literals can be used. LV2
    /// specifies that the contents must be "a valid XHTML Basic 1.1 fragment suitable for use as
    /// the content of the <body> element."
    documentation: BTreeSet<Literal>
}

impl<'a> Documented<'a> for DocumentedImpl {
    type DocumentationIter = <BTreeSet<Literal> as IntoParallelRefIterator<'a>>::Iter;

    fn documentation_iter(&'a self) -> Self::DocumentationIter {
        self.documentation.par_iter()
    }
}

/// Implements base functionality for providing LV2 extension data interfaces.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct ExtensionDataProvider {
    /// Extension data interfaces provided.
    extension_data: EnumSet<ExtensionData>,

    /// Extension data interfaces that are provided, but not understood by this crate.
    unknown_extension_data: BTreeSet<UnknownExtensionData>
}

impl<'a> Provider<'a, ExtensionData> for ExtensionDataProvider {
    type BorrowedElt = ExtensionData;
    type ProvidedIter = IterBridge<EnumSetIter<ExtensionData>>;

    fn provided_iter(&'a self) -> Self::ProvidedIter {
        self.extension_data.iter().par_bridge()
    }
}

impl<'a> Provider<'a, UnknownExtensionData> for ExtensionDataProvider {
    type BorrowedElt = &'a UnknownExtensionData;
    type ProvidedIter = <BTreeSet<UnknownExtensionData> as IntoParallelRefIterator<'a>>::Iter;

    fn provided_iter(&'a self) -> Self::ProvidedIter {
        self.unknown_extension_data.par_iter()
    }
}

/// Implements base functionality for requiring (and optionally supporting) LV2 host features and
/// LV2 options.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct HostFeatureRequirer {
    /// Required host features. This set should not intersect with
    /// [`optional_host_features`](self::HostFeatureRequirer::optional_host_features).
    required_host_features: EnumSet<HostFeature>,

    /// Optionally supported host features. This set should not intersect with
    /// [`required_host_features`](self::HostFeatureRequirer::required_host_features).
    optional_host_features: EnumSet<HostFeature>,

    /// Host features that are required, but not understood by this crate. This set should not
    /// intersect with
    /// [`optional_unknown_host_features`](self::HostFeatureRequirer::optional_unknown_host_features)
    required_unknown_host_features: BTreeSet<UnknownHostFeature>,

    /// Host features that are optionally supported, but not understood by this crate. This set
    /// should not intersect with
    /// [`required_unknown_host_features`](self::HostFeatureRequirer::required_unknown_host_features).
    optional_unknown_host_features: BTreeSet<UnknownHostFeature>,

    /// LV2 options that are required, but not understood by this crate. This set should not
    /// intersect with
    /// [`optional_unknown_options`](self::HostFeatureRequirer::optional_unknown_options).
    required_unknown_options: BTreeSet<UnknownOption>,

    /// LV2 options that are optionally supported, but not understood by this crate. This set should
    /// not intersect with
    /// [`required_unknown_options`](self::HostFeatureRequirer::required_unknown_options).
    optional_unknown_options: BTreeSet<UnknownOption>
}

impl<'a> Requirer<'a, HostFeature> for HostFeatureRequirer {
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

impl<'a> Requirer<'a, UnknownHostFeature> for HostFeatureRequirer {
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

impl<'a> Requirer<'a, UnknownOption> for HostFeatureRequirer {
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