//! Utilities to help with implementation of traits and/or other functionality.

use enumset::{EnumSet, EnumSetIter, EnumSetType};
use std::collections::BTreeSet;
use rayon::iter::{IterBridge, IntoParallelRefIterator, ParallelBridge};
use crate::rdf_util::Literal;
use crate::bundle_model::constants::HostFeature;
use crate::bundle_model::unknowns::{UnknownHostFeature, UnknownOption};

/// Represents a set that contains both "known" items and "unknown" items. The "unknown" items
/// typically represent LV2 data that this crate does not understand, such as information from
/// nonstandard extensions.
///
/// # Parameters
/// - `K`: Type for "known" items.
/// - `U`: Type for "unknown" items.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct KnownAndUnknownSet<K: EnumSetType, U> {
    /// Set of known items.
    knowns: EnumSet<K>,

    /// Set of unknown items.
    unknowns: BTreeSet<U>
}

impl<K: EnumSetType, U> KnownAndUnknownSet<K, U> {
    /// Gets a (parallel) iterator over the "known" elements of this set. The iterator will not
    /// repeat elements.
    pub(crate) fn knowns_iter(&self) -> IterBridge<EnumSetIter<K>>
        where K: Send, EnumSet<K>: Send
    {
        self.knowns.iter().par_bridge()
    }

    /// Gets a (parallel) iterator over the "unknown" elements of this set. The iterator will not
    /// repeat elements.
    pub(crate) fn unknowns_iter(&self) -> <BTreeSet<U> as IntoParallelRefIterator>::Iter
        where U: Ord + Sync
    {
        self.unknowns.par_iter()
    }
}

/// Base functionality for implementing the [`Named`](crate::bundle_model::Named) trait.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct NamedImpl {
    /// Human-readable names. Multiple language-tagged literals can be used.
    pub(crate) names: BTreeSet<Literal>,

    /// Short names, up to 16 Unicode grapheme clusters each. Multiple language-tagged literals can
    /// be used.
    pub(crate) short_names: BTreeSet<Literal>
}

/// Base functionality for implementing the [`Documented`](crate::bundle_model::Documented) trait.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct DocumentedImpl {
    /// Documentation embedded in the bundle. Multiple language-tagged literals can be used. LV2
    /// specifies that the contents must be "a valid XHTML Basic 1.1 fragment suitable for use as
    /// the content of the <body> element."
    pub(crate) documentation: BTreeSet<Literal>
}

/// Implements base functionality for requiring (and optionally supporting) LV2 host features and
/// LV2 options.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct HostFeatureRequirer {
    /// Required host features. This set should not intersect with
    /// [`optional_host_features`](self::HostFeatureRequirer::optional_host_features).
    pub(crate) required_host_features: KnownAndUnknownSet<HostFeature, UnknownHostFeature>,

    /// Optionally supported host features. This set should not intersect with
    /// [`required_host_features`](self::HostFeatureRequirer::required_host_features).
    pub(crate) optional_host_features: KnownAndUnknownSet<HostFeature, UnknownHostFeature>,

    /// Required LV2 options. This set should not intersect with
    /// [`optional_options`](self::HostFeatureRequirer::optional_options)
    pub(crate) required_options: BTreeSet<UnknownOption>,

    /// Optionally supported LV2 options. This set should not intersect with
    /// [`required_options`](self::HostFeatureRequirer::required_options)
    pub(crate) optional_options: BTreeSet<UnknownOption>
}