//! Representation of implications in which the presence of one LV2 feature, property, etc. can
//! imply the presence of others.

// TODO: Explore possibility of generating some of the information in this module automatically at
// build time, maybe using macros and/or RDF?

use enumset::EnumSet;
use enum_map::EnumMap;
use rayon::iter::FromParallelIterator;

use crate::bundle_model::constants::{PluginType, PortDesignation, Unit};
use crate::enum_graph::EnumSetDiGraph;

lazy_static! {
    /// Directed graph defining the implications among plugin types. If an edge (p0, p1) exists in
    /// the graph, then all plugins of type p0 must also be of type p1. The graph is its own
    /// transitive closure, so there is no need to consider indirect paths when finding the implied
    /// plugin types.
    pub static ref PLUGIN_TYPES_IMPLIED: EnumSetDiGraph<PluginType> = {
        let direct_implications = vec![
            (PluginType::Reverb, PluginType::Delay),
            (PluginType::Reverb, PluginType::Simulator),
            (PluginType::Waveshaper, PluginType::Distortion),
            (PluginType::Amplifier, PluginType::Dynamics),
            (PluginType::Compressor, PluginType::Dynamics),
            (PluginType::Envelope, PluginType::Dynamics),
            (PluginType::Expander, PluginType::Dynamics),
            (PluginType::Gate, PluginType::Dynamics),
            (PluginType::Limiter, PluginType::Dynamics),
            (PluginType::Allpass, PluginType::Filter),
            (PluginType::Bandpass, PluginType::Filter),
            (PluginType::Comb, PluginType::Filter),
            (PluginType::EQ, PluginType::Filter),
            (PluginType::MultiEQ, PluginType::Filter),
            (PluginType::ParaEQ, PluginType::Filter),
            (PluginType::Highpass, PluginType::Filter),
            (PluginType::Lowpass, PluginType::Filter),
            (PluginType::MultiEQ, PluginType::EQ),
            (PluginType::ParaEQ, PluginType::EQ),
            (PluginType::Constant, PluginType::Generator),
            (PluginType::Instrument, PluginType::Generator),
            (PluginType::Oscillator, PluginType::Generator),
            (PluginType::Chorus, PluginType::Modulator),
            (PluginType::Flanger, PluginType::Modulator),
            (PluginType::Phaser, PluginType::Modulator),
            (PluginType::Pitch, PluginType::Spectral),
            (PluginType::Analyser, PluginType::Utility),
            (PluginType::Converter, PluginType::Utility),
            (PluginType::Function, PluginType::Utility),
            (PluginType::Mixer, PluginType::Utility)
        ];
        EnumSetDiGraph::from_par_iter(direct_implications).transitive_closure()
    };

    /// Maps LV2 port designations to the port units that they imply, if any.
    pub static ref UNITS_IMPLIED_BY_DESIGNATIONS: EnumMap<PortDesignation, Option<Unit>> = {
        let implications = vec![
            (PortDesignation::Gain, Unit::Decibel)
        ];
        let mut output = EnumMap::from(|_| None);
        output.extend(implications.into_iter()
            .map(|(designation, unit)| (designation, Some(unit))));
        output
    };
}