//! Representation of LV2 ports.

use rayon::iter::{IntoParallelRefIterator, IterBridge, ParallelBridge};
use crate::bundle_model::{HasRelatedSet, NameRelation, ShortNameRelation, DocRelation, TypeRelation, LabelRelation, GenericRelation, IdentifiedBy, OptionallyIdentifiedBy};
use crate::bundle_model::impl_util::{KnownAndUnknownSet, DocumentedImpl, NamedImpl};
use crate::bundle_model::constants::{PortType, PortDesignation, PortChannel, PortProperty};
use crate::bundle_model::unknowns::{UnknownPortType, UnknownPortDesignation, UnknownPortProperty};
use crate::bundle_model::symbol::Symbol;
use crate::rdf_util::Literal;
use enumset::{EnumSet, EnumSetIter};
use std::collections::BTreeSet;
use ordered_float::OrderedFloat;
use num_bigint::BigUint;

/// Represents a scale point, i.e. a special marked value for a control port.
#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct ScalePoint {
    /// Labels to be displayed in association with the scale point.
    labels: BTreeSet<Literal>,

    /// Control port value for the scale point.
    value: OrderedFloat<f32>
}

impl<'a> HasRelatedSet<'a, LabelRelation, Literal> for ScalePoint {
    type BorrowedElt = &'a Literal;
    type SetIter = <BTreeSet<Literal> as IntoParallelRefIterator<'a>>::Iter;

    fn set_iter(&'a self) -> Self::SetIter {
        self.labels.par_iter()
    }
}

/// Representation of an LV2 port.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PortInfo {
    /// Set of LV2 port types to which the port belongs.
    port_types: KnownAndUnknownSet<PortType, UnknownPortType>,

    // TODO: Make a separate type for the port index so the IdentifiedBy impl is clearer?

    /// Port index.
    index: u32,

    /// Port symbol.
    symbol: Option<Symbol>,

    /// Name and short name information.
    named_impl: NamedImpl,

    /// Documentation information.
    documented_impl: DocumentedImpl,

    /// Default value for the port.
    default_value: Option<Literal>,

    /// Soft maximum value for the port. Hosts *are* allowed to set the value higher.
    max_value: Option<Literal>,

    /// Soft minimum value for the port. Hosts *are* allowed to set the value lower.
    min_value: Option<Literal>,

    /// Scale points for the port, i.e. special marked values.
    scale_pts: BTreeSet<ScalePoint>,

    /// LV2 port properties that apply to the port.
    port_props: KnownAndUnknownSet<PortProperty, UnknownPortProperty>,

    /// Standard LV2 designations that apply to the port.
    designations: EnumSet<PortDesignation>,

    /// Standard LV2 channel designations that apply to the port.
    channel_designations: EnumSet<PortChannel>,

    /// Unknown LV2 designations (including channel designations) that apply to the port.
    unknown_designations: BTreeSet<UnknownPortDesignation>,

    /// Indicates how 'important' it is to display the port, for scenarios where not all ports can
    /// be displayed. A higher value means higher priority.
    display_priority: Option<BigUint>,

    /// Indicates into how many evenly spaced steps the port's range should be divided, when using a
    /// step-based controller such as arrow keys.
    range_steps: Option<BigUint>,

    /// Minimum allowed buffer size for the port, in bytes.
    min_buffer_size: Option<BigUint>,

    /// Can be used to specify that the port's buffer must be at least as large as the largest of
    /// some other set of port buffer sizes. LV2 symbols are used to identify the ports in the set.
    buffer_as_large_as: BTreeSet<Symbol>
}

impl IdentifiedBy<u32> for PortInfo {
    fn id(&self) -> &u32 {
        &self.index
    }
}

impl OptionallyIdentifiedBy<Symbol> for PortInfo {
    fn id(&self) -> Option<&Symbol> {
        self.symbol.as_ref()
    }
}

impl<'a> HasRelatedSet<'a, TypeRelation, PortType> for PortInfo {
    type BorrowedElt = PortType;
    type SetIter = IterBridge<EnumSetIter<PortType>>;

    fn set_iter(&'a self) -> Self::SetIter {
        self.port_types.knowns_iter()
    }
}

impl<'a> HasRelatedSet<'a, TypeRelation, UnknownPortType> for PortInfo {
    type BorrowedElt = &'a UnknownPortType;
    type SetIter = <BTreeSet<UnknownPortType> as IntoParallelRefIterator<'a>>::Iter;

    fn set_iter(&'a self) -> Self::SetIter {
        self.port_types.unknowns_iter()
    }
}

impl<'a> HasRelatedSet<'a, NameRelation, Literal> for PortInfo {
    type BorrowedElt = &'a Literal;
    type SetIter = <BTreeSet<Literal> as IntoParallelRefIterator<'a>>::Iter;

    fn set_iter(&'a self) -> Self::SetIter {
        self.named_impl.names.par_iter()
    }
}

impl<'a> HasRelatedSet<'a, ShortNameRelation, Literal> for PortInfo {
    type BorrowedElt = &'a Literal;
    type SetIter = <BTreeSet<Literal> as IntoParallelRefIterator<'a>>::Iter;

    fn set_iter(&'a self) -> Self::SetIter {
        self.named_impl.short_names.par_iter()
    }
}

impl<'a> HasRelatedSet<'a, DocRelation, Literal> for PortInfo {
    type BorrowedElt = &'a Literal;
    type SetIter = <BTreeSet<Literal> as IntoParallelRefIterator<'a>>::Iter;

    fn set_iter(&'a self) -> Self::SetIter {
        self.documented_impl.documentation.par_iter()
    }
}

impl<'a> HasRelatedSet<'a, GenericRelation, PortDesignation> for PortInfo {
    type BorrowedElt = PortDesignation;
    type SetIter = IterBridge<EnumSetIter<PortDesignation>>;

    fn set_iter(&'a self) -> Self::SetIter {
        self.designations.iter().par_bridge()
    }
}

impl<'a> HasRelatedSet<'a, GenericRelation, PortChannel> for PortInfo {
    type BorrowedElt = PortChannel;
    type SetIter = IterBridge<EnumSetIter<PortChannel>>;

    fn set_iter(&'a self) -> Self::SetIter {
        self.channel_designations.iter().par_bridge()
    }
}

impl<'a> HasRelatedSet<'a, GenericRelation, UnknownPortDesignation> for PortInfo {
    type BorrowedElt = &'a UnknownPortDesignation;
    type SetIter = <BTreeSet<UnknownPortDesignation> as IntoParallelRefIterator<'a>>::Iter;

    fn set_iter(&'a self) -> Self::SetIter {
        self.unknown_designations.par_iter()
    }
}

impl<'a> HasRelatedSet<'a, GenericRelation, PortProperty> for PortInfo {
    type BorrowedElt = PortProperty;
    type SetIter = IterBridge<EnumSetIter<PortProperty>>;

    fn set_iter(&'a self) -> Self::SetIter {
        self.port_props.knowns_iter()
    }
}

impl<'a> HasRelatedSet<'a, GenericRelation, UnknownPortProperty> for PortInfo {
    type BorrowedElt = &'a UnknownPortProperty;
    type SetIter = <BTreeSet<UnknownPortProperty> as IntoParallelRefIterator<'a>>::Iter;

    fn set_iter(&'a self) -> Self::SetIter {
        self.port_props.unknowns_iter()
    }
}

impl<'a> HasRelatedSet<'a, GenericRelation, ScalePoint> for PortInfo {
    type BorrowedElt = &'a ScalePoint;
    type SetIter = <BTreeSet<ScalePoint> as IntoParallelRefIterator<'a>>::Iter;

    fn set_iter(&'a self) -> Self::SetIter {
        self.scale_pts.par_iter()
    }
}