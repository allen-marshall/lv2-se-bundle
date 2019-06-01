//! Representation of LV2 plugins.

use std::collections::BTreeSet;
use crate::rdf_util::{Literal, Iri};
use enumset::EnumSetIter;
use crate::bundle_model::constants::{ExtensionData, HostFeature};
use crate::bundle_model::unknowns::{UnknownHostFeature, UnknownExtensionData, UnknownOption};
use num_bigint::BigUint;
use rayon::iter::{IterBridge, IntoParallelRefIterator};
use crate::bundle_model::{ResourceVersion, Provider, Requirer, Loadable, IdentifiedBy, OptionallyIdentifiedBy, Named, Documented};
use crate::bundle_model::symbol::Symbol;
use crate::bundle_model::project::ProjectInfo;
use crate::bundle_model::impl_util::{KnownAndUnknownSet, NamedImpl, DocumentedImpl, HostFeatureRequirer};

/// Representation of an LV2 plugin.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PluginInfo {
    /// IRI identifying the plugin.
    iri: Iri,

    /// LV2 symbol identifying the plugin.
    symbol: Option<Symbol>,

    /// URI pointing to the shared library that implements the plugin.
    binary: Iri,

    /// Plugin version.
    version: ResourceVersion,

    /// Name and short name information.
    named_impl: NamedImpl,

    /// Documentation information.
    documented_impl: DocumentedImpl,

    /// Description of the project to which the plugin belongs, if specified.
    project: Option<ProjectInfo>,

    // TODO: Add ports field.

    /// Set of LV2 extension data interfaces provided by the plugin.
    provided_extension_data: KnownAndUnknownSet<ExtensionData, UnknownExtensionData>,

    /// Information about required (and optional) host features and LV2 options.
    host_feature_requirer: HostFeatureRequirer,

    /// Number of latency frames introduced by the plugin, if specified.
    latency: Option<BigUint>,

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
        self.named_impl.names.par_iter()
    }

    fn short_names_iter(&'a self) -> Self::ShortNamesIter {
        self.named_impl.short_names.par_iter()
    }
}

impl<'a> Documented<'a> for PluginInfo {
    type DocumentationIter = <BTreeSet<Literal> as IntoParallelRefIterator<'a>>::Iter;

    fn documentation_iter(&'a self) -> Self::DocumentationIter {
        self.documented_impl.documentation.par_iter()
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
        self.provided_extension_data.knowns_iter()
    }
}

impl<'a> Provider<'a, UnknownExtensionData> for PluginInfo {
    type BorrowedElt = &'a UnknownExtensionData;
    type ProvidedIter = <BTreeSet<UnknownExtensionData> as IntoParallelRefIterator<'a>>::Iter;

    fn provided_iter(&'a self) -> Self::ProvidedIter {
        self.provided_extension_data.unknowns_iter()
    }
}

impl<'a> Requirer<'a, HostFeature> for PluginInfo {
    type BorrowedElt = HostFeature;
    type RequiredIter = IterBridge<EnumSetIter<HostFeature>>;
    type OptionallySupportedIter = IterBridge<EnumSetIter<HostFeature>>;

    fn required_iter(&'a self) -> Self::RequiredIter {
        self.host_feature_requirer.required_host_features.knowns_iter()
    }

    fn optionally_supported_iter(&'a self) -> Self::OptionallySupportedIter {
        self.host_feature_requirer.optional_host_features.knowns_iter()
    }
}

impl<'a> Requirer<'a, UnknownHostFeature> for PluginInfo {
    type BorrowedElt = &'a UnknownHostFeature;
    type RequiredIter = <BTreeSet<UnknownHostFeature> as IntoParallelRefIterator<'a>>::Iter;
    type OptionallySupportedIter = <BTreeSet<UnknownHostFeature> as IntoParallelRefIterator<'a>>::Iter;

    fn required_iter(&'a self) -> Self::RequiredIter {
        self.host_feature_requirer.required_host_features.unknowns_iter()
    }

    fn optionally_supported_iter(&'a self) -> Self::OptionallySupportedIter {
        self.host_feature_requirer.optional_host_features.unknowns_iter()
    }
}

impl<'a> Requirer<'a, UnknownOption> for PluginInfo {
    type BorrowedElt = &'a UnknownOption;
    type RequiredIter = <BTreeSet<UnknownOption> as IntoParallelRefIterator<'a>>::Iter;
    type OptionallySupportedIter = <BTreeSet<UnknownOption> as IntoParallelRefIterator<'a>>::Iter;

    fn required_iter(&'a self) -> Self::RequiredIter {
        self.host_feature_requirer.required_options.par_iter()
    }

    fn optionally_supported_iter(&'a self) -> Self::OptionallySupportedIter {
        self.host_feature_requirer.optional_options.par_iter()
    }
}