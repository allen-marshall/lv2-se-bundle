//! Representation of LV2 ports.

use std::collections::BTreeSet;

use enumset::EnumSet;

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
    /// Zero or more labels to be displayed in association with the scale point.
    labels: BTreeSet<Literal>,

    /// Control port value for the scale point.
    value: OrderedFloat<f32>
}

enum_set_type! {
    /// Enumeration of boolean properties a port can have. Several, but not all, of these flags
    /// correspond to instances of the `lv2:PortProperty` RDF class from the LV2 standard.
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
        AutoMorphable
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

    // TODO: Designation.

    index: Option<u32>,

    symbol: Option<String>,

    names: BTreeSet<Literal>,

    short_names: BTreeSet<Literal>,

    max_value: Option<Literal>,

    min_value: Option<Literal>,

    default_value: Option<Literal>,

    scale_points: BTreeSet<ScalePoint>,

    display_priority: Option<Literal>,

    range_steps: Option<Literal>,

    /// Buffer types to which this port can be morphed by the host. An empty set means the host
    /// cannot change the buffer type.
    host_morph_types: BTreeSet<PortBufferType>,

    /// Buffer types supported by this port, before any morphing occurs.
    buffer_types: BTreeSet<PortBufferType>,

//    unit: Option<Unit>
}