//! Representation of LV2 ports.

use std::collections::BTreeSet;

use enumset::EnumSet;

use num_bigint::BigUint;

use ordered_float::OrderedFloat;

use crate::rdf_util::Literal;
use crate::bundle_model::subclasses::StdAtomType;

/// Contains extra information associated with an LV2 atom port.
///
/// Note: This type's implementations of [`Ord`](std::cmp::Ord) and
/// [`PartialOrd`](std::cmp::PartialOrd) have little semantic meaning, and exist mainly for use with
/// collections that require an ordered element type.
#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct AtomPortInfo {
    /// Main atom types that the port can accept in its buffer.
    main_types: EnumSet<StdAtomType>,

    /// Element types accepted by the element-accepting atom types in
    /// [`main_types`](self::AtomPortInfo::main_types). Must be empty if
    /// [`expects_element_type`](crate::bundle_model::subclasses::StdAtomType::expects_element_type)
    /// is not true for any of the atom types in
    /// [`main_types`](self::AtomPortInfo::main_types).
    element_types: EnumSet<StdAtomType>
}

/// Identifiers for port buffer types understood by this crate.
#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum PortBufferType {
    /// Indicates that the port operates on single-channel audio-rate sample data. Samples are
    /// expected to be of the C `float` type.
    AudioPort,

    /// Indicates that the port operates on single-channel control-rate sample data (i.e. one sample
    /// per plugin `run()` call). Samples are expected to be of the C `float` type.
    ControlPort,

    /// Similar to [`AudioPort`](self::PortBufferType::AudioPort), except this type of port
    /// explicitly handles control signals (at audio rate) instead of audio signals.
    CVPort,

    /// Indicates that the port operates on LV2 atoms from a specified set of allowable atom types.
    AtomPort {
        /// Additional information about the port, including the allowed atom type(s).
        atom_port_info: AtomPortInfo
    }
}

/// Represents a 'scale point', i.e. a special marked value for a control port.
#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct ScalePoint {
    /// Labels to be displayed in association with the scale point.
    labels: BTreeSet<Literal>,

    /// Control port value for the scale point.
    value: OrderedFloat<f32>
}

enum_set_type! {
    /// Enumeration of measurement units defined by the LV2 standard.
    // TODO: Maybe move this to a different module?
    pub enum StdUnit {
        Bar,
        Beat,

        /// Beats per minute.
        Bpm,

        Cent,
        Centimeter,
        Coefficient,
        Decibel,
        Degree,
        AudioFrame,
        Hertz,
        Inch,
        Kilohertz,
        Kilometer,
        Meter,
        Megahertz,
        MidiNote,
        Mile,
        Minute,
        Millimeter,
        Millisecond,
        Octave,
        Percent,
        Second,

        /// Semitone using 12-tone equal temperament.
        Semitone12Tet
    }
}

enum_set_type! {
    /// Enumeration of boolean properties a port can have. Several, but not all, of these flags
    /// correspond to instances of the `lv2:PortProperty` RDF class from the LV2 standard.
    ///
    /// Note: This type's implementations of [`Ord`](std::cmp::Ord) and
    /// [`PartialOrd`](std::cmp::PartialOrd) have little semantic meaning, and exist mainly for use
    /// with collections that require an ordered element type.
    enum PortFlags {
        /// Indicates that the port can receive input. A port can have input, output, both, or
        /// neither.
        InputPort,

        /// Indicates that the port can produce output. A port can have input, output, both, or
        /// neither.
        OutputPort,

        /// Indicates that connecting the port to a non-null buffer is optional.
        ConnOptional,

        /// Indicates that the port's only valid values are those defined by its scale points.
        Enumeration,

        /// Indicates that the port's only valid values are integers.
        IntOnly,

        /// Indicates that the port is a `sidechain` rather than a main port. Hosts are not required
        /// to connect anything to a sidechain port, not even a null pointer.
        SideChain,

        /// Indicates that the port reports the plugin's latency in samples.
        ReportsLatency,

        /// Indicates that the port's bounds (e.g. maximum and minimum) should be interpreted as
        /// multiples of the sample rate.
        BoundsRelativeToSampleRate,

        /// Indicates that the port's value represents a boolean. Ports with this flag should
        /// interpret positive values as true, and zero or negative values as false.
        Toggle,

        /// Indicates that changing the port's input value may cause audio artifacts.
        ChangeCausesArtifacts,

        /// Indicates that the port's signal should be interpreted as a smooth modulation signal.
        ContinuousCV,

        /// Indicates that the port's signal should be interpreted as a discrete modulation signal.
        DiscreteCV,

        /// Indicates that changing the port's input value may trigger expensive computation.
        ChangeExpensive,

        /// Indicates that the port's bounds (e.g. maximum and minimum) should be considered strict.
        StrictBounds,

        /// Indicates that the port's value is on a logarithmic scale.
        Logarithmic,

        /// Indicates that the port is not intended to receive a modulation/automation signal.
        NotAutomatic,

        /// Indicates that the port is not intended to be shown as a control in the GUI.
        NotOnGui,

        /// Indicates that the port represents a trigger, and should be reset to its default value
        /// when not being triggered.
        Trigger,

        /// Indicates that the port may automatically change its port type whenever a port on the
        /// same plugin instance is morphed by the host.
        AutoMorphable,

        /// Designates the port as a main control channel. Typically used for a MIDI channel that
        /// controls an instrument plugin.
        ControlChannel,

        /// Designates the port as a center channel.
        CenterChannel,

        /// Designates the port as a center left channel.
        CenterLeftChannel,

        /// Designates the port as a center right channel.
        CenterRightChannel,

        /// Designates the port as a left channel.
        LeftChannel,

        /// Designates the port as a low-frequency effects channel.
        LowFrequencyEffectsChannel,

        /// Designates the port as a rear center channel.
        RearCenterChannel,

        /// Designates the port as a rear left channel.
        RearLeftChannel,

        /// Designates the port as a rear right channel.
        RearRightChannel,

        /// Designates the port as a right channel.
        RightChannel,

        /// Designates the port as a side channel.
        SideChannel,

        /// Designates the port as a side left channel.
        SideLeftChannel,

        /// Designates the port as a side right channel.
        SideRightChannel,

        /// Designates the port as representing an amplitude.
        AmplitudeDesignation,

        /// Designates the port as representing an envelope's attack duration.
        AttackDesignation,

        /// Designates the port as a boolean bypass channel. A value of true means bypassed.
        BypassDesignation,

        /// Designates the port as representing a cutoff frequency.
        CutoffFrequencyDesignation,

        /// Designates the port as representing an envelope's decay duration.
        DecayDesignation,

        /// Designates the port as representing an envelope's delay duration.
        DelayDesignation,

        /// Designates the port as representing a dry level for a signal.
        DryLevelDesignation,

        /// Designates the port as representing a frequency.
        FrequencyDesignation,

        /// Designates the port as representing a gain in decibels.
        GainDesignation,

        /// Designates the port as representing an envelope's hold duration.
        HoldDesignation,

        /// Designates the port as representing a rectangular wave's pulse width.
        PulseWidthDesignation,

        /// Designates the port as representing a compression ratio.
        CompressionRatioDesignation,

        /// Designates the port as representing an envelope's release duration.
        ReleaseDesignation,

        /// Designates the port as representing a filter resonance.
        ResonanceDesignation,

        /// Designates the port as representing a sample rate in Hertz.
        SampleRateDesignation,

        /// Designates the port as representing an envelope's sustain level.
        SustainDesignation,

        /// Designates the port as representing a compression threshold.
        CompressionThresholdDesignation,

        /// Designates the port as representing a waveform.
        WaveformDesignation,

        /// Designates the port as representing a wet/dry ratio.
        WetDryRatioDesignation,

        /// Designates the port as representing a wet level for a signal.
        WetLevelDesignation
    }
}

/// Representation of an LV2 port.
///
/// Note: This type's implementations of [`Ord`](std::cmp::Ord) and
/// [`PartialOrd`](std::cmp::PartialOrd) have little semantic meaning, and exist mainly for use with
/// collections that require an ordered element type.
#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct Port {
    /// Boolean properties.
    flags: EnumSet<PortFlags>,

    /// Index of the port.
    index: Option<u32>,

    /// Symbol identifying the port.
    symbol: Option<String>,

    /// Labels to be displayed in association with the port. Unlike the
    /// [`symbol`](self::Port::symbol), these labels do not act as identifiers.
    names: BTreeSet<Literal>,

    /// Short labels (no more than 16 characters) to be displayed in association with the port.
    /// Unlike the [`symbol`](self::Port::symbol), these labels do not act as identifiers.
    short_names: BTreeSet<Literal>,

    /// Display priority for the port, to be used when not all ports can be shown in the UI. Higher
    /// values mean more priority.
    display_priority: Option<BigUint>,

    /// Maximum useful value for the port.
    max_value: Option<Literal>,

    /// Minimum useful value for the port.
    min_value: Option<Literal>,

    /// Default value for the port.
    default_value: Option<Literal>,

    /// Identifies the units for the port value.
    unit: Option<StdUnit>,

    /// Set of the port's scale points, i.e. marked values that are special in some way.
    scale_points: BTreeSet<ScalePoint>,

    /// Number of evenly spaced steps to use (between the maximum and minimum values) when editing
    /// the port value through a stepwise interface, such as arrow keys on the keyboard.
    num_range_steps: Option<BigUint>,

    /// Buffer types to which this port can be morphed by the host. An empty set means the host
    /// cannot change the buffer type.
    host_morph_types: BTreeSet<PortBufferType>,

    /// Buffer types supported by this port, before any morphing occurs.
    buffer_types: BTreeSet<PortBufferType>,
}