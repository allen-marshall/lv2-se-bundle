//! Representation of implications in which the presence of one LV2 feature, property, etc. can
//! imply the presence of others.

// TODO: Explore possibility of generating some of the information in this module automatically at
// build time, maybe using macros and/or RDF?

use enumset::EnumSet;

use crate::bundle_model::constants::{PluginType, PortDesignation, Unit};

/// Finds plugin types implied by other plugin types. If a plugin is explicitly identified as
/// belonging to the plugin types in a set `x`, it must also belong to all plugin types in the set
/// `plug_types_implied_by_plug_types(x)`. The returned set will contain at least the same set of
/// plugin types provided in the argument.
pub fn plugin_types_implied_by_plugin_types(explicit_plugin_types: EnumSet<PluginType>) -> EnumSet<PluginType> {
    let mut plugin_types = explicit_plugin_types;

    // Insert plugin type superclasses if any of their subclasses are present.
    if explicit_plugin_types.contains(PluginType::Reverb) {
        plugin_types.insert(PluginType::Delay);
        plugin_types.insert(PluginType::Simulator);
    }
    if explicit_plugin_types.contains(PluginType::Waveshaper) {
        plugin_types.insert(PluginType::Distortion);
    }
    if !explicit_plugin_types.is_disjoint(PluginType::Amplifier | PluginType::Compressor | PluginType::Envelope | PluginType::Expander | PluginType::Gate | PluginType::Limiter) {
        plugin_types.insert(PluginType::Dynamics);
    }
    if !explicit_plugin_types.is_disjoint(PluginType::Allpass | PluginType::Bandpass | PluginType::Comb | PluginType::EQ | PluginType::MultiEQ | PluginType::ParaEQ | PluginType::Highpass | PluginType::Lowpass) {
        plugin_types.insert(PluginType::Filter);
    }
    if !explicit_plugin_types.is_disjoint(PluginType::MultiEQ | PluginType::ParaEQ) {
        plugin_types.insert(PluginType::EQ);
    }
    if !explicit_plugin_types.is_disjoint(PluginType::Constant | PluginType::Instrument | PluginType::Oscillator) {
        plugin_types.insert(PluginType::Generator);
    }
    if !explicit_plugin_types.is_disjoint(PluginType::Chorus | PluginType::Flanger | PluginType::Phaser) {
        plugin_types.insert(PluginType::Modulator);
    }
    if explicit_plugin_types.contains(PluginType::Pitch) {
        plugin_types.insert(PluginType::Spectral);
    }
    if !explicit_plugin_types.is_disjoint(PluginType::Analyser | PluginType::Converter | PluginType::Function | PluginType::Mixer) {
        plugin_types.insert(PluginType::Utility);
    }

    plugin_types
}

/// Finds port units implied by a set of LV2 port designations. Some port designations automatically
/// imply a unit for the port, and this function looks for such cases. The returned set is empty if
/// none of the specified port designations imply a unit.
pub fn units_implied_by_designations(designations: EnumSet<PortDesignation>) -> EnumSet<Unit> {
    if designations.contains(PortDesignation::Gain) {
        EnumSet::only(Unit::Decibel)
    }
    else {
        EnumSet::empty()
    }
}