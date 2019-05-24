//! Representation of LV2 plugins.

use std::collections::BTreeSet;
use crate::rdf_util::{Literal, Iri};
use enumset::EnumSet;
use crate::bundle_model::constants::{PluginExtensionData, HostFeature};
use num_bigint::BigUint;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use crate::bundle_model::ResourceVersion;
use crate::bundle_model::symbol::Symbol;

/// Representation of an LV2 plugin.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PluginInfo {
    /// IRI identifying the plugin.
    iri: Iri,

    /// URI pointing to the shared library that implements the plugin.
    binary: Option<Iri>,

    /// Human-readable plugin names. Multiple language-tagged literals can be used. These should be
    /// extracted from the doap:name RDF property.
    names: BTreeSet<Literal>,

    /// Plugin documentation embedded in the bundle. Multiple language-tagged literals can be used.
    /// LV2 specifies that the contents should be "a valid XHTML Basic 1.1 fragment suitable for use
    /// as the content of the <body> element."
    documentation: BTreeSet<Literal>,

    // TODO: Add project field.

    /// Short name for the plugin, up to 16 characters.
    short_name: Option<String>,

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
    extension_data: EnumSet<PluginExtensionData>,

    /// Flag indicating whether the plugin is enabled or bypassed. Most bundles probably won't
    /// specify this, as its value seems to only make sense at runtime.
    enabled: Option<bool>,

    /// Flag indicating whether the plugin is processing as fast as possible (true) or being limited
    /// to real time (false). Most bundles probably won't specify this, as its value seems to only
    /// make sense at runtime.
    free_wheeling: Option<bool>
}

impl PluginInfo {
    /// Gets the IRI that identifies the plugin.
    pub fn iri(&self) -> &Iri {
        &self.iri
    }

    /// Gets a URI pointing to the shared library that implements the plugin. Returns
    /// [`None`](std::option::Option::None) if the bundle does not specify a binary for the plugin.
    pub fn binary(&self) -> Option<&Iri> {
        self.binary.as_ref()
    }

    /// Gets an iterator over the human-readable name literals for the plugin. A plugin may have
    /// multiple language-tagged name literals to provide multilingual naming.
    pub fn names(&self) -> impl ParallelIterator<Item = &Literal> {
        self.names.par_iter()
    }

    /// Gets an iterator over the plugin's documentation literals. A plugin may have multiple
    /// language-tagged documentation literals to provide multilingual documentation. The LV2
    /// specification indicates that the literal contents should be "a valid XHTML Basic 1.1
    /// fragment suitable for use as the content of the <body> element."
    pub fn documentation(&self) -> impl ParallelIterator<Item = &Literal> {
        self.documentation.par_iter()
    }

    /// Gets the short name for the plugin, which can be up to 16 characters. Returns
    /// [`None`](std::option::Option::None) if the bundle does not specify a short name for the
    /// plugin.
    pub fn short_name(&self) -> Option<&String> {
        self.short_name.as_ref()
    }

    /// Gets the LV2 symbol identifying the plugin. While this can be useful for plugin search
    /// functionality in a host UI, the usual way of identifying a plugin is by its IRI. Returns
    /// [`None`](std::option::Option::None) if the bundle does not specify a symbol for the plugin.
    pub fn symbol(&self) -> Option<&Symbol> {
        self.symbol.as_ref()
    }

    /// Gets the plugin version specified in the bundle.
    pub fn version(&self) -> &ResourceVersion {
        &self.version
    }

    /// Gets the number of latency frames introduced by the plugin. Returns
    /// [`None`](std::option::Option::None) if the bundle does not specify a latency amount for the
    /// plugin.
    pub fn latency(&self) -> Option<&BigUint> {
        self.latency.as_ref()
    }

    /// Gets an iterator over the host features required by the plugin. The iterator will not repeat
    /// items.
    pub fn required_host_features(&self) -> impl Iterator<Item = HostFeature> {
        self.required_host_features.iter()
    }

    /// Checks if the specified host feature is required by the plugin.
    ///
    /// # Parameters
    /// - `host_feature`: The host feature to check for.
    pub fn requires_host_featue(&self, host_feature: HostFeature) -> bool {
        self.required_host_features.contains(host_feature)
    }

    /// Gets an iterator over the host features that are supported but not required by the plugin.
    /// The iterator will not repeat items.
    pub fn optional_host_features(&self) -> impl Iterator<Item = HostFeature> {
        self.optional_host_features.iter()
    }

    /// Checks if the specified host feature is optionally supported by the plugin.
    ///
    /// # Parameters
    /// - `host_feature`: The host feature to check for.
    pub fn optionally_supports_host_feature(&self, host_feature: HostFeature) -> bool {
        self.optional_host_features.contains(host_feature)
    }

    /// Gets an iterator over the host features that are supported by the plugin, as either required
    /// features or optional features. The iterator will not repeat items.
    pub fn supported_host_features(&self) -> impl Iterator<Item = HostFeature> {
        (self.required_host_features | self.optional_host_features).iter()
    }

    /// Checks if the specified host feature is supported by the plugin, as either a required
    /// feature or an optional feature.
    ///
    /// # Parameters
    /// - `host_feature`: The host feature to check for.
    pub fn supports_host_feature(&self, host_feature: HostFeature) -> bool {
        self.requires_host_featue(host_feature)
            || self.optionally_supports_host_feature(host_feature)
    }

    /// Gets an iterator over the types of extension data provided by the plugin via
    /// `LV2_Descriptor::instantiate()`. The iterator will not repeat items.
    pub fn extension_data(&self) -> impl Iterator<Item = PluginExtensionData> {
        self.extension_data.iter()
    }

    /// Checks if the plugin provides the specified type of extension data via
    /// `LV2_Descriptor::instantiate()`.
    ///
    /// # Parameters
    /// - `extension_data`: The type of extension data to check for.
    pub fn has_extension_data(&self, extension_data: PluginExtensionData) -> bool {
        self.extension_data.contains(extension_data)
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