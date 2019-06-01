//! Representation of LV2 plugins.

use std::collections::BTreeSet;
use crate::rdf_util::{Literal, Iri};
use enumset::EnumSetIter;
use crate::bundle_model::constants::{ExtensionData, HostFeature, PluginType};
use crate::bundle_model::unknowns::{UnknownHostFeature, UnknownExtensionData, UnknownOption, UnknownPluginType};
use num_bigint::BigUint;
use rayon::iter::{IterBridge, IntoParallelRefIterator, ParallelIterator};
use crate::bundle_model::{ResourceVersion, Loadable, IdentifiedBy, OptionallyIdentifiedBy, HasRelatedSet, NameRelation, ShortNameRelation, DocRelation, RequiresRelation, OptionallySupportsRelation, ProvidesRelation};
use crate::bundle_model::symbol::Symbol;
use crate::bundle_model::project::ProjectInfo;
use crate::bundle_model::port::PortInfo;
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

    /// Set of LV2 plugin types to which the plugin belongs.
    plugin_types: KnownAndUnknownSet<PluginType, UnknownPluginType>,

    /// Plugin version.
    version: ResourceVersion,

    /// Name and short name information.
    named_impl: NamedImpl,

    /// Documentation information.
    documented_impl: DocumentedImpl,

    // TODO: Avoid creating multiple ProjectInfo objects for the same project if multiple plugins
    // are part of the same project. Maybe use Option<Iri> instead of Option<ProjectInfo>?
    /// Description of the project to which the plugin belongs, if specified.
    project: Option<ProjectInfo>,

    /// Description of the plugin's ports, in order of their port indices.
    ports: Vec<PortInfo>,

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
    /// Gets a (parallel) iterator over the known plugin types to which the plugin belongs.
    pub fn known_plugin_types_iter(&self) -> impl ParallelIterator<Item = PluginType> {
        self.plugin_types.knowns_iter()
    }

    /// Gets a (parallel) iterator over the unknown plugin types to which the plugin belongs.
    pub fn unknown_plugin_types_iter(&self) -> impl ParallelIterator<Item = &UnknownPluginType> {
        self.plugin_types.unknowns_iter()
    }

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

impl Loadable for PluginInfo {
    fn binary(&self) -> Option<&Iri> {
        Some(&self.binary)
    }
}

impl<'a> HasRelatedSet<'a, NameRelation, Literal> for PluginInfo {
    type BorrowedElt = &'a Literal;
    type SetIter = <BTreeSet<Literal> as IntoParallelRefIterator<'a>>::Iter;

    fn set_iter(&'a self) -> Self::SetIter {
        self.named_impl.names.par_iter()
    }
}

impl<'a> HasRelatedSet<'a, ShortNameRelation, Literal> for PluginInfo {
    type BorrowedElt = &'a Literal;
    type SetIter = <BTreeSet<Literal> as IntoParallelRefIterator<'a>>::Iter;

    fn set_iter(&'a self) -> Self::SetIter {
        self.named_impl.short_names.par_iter()
    }
}

impl<'a> HasRelatedSet<'a, DocRelation, Literal> for PluginInfo {
    type BorrowedElt = &'a Literal;
    type SetIter = <BTreeSet<Literal> as IntoParallelRefIterator<'a>>::Iter;

    fn set_iter(&'a self) -> Self::SetIter {
        self.documented_impl.documentation.par_iter()
    }
}

impl<'a> HasRelatedSet<'a, ProvidesRelation, ExtensionData> for PluginInfo {
    type BorrowedElt = ExtensionData;
    type SetIter = IterBridge<EnumSetIter<ExtensionData>>;

    fn set_iter(&'a self) -> Self::SetIter {
        self.provided_extension_data.knowns_iter()
    }
}

impl<'a> HasRelatedSet<'a, ProvidesRelation, UnknownExtensionData> for PluginInfo {
    type BorrowedElt = &'a UnknownExtensionData;
    type SetIter = <BTreeSet<UnknownExtensionData> as IntoParallelRefIterator<'a>>::Iter;

    fn set_iter(&'a self) -> Self::SetIter {
        self.provided_extension_data.unknowns_iter()
    }
}

impl<'a> HasRelatedSet<'a, RequiresRelation, HostFeature> for PluginInfo {
    type BorrowedElt = HostFeature;
    type SetIter = IterBridge<EnumSetIter<HostFeature>>;

    fn set_iter(&'a self) -> Self::SetIter {
        self.host_feature_requirer.required_host_features.knowns_iter()
    }
}

impl<'a> HasRelatedSet<'a, RequiresRelation, UnknownHostFeature> for PluginInfo {
    type BorrowedElt = &'a UnknownHostFeature;
    type SetIter = <BTreeSet<UnknownHostFeature> as IntoParallelRefIterator<'a>>::Iter;

    fn set_iter(&'a self) -> Self::SetIter {
        self.host_feature_requirer.required_host_features.unknowns_iter()
    }
}

impl<'a> HasRelatedSet<'a, OptionallySupportsRelation, HostFeature> for PluginInfo {
    type BorrowedElt = HostFeature;
    type SetIter = IterBridge<EnumSetIter<HostFeature>>;

    fn set_iter(&'a self) -> Self::SetIter {
        self.host_feature_requirer.optional_host_features.knowns_iter()
    }
}

impl<'a> HasRelatedSet<'a, OptionallySupportsRelation, UnknownHostFeature> for PluginInfo {
    type BorrowedElt = &'a UnknownHostFeature;
    type SetIter = <BTreeSet<UnknownHostFeature> as IntoParallelRefIterator<'a>>::Iter;

    fn set_iter(&'a self) -> Self::SetIter {
        self.host_feature_requirer.optional_host_features.unknowns_iter()
    }
}

impl<'a> HasRelatedSet<'a, RequiresRelation, UnknownOption> for PluginInfo {
    type BorrowedElt = &'a UnknownOption;
    type SetIter = <BTreeSet<UnknownOption> as IntoParallelRefIterator<'a>>::Iter;

    fn set_iter(&'a self) -> Self::SetIter {
        self.host_feature_requirer.required_options.par_iter()
    }
}

impl<'a> HasRelatedSet<'a, OptionallySupportsRelation, UnknownOption> for PluginInfo {
    type BorrowedElt = &'a UnknownOption;
    type SetIter = <BTreeSet<UnknownOption> as IntoParallelRefIterator<'a>>::Iter;

    fn set_iter(&'a self) -> Self::SetIter {
        self.host_feature_requirer.optional_options.par_iter()
    }
}