//! Representation of LV2 plugins.

use std::collections::BTreeSet;
use std::collections::btree_set;
use crate::rdf_util::{Literal, Iri};
use enumset::{EnumSet, EnumSetIter};
use crate::bundle_model::constants::{ExtensionData, HostFeature};
use num_bigint::BigUint;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use crate::bundle_model::{ResourceVersion, LoadableEntity, Nameable, ExtensionDataProvider, HostFeatureSupporter};
use crate::bundle_model::symbol::Symbol;
use crate::bundle_model::project::ProjectInfo;

/// Representation of an LV2 plugin.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PluginInfo {
    /// IRI identifying the plugin.
    iri: Iri,

    /// URI pointing to the shared library that implements the plugin.
    binary: Iri,

    /// Human-readable plugin names. Multiple language-tagged literals can be used. These should be
    /// extracted from the doap:name RDF property.
    names: BTreeSet<Literal>,

    /// Plugin documentation embedded in the bundle. Multiple language-tagged literals can be used.
    /// LV2 specifies that the contents should be "a valid XHTML Basic 1.1 fragment suitable for use
    /// as the content of the <body> element."
    documentation: BTreeSet<Literal>,

    /// Description of the project to which the plugin belongs, if specified.
    project: Option<ProjectInfo>,

    /// Short names for the plugin, up to 16 characters each. Multiple language-tagged literals can
    /// be used.
    short_names: BTreeSet<Literal>,

    /// LV2 symbol identifying the plugin.
    symbol: Option<Symbol>,

    /// Plugin version.
    version: ResourceVersion,

    /// Number of latency frames introduced by the plugin, if specified.
    latency: Option<BigUint>,

    // TODO: Add ports field.

    /// Host features required by the plugin. This set should not intersect with
    /// [`optional_host_features`](self::PluginInfo::optional_host_features).
    required_host_features: EnumSet<HostFeature>,

    /// Host features optionally supported by the plugin. This set should not intersect with
    /// [`required_host_features`](self::PluginInfo::required_host_features).
    optional_host_features: EnumSet<HostFeature>,

    /// Types of extension data provided by the plugin in `LV2_Descriptor::instantiate()`.
    extension_data: EnumSet<ExtensionData>,

    /// Flag indicating whether the plugin is enabled or bypassed. Most bundles probably won't
    /// specify this, as its value seems to only make sense at runtime.
    enabled: Option<bool>,

    /// Flag indicating whether the plugin is processing as fast as possible (true) or being limited
    /// to real time (false). Most bundles probably won't specify this, as its value seems to only
    /// make sense at runtime.
    free_wheeling: Option<bool>
}

impl PluginInfo {
    /// Gets the plugin version specified in the bundle.
    pub fn version(&self) -> &ResourceVersion {
        &self.version
    }

    /// Gets an iterator over the plugin's documentation literals. A plugin may have multiple
    /// language-tagged documentation literals to provide multilingual documentation. The LV2
    /// specification indicates that the literal contents should be "a valid XHTML Basic 1.1
    /// fragment suitable for use as the content of the <body> element."
    pub fn documentation(&self) -> impl ParallelIterator<Item = &Literal> {
        self.documentation.par_iter()
    }

    /// Gets the project information for the plugin. Returns [`None`](std::option::Option::None) if
    /// the bundle does not specify a project for the plugin.
    pub fn project(&self) -> Option<&ProjectInfo> {
        self.project.as_ref()
    }

    /// Gets the number of latency frames introduced by the plugin. Returns
    /// [`None`](std::option::Option::None) if the bundle does not specify a latency amount for the
    /// plugin.
    pub fn latency(&self) -> Option<&BigUint> {
        self.latency.as_ref()
    }

    /// Gets a boolean indicating whether the plugin is enabled (true) or bypassed (false). Returns
    /// [`None`](std::option::Option::None) if the bundle does not specify an enabled/bypassed
    /// state. Most bundles probably won't specify this flag, as its value seems to only make sense
    /// at runtime.
    pub fn enabled(&self) -> Option<bool> {
        self.enabled
    }

    /// Gets a boolean indicating whether the plugin is running as fast as possible (true) or being
    /// limited to real time (false). Returns [`None`](std::option::Option::None) if the bundle does
    /// not specify a free-wheeling state. Most bundles probably won't specify this flag, as its
    /// value seems to only make sense at runtime.
    pub fn free_wheeling(&self) -> Option<bool> {
        self.free_wheeling
    }
}

impl LoadableEntity for PluginInfo {
    fn iri(&self) -> Option<&Iri> {
        Some(&self.iri)
    }

    fn binary(&self) -> Option<&Iri> {
        Some(&self.binary)
    }

    fn symbol(&self) -> Option<&Symbol> {
        self.symbol.as_ref()
    }
}

impl<'a> Nameable<'a> for PluginInfo {
    type NamesIter = btree_set::Iter<'a, Literal>;
    type ShortNamesIter = btree_set::Iter<'a, Literal>;

    fn names_iter(&'a self) -> Self::NamesIter {
        self.names.iter()
    }

    fn short_names_iter(&'a self) -> Self::ShortNamesIter {
        self.short_names.iter()
    }
}

impl ExtensionDataProvider for PluginInfo {
    type ExtensionDataIter = EnumSetIter<ExtensionData>;

    fn extension_data_iter(&self) -> Self::ExtensionDataIter {
        self.extension_data.iter()
    }
}

impl HostFeatureSupporter for PluginInfo {
    type RequiredHostFeaturesIter = EnumSetIter<HostFeature>;

    type OptionalHostFeaturesIter = EnumSetIter<HostFeature>;

    fn required_host_features_iter(&self) -> Self::RequiredHostFeaturesIter {
        self.required_host_features.iter()
    }

    fn optional_host_features_iter(&self) -> Self::OptionalHostFeaturesIter {
        self.optional_host_features.iter()
    }
}