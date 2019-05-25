//! Representation of LV2 plugins.

use std::collections::BTreeSet;
use crate::rdf_util::{Literal, Iri};
use enumset::{EnumSet, EnumSetIter};
use crate::bundle_model::constants::{ExtensionData, HostFeature};
use crate::bundle_model::unknowns::{UnknownHostFeature, UnknownExtensionData, UnknownOption};
use num_bigint::BigUint;
use rayon::iter::{IterBridge, IntoParallelRefIterator, ParallelBridge};
use crate::bundle_model::{ResourceVersion, Provider, Requirer, Loadable, IdentifiedBy, OptionallyIdentifiedBy, Named, Documented};
use crate::bundle_model::symbol::Symbol;
use crate::bundle_model::project::ProjectInfo;

/// Representation of an LV2 plugin.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PluginInfo {
    /// IRI identifying the plugin.
    iri: Iri,

    /// URI pointing to the shared library that implements the plugin.
    binary: Iri,

    /// Human-readable plugin names. Multiple language-tagged literals can be used.
    names: BTreeSet<Literal>,

    /// Plugin documentation embedded in the bundle. Multiple language-tagged literals can be used.
    /// LV2 specifies that the contents should be "a valid XHTML Basic 1.1 fragment suitable for use
    /// as the content of the <body> element."
    documentation: BTreeSet<Literal>,

    /// Description of the project to which the plugin belongs, if specified.
    project: Option<ProjectInfo>,

    /// Short names for the plugin, up to 16 Unicode grapheme clusters each. Multiple
    /// language-tagged literals can be used.
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

    /// Host features that are required by the plugin, but not understood by this crate. This set
    /// should not intersect with
    /// [`optional_unknown_host_features`](self::PluginInfo::optional_unknown_host_features)
    required_unknown_host_features: BTreeSet<UnknownHostFeature>,

    /// Host features that are optionally supported by the plugin, but not understood by this crate.
    /// This set should not intersect with
    /// [`required_unknown_host_features`](self::PluginInfo::required_unknown_host_features).
    optional_unknown_host_features: BTreeSet<UnknownHostFeature>,

    /// LV2 options that are required by the plugin, but not understood by this crate. This set
    /// should not intersect with
    /// [`optional_unknown_options`](self::PluginInfo::optional_unknown_options).
    required_unknown_options: BTreeSet<UnknownOption>,

    /// LV2 options that are optionally supported by the plugin, but not understood by this crate.
    /// This set should not intersect with
    /// [`required_unknown_options`](self::PluginInfo::required_unknown_options).
    optional_unknown_options: BTreeSet<UnknownOption>,

    /// Types of extension data provided by the plugin in `LV2_Descriptor::instantiate()`.
    extension_data: EnumSet<ExtensionData>,

    /// Types of extension data that are provided by the plugin, but not understood by this crate.
    unknown_extension_data: BTreeSet<UnknownExtensionData>,

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

impl IdentifiedBy<Iri> for PluginInfo {
    fn id(&self) -> &Iri {
        &self.iri
    }
}

impl OptionallyIdentifiedBy<Symbol> for PluginInfo {
    fn id(&self) -> Option<&Symbol> {
        self.symbol.as_ref()
    }
}

impl<'a> Named<'a> for PluginInfo {
    type NamesIter = <BTreeSet<Literal> as IntoParallelRefIterator<'a>>::Iter;
    type ShortNamesIter = <BTreeSet<Literal> as IntoParallelRefIterator<'a>>::Iter;

    fn names_iter(&'a self) -> Self::NamesIter {
        self.names.par_iter()
    }

    fn short_names_iter(&'a self) -> Self::ShortNamesIter {
        self.short_names.par_iter()
    }
}

impl<'a> Documented<'a> for PluginInfo {
    type DocumentationIter = <BTreeSet<Literal> as IntoParallelRefIterator<'a>>::Iter;

    fn documentation_iter(&'a self) -> Self::DocumentationIter {
        self.documentation.par_iter()
    }
}

impl Loadable for PluginInfo {
    fn binary(&self) -> Option<&Iri> {
        Some(&self.binary)
    }
}

impl<'a> Provider<'a, ExtensionData> for PluginInfo {
    type BorrowedElt = ExtensionData;
    type ProvidedIter = IterBridge<EnumSetIter<ExtensionData>>;

    fn provided_iter(&'a self) -> Self::ProvidedIter {
        self.extension_data.iter().par_bridge()
    }
}

impl<'a> Provider<'a, UnknownExtensionData> for PluginInfo {
    type BorrowedElt = &'a UnknownExtensionData;
    type ProvidedIter = <BTreeSet<UnknownExtensionData> as IntoParallelRefIterator<'a>>::Iter;

    fn provided_iter(&'a self) -> Self::ProvidedIter {
        self.unknown_extension_data.par_iter()
    }
}

impl<'a> Requirer<'a, HostFeature> for PluginInfo {
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

impl<'a> Requirer<'a, UnknownHostFeature> for PluginInfo {
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

impl<'a> Requirer<'a, UnknownOption> for PluginInfo {
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