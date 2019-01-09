//! Functionality related to RDF class hierarchies used by the LV2 standard. Not all standard LV2
//! class hierarchies are represented here, as some are simple enough to be better represented by
//! other means.

// TODO: Explore possibility of generating these class hierarchies automatically at build time,
// maybe using macros and/or RDF?

use std::option;
use std::vec;
use std::hash::Hash;
use std::collections::HashSet;

/// Trait for types whose instances correspond to classes in a (subset of an) RDF class hierarchy.
/// This trait is most suitable for simple [`Copy`](std::marker::Copy) types whose hierarchy is
/// known at compile time. The hierarchy is not restricted to be a tree; it may be any directed
/// graph. (Cycles are unusual but allowed.)
pub trait ClassInHierarchy: Clone + Eq + Hash + Sized {
    /// Type for the iterator returned by
    /// [`direct_superclasses`](self::ClassInHierarchy::direct_superclasses).
    type SuperclassesIter: Iterator<Item = Self>;

    /// Gets the set of direct superclasses for this RDF class, not including this class itself.
    fn direct_superclasses(&self) -> Self::SuperclassesIter;

    /// Checks if this RDF class is a subclass of another class. A class is considered to be a
    /// subclass of itself.
    fn is_subclass_of(&self, other: &Self) -> bool {
        // Perform a depth-first search up the subclass links to find out if other is reachable
        // from self.
        let mut visited = HashSet::new();
        let mut remaining = vec![self.clone()];
        while let Some(next) = remaining.pop() {
            if other == &next {
                return true;
            }
            if !visited.contains(&next) {
                remaining.extend(next.direct_superclasses());
                visited.insert(next);
            }
        }
        false
    }

    /// Checks if this RDF class is a superclass of another class. A class is considered to be a
    /// superclass of itself.
    fn is_superclass_of(&self, other: &Self) -> bool {
        other.is_subclass_of(self)
    }
}

/// Identifiers for standard LV2 plugin classes. Non-standard plugin classes can exist but are
/// not represented by this type.
///
/// Note: This type's implementations of [`Ord`](std::cmp::Ord) and
/// [`PartialOrd`](std::cmp::PartialOrd) have little semantic meaning, and exist mainly for use with
/// collections that require an ordered element type. In particular, superclasses are not guaranteed
/// to have a particular ordering relative to their subclasses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum StdPluginType {
    /// Base class for the plugin class hierarchy.
    Plugin,
    DelayPlugin,
    ReverbPlugin,
    DistortionPlugin,
    WaveshaperPlugin,
    DynamicsPlugin,
    AmplifierPlugin,
    CompressorPlugin,
    EnvelopePlugin,
    ExpanderPlugin,
    GatePlugin,
    LimiterPlugin,
    FilterPlugin,
    AllpassPlugin,
    BandpassPlugin,
    CombPlugin,
    EQPlugin,
    MultiEQPlugin,
    ParaEQPlugin,
    HighpassPlugin,
    LowpassPlugin,
    GeneratorPlugin,
    ConstantPlugin,
    InstrumentPlugin,
    OscillatorPlugin,
    ModulatorPlugin,
    ChorusPlugin,
    FlangerPlugin,
    PhaserPlugin,
    SimulatorPlugin,
    SpatialPlugin,
    SpectralPlugin,
    PitchPlugin,
    UtilityPlugin,
    AnalyserPlugin,
    ConverterPlugin,
    FunctionPlugin,
    MixerPlugin
}

/// Identifiers for standard LV2 atom classes. Non-standard atom classes can exist but are not
/// represented by this type.
///
/// Note: This type's implementations of [`Ord`](std::cmp::Ord) and
/// [`PartialOrd`](std::cmp::PartialOrd) have little semantic meaning, and exist mainly for use with
/// collections that require an ordered element type. In particular, superclasses are not guaranteed
/// to have a particular ordering relative to their subclasses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum StdAtomType {
    /// Base class for the atom class hierarchy.
    Atom,

    /// Boolean atom type.
    Bool,

    /// Type for atoms that provide a generic chunk of memory, with size determined by the atom's
    /// size field.
    Chunk,

    /// Similar to an RDF literal. The atom contains UTF-8 data, with an optional language or type
    /// tag.
    Literal,

    /// Base class for numeric atom types.
    Number,

    /// Double-precision floating point number (always 64 bits).
    Double,

    /// Single-precision floating point number (always 32 bits).
    Float,

    /// Signed integer (always 32 bits).
    Int,

    /// Signed integer (always 64 bits).
    Long,

    /// Object atom type. Object atoms are essentially dictionaries with LV2 URIDs as keys and atoms
    /// as values.
    Object,

    /// Atom type representing a property of an [`Object`](self::StdAtomType::Object). An atom of
    /// this type contains a key-value pair.
    Property,

    /// An atom of this type contains a list of time-stamped atoms, which must all have the same
    /// pre-specified type.
    Sequence,

    /// UTF-8 string type.
    String,

    /// UTF-8 URI type.
    Uri,

    /// UTF-8 URI type with only a path component.
    Path,

    /// An atom of this type contains a list of atoms, which may have different types.
    Tuple,

    /// LV2 URID atom. A URID is a 32-bit unsigned integer that has been mapped to a URI.
    Urid,

    /// An atom of this type contains a list of atoms, which must all have the same pre-specified
    /// type.
    Vector,

    /// An atom of this type contains a list of [`Float`](self::StdAtomType::Float) atoms.
    Sound,

    /// An atom of this type contains a single MIDI event.
    MidiEvent,

    /// A specialized MIDI atom type.
    MidiSystemMessage,

    /// A specialized MIDI atom type.
    MidiSystemCommon,

    /// A specialized MIDI atom type.
    MidiQuarterFrame,

    /// A specialized MIDI atom type.
    MidiSongPosition,

    /// A specialized MIDI atom type.
    MidiSongSelect,

    /// A specialized MIDI atom type.
    MidiTuneRequest,

    /// A specialized MIDI atom type.
    MidiSystemExclusive,

    /// A specialized MIDI atom type.
    MidiSystemRealtime,

    /// A specialized MIDI atom type.
    MidiActiveSense,

    /// A specialized MIDI atom type.
    MidiClock,

    /// A specialized MIDI atom type.
    MidiContinue,

    /// A specialized MIDI atom type.
    MidiReset,

    /// A specialized MIDI atom type.
    MidiStart,

    /// A specialized MIDI atom type.
    MidiStop,

    /// A specialized MIDI atom type.
    MidiVoiceMessage,

    /// A specialized MIDI atom type.
    MidiAftertouch,

    /// A specialized MIDI atom type.
    MidiBender,

    /// A specialized MIDI atom type.
    MidiChannelPressure,

    /// A specialized MIDI atom type.
    MidiController,

    /// A specialized MIDI atom type.
    MidiNoteOff,

    /// A specialized MIDI atom type.
    MidiNoteOn,

    /// A specialized MIDI atom type.
    MidiProgramChange
}

impl ClassInHierarchy for StdPluginType {
    type SuperclassesIter = vec::IntoIter<StdPluginType>;

    fn direct_superclasses(&self) -> Self::SuperclassesIter {
        use self::StdPluginType::*;
        match self {
            Plugin => vec![].into_iter(),
            ReverbPlugin => vec![Plugin, DelayPlugin, SimulatorPlugin].into_iter(),
            WaveshaperPlugin => vec![DistortionPlugin].into_iter(),
            AmplifierPlugin => vec![DynamicsPlugin].into_iter(),
            CompressorPlugin => vec![DynamicsPlugin].into_iter(),
            EnvelopePlugin => vec![DynamicsPlugin].into_iter(),
            ExpanderPlugin => vec![DynamicsPlugin].into_iter(),
            GatePlugin => vec![DynamicsPlugin].into_iter(),
            LimiterPlugin => vec![DynamicsPlugin].into_iter(),
            AllpassPlugin => vec![FilterPlugin].into_iter(),
            BandpassPlugin => vec![FilterPlugin].into_iter(),
            CombPlugin => vec![FilterPlugin].into_iter(),
            EQPlugin => vec![FilterPlugin].into_iter(),
            MultiEQPlugin => vec![EQPlugin].into_iter(),
            ParaEQPlugin => vec![EQPlugin].into_iter(),
            HighpassPlugin => vec![FilterPlugin].into_iter(),
            LowpassPlugin => vec![FilterPlugin].into_iter(),
            ConstantPlugin => vec![GeneratorPlugin].into_iter(),
            InstrumentPlugin => vec![GeneratorPlugin].into_iter(),
            OscillatorPlugin => vec![GeneratorPlugin].into_iter(),
            ChorusPlugin => vec![ModulatorPlugin].into_iter(),
            FlangerPlugin => vec![ModulatorPlugin].into_iter(),
            PhaserPlugin => vec![ModulatorPlugin].into_iter(),
            PitchPlugin => vec![SpectralPlugin].into_iter(),
            AnalyserPlugin => vec![UtilityPlugin].into_iter(),
            ConverterPlugin => vec![UtilityPlugin].into_iter(),
            FunctionPlugin => vec![UtilityPlugin].into_iter(),
            MixerPlugin => vec![UtilityPlugin].into_iter(),
            _ => vec![Plugin].into_iter()
        }
    }
}

impl ClassInHierarchy for StdAtomType {
    type SuperclassesIter = option::IntoIter<StdAtomType>;

    fn direct_superclasses(&self) -> Self::SuperclassesIter {
        use self::StdAtomType::*;
        match self {
            Atom => None.into_iter(),
            Double => Some(Number).into_iter(),
            Float => Some(Number).into_iter(),
            Int => Some(Number).into_iter(),
            Long => Some(Number).into_iter(),
            Uri => Some(String).into_iter(),
            Path => Some(Uri).into_iter(),
            Sound => Some(Vector).into_iter(),

            MidiSystemMessage => Some(MidiEvent).into_iter(),
            MidiSystemCommon => Some(MidiSystemMessage).into_iter(),
            MidiQuarterFrame => Some(MidiSystemCommon).into_iter(),
            MidiSongPosition => Some(MidiSystemCommon).into_iter(),
            MidiSongSelect => Some(MidiSystemCommon).into_iter(),
            MidiTuneRequest => Some(MidiSystemCommon).into_iter(),
            MidiSystemExclusive => Some(MidiSystemMessage).into_iter(),
            MidiSystemRealtime => Some(MidiSystemMessage).into_iter(),
            MidiActiveSense => Some(MidiSystemRealtime).into_iter(),
            MidiClock => Some(MidiSystemRealtime).into_iter(),
            MidiContinue => Some(MidiSystemRealtime).into_iter(),
            MidiReset => Some(MidiSystemRealtime).into_iter(),
            MidiStart => Some(MidiSystemRealtime).into_iter(),
            MidiStop => Some(MidiSystemRealtime).into_iter(),
            MidiVoiceMessage => Some(MidiEvent).into_iter(),
            MidiAftertouch => Some(MidiVoiceMessage).into_iter(),
            MidiBender => Some(MidiVoiceMessage).into_iter(),
            MidiChannelPressure => Some(MidiVoiceMessage).into_iter(),
            MidiController => Some(MidiVoiceMessage).into_iter(),
            MidiNoteOff => Some(MidiVoiceMessage).into_iter(),
            MidiNoteOn => Some(MidiVoiceMessage).into_iter(),
            MidiProgramChange => Some(MidiVoiceMessage).into_iter(),

            _ => Some(Atom).into_iter()
        }
    }
}