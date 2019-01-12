//! Representation of LV2 ports.

use std::collections::BTreeSet;

use crate::rdf_util::KnownOrUnknown;
use crate::bundle_model::subclasses::StdAtomType;

/// Represents the buffer types that an LV2 atom port can accept.
///
/// Note: This type's implementations of [`Ord`](std::cmp::Ord) and
/// [`PartialOrd`](std::cmp::PartialOrd) have little semantic meaning, and exist mainly for use with
/// collections that require an ordered element type.
#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct AtomBufferType {
    /// Main atom type for the atom buffer.
    main_type: KnownOrUnknown<StdAtomType>,

    /// Set of element types accepted by the atom buffer. This is used for homogeneous collection
    /// atom types like Vector.
    element_types: BTreeSet<AtomBufferType>
}

/// Identifiers for standard port buffer types understood by this crate.
#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum StdPortBufferType {
    /// Indicates that the port operates on single-channel audio-rate sample data. Samples are
    /// expected to be of the C `float` type.
    AudioPort,

    /// Indicates that the port operates on single-channel control-rate sample data (i.e. one sample
    /// per plugin `run()` call). Samples are expected to be of the C `float` type.
    ControlPort,

    /// Similar to [`AudioPort`](self::StdPortBufferType::AudioPort), except this type of port
    /// explicitly handles control signals (at audio rate) instead of audio signals.
    CVPort,

    /// Indicates that the port operates on LV2 atoms from a specified set of allowable atom types.
    AtomPort {
        /// The types of LV2 atoms supported by the port.
        allowed_types: BTreeSet<AtomBufferType>
    }
}

/// Representation of an LV2 port.
///
/// Note: This type's implementations of [`Ord`](std::cmp::Ord) and
/// [`PartialOrd`](std::cmp::PartialOrd) have little semantic meaning, and exist mainly for use with
/// collections that require an ordered element type.
#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct Port {
    /// Indicates whether this port can receive input. A port can be an input port, an output port,
    /// both, or neither.
    is_input_port: bool,

    /// Indicates whether this port can send output. A port can be an input port, an output port,
    /// both, or neither.
    is_output_port: bool,

    /// Indicates whether the plugin can change the buffer type of this port.
    is_auto_morphable: bool,

    /// Buffer types to which this port can be morphed by the host. An empty set means the host
    /// cannot change the buffer type.
    host_morph_types: BTreeSet<KnownOrUnknown<StdPortBufferType>>,

    /// Buffer types supported by this port, before any morphing occurs.
    buffer_types: BTreeSet<KnownOrUnknown<StdPortBufferType>>
}