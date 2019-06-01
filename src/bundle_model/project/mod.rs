//! Representation of LV2 project information.

use crate::rdf_util::{Iri, Literal};
use std::collections::btree_set::BTreeSet;
use rayon::iter::IntoParallelRefIterator;
use crate::bundle_model::{OptionallyIdentifiedBy, HasRelatedSet, NameRelation, ShortNameRelation};
use crate::bundle_model::symbol::Symbol;
use crate::bundle_model::impl_util::NamedImpl;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProjectInfo {
    /// The IRI identifying the project, if specified.
    iri: Option<Iri>,

    /// LV2 symbol identifying the project, if specified.
    symbol: Option<Symbol>,

    /// Name and short name information.
    named_impl: NamedImpl
}

impl OptionallyIdentifiedBy<Iri> for ProjectInfo {
    fn id(&self) -> Option<&Iri> {
        self.iri.as_ref()
    }
}

impl OptionallyIdentifiedBy<Symbol> for ProjectInfo {
    fn id(&self) -> Option<&Symbol> {
        self.symbol.as_ref()
    }
}

impl<'a> HasRelatedSet<'a, NameRelation, Literal> for ProjectInfo {
    type BorrowedElt = &'a Literal;
    type SetIter = <BTreeSet<Literal> as IntoParallelRefIterator<'a>>::Iter;

    fn set_iter(&'a self) -> Self::SetIter {
        self.named_impl.names.par_iter()
    }
}

impl<'a> HasRelatedSet<'a, ShortNameRelation, Literal> for ProjectInfo {
    type BorrowedElt = &'a Literal;
    type SetIter = <BTreeSet<Literal> as IntoParallelRefIterator<'a>>::Iter;

    fn set_iter(&'a self) -> Self::SetIter {
        self.named_impl.short_names.par_iter()
    }
}