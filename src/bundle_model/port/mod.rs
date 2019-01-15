//! Representation of LV2 ports.

use std::collections::BTreeSet;

use enumset::EnumSet;

use num_bigint::BigUint;

use ordered_float::OrderedFloat;

use crate::rdf_util::Literal;
use crate::bundle_model::constants::{AtomType, Unit, PortProperty, PortDesignation, PortChannel};

/// Contains extra information associated with an LV2 atom port.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct AtomPortInfo {
    /// Main atom types that the port can accept in its buffer.
    main_types: EnumSet<AtomType>,

    /// Element types accepted by the element-accepting atom types in
    /// [`main_types`](self::AtomPortInfo::main_types). Must be empty if
    /// [`expects_element_type`](crate::bundle_model::subclasses::StdAtomType::expects_element_type)
    /// is not true for any of the atom types in [`main_types`](self::AtomPortInfo::main_types).
    element_types: EnumSet<AtomType>
}

/// Identifiers for port buffer types understood by this crate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
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
    /// Enumeration of boolean properties a port can have. Several, but not all, of these flags
    /// correspond to instances of the `lv2:PortProperty` RDF class from the LV2 standard.
    enum PortFlags {
        /// Indicates that the port can receive input. A port can have input, output, both, or
        /// neither.
        InputPort,

        /// Indicates that the port can produce output. A port can have input, output, both, or
        /// neither.
        OutputPort,

        /// Indicates that the port may automatically change its port type whenever a port on the
        /// same plugin instance is morphed by the host.
        AutoMorphable
    }
}

/// Representation of an LV2 port.
#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct Port {
    /// Some boolean properties.
    flags: EnumSet<PortFlags>,

    /// Standard LV2 port properties that apply to the port.
    port_props: EnumSet<PortProperty>,

    /// Standard LV2 designations that apply to the port.
    designations: EnumSet<PortDesignation>,

    /// Standard LV2 channel designations that apply to the port.
    channel_designations: EnumSet<PortChannel>,

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
    unit: Option<Unit>,

    /// Set of the port's scale points, i.e. marked values that are special in some way.
    scale_points: BTreeSet<ScalePoint>,

    /// Number of evenly spaced steps to use (between the maximum and minimum values) when editing
    /// the port value through a stepwise interface, such as arrow keys on the keyboard.
    num_range_steps: Option<BigUint>,

    /// Standard buffer types to which this port can be morphed by the host. An empty set means the
    /// host cannot change the buffer type.
    host_morph_types: BTreeSet<PortBufferType>,

    /// Standard buffer types supported by this port, before any morphing occurs.
    buffer_types: BTreeSet<PortBufferType>
}