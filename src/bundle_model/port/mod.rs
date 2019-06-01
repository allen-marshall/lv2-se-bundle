//! Representation of LV2 ports.

use rayon::iter::{ParallelIterator, IntoParallelRefIterator, IterBridge};
use crate::bundle_model::{HasRelatedSet, NameRelation, ShortNameRelation, DocRelation, TypeRelation};
use crate::bundle_model::impl_util::{KnownAndUnknownSet, DocumentedImpl, NamedImpl};
use crate::bundle_model::constants::{PortType, PortDesignation, PortChannel};
use crate::bundle_model::unknowns::{UnknownPortType, UnknownPortDesignation};
use crate::rdf_util::Literal;
use enumset::{EnumSet, EnumSetIter};
use std::collections::BTreeSet;

/// Representation of an LV2 port.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PortInfo {
    /// Set of LV2 port types to which the port belongs.
    port_types: KnownAndUnknownSet<PortType, UnknownPortType>,

    /// Name and short name information.
    named_impl: NamedImpl,

    /// Documentation information.
    documented_impl: DocumentedImpl,

    /// Default value for the port.
    default: Option<Literal>,

    /// Standard LV2 designations that apply to the port.
    designations: EnumSet<PortDesignation>,

    /// Standard LV2 channel designations that apply to the port.
    channel_designations: EnumSet<PortChannel>,

    /// Unknown LV2 designations (including channel designations) that apply to the port.
    unknown_designations: BTreeSet<UnknownPortDesignation>
}

impl PortInfo {
    /// Gets a (parallel) iterator over the known port types to which the port belongs.
    pub fn known_port_types_iter(&self) -> impl ParallelIterator<Item = PortType> {
        self.port_types.knowns_iter()
    }

    /// Gets a (parallel) iterator over the unknown port types to which the port belongs.
    pub fn unknown_port_types_iter(&self) -> impl ParallelIterator<Item = &UnknownPortType> {
        self.port_types.unknowns_iter()
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