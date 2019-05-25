//! Representation of LV2 project information.

use crate::rdf_util::{Iri, Literal};
use std::collections::btree_set::BTreeSet;
use rayon::iter::IntoParallelRefIterator;
use crate::bundle_model::{OptionallyIdentifiedBy, Named};
use crate::bundle_model::symbol::Symbol;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProjectInfo {
    /// The IRI identifying the project, if specified.
    iri: Option<Iri>,

    /// LV2 symbol identifying the project, if specified.
    symbol: Option<Symbol>,

    /// Human-readable project names. Multiple language-tagged literals can be used.
    names: BTreeSet<Literal>,

    /// Short names for the project, up to 16 Unicode grapheme clusters each. Multiple
    /// language-tagged literals can be used.
    short_names: BTreeSet<Literal>,
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

impl<'a> Named<'a> for ProjectInfo {
    type NamesIter = <BTreeSet<Literal> as IntoParallelRefIterator<'a>>::Iter;
    type ShortNamesIter = <BTreeSet<Literal> as IntoParallelRefIterator<'a>>::Iter;

    fn names_iter(&'a self) -> Self::NamesIter {
        self.names.par_iter()
    }

    fn short_names_iter(&'a self) -> Self::ShortNamesIter {
        self.short_names.par_iter()
    }
}