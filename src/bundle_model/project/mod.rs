//! Representation of LV2 project information.

use crate::rdf_util::{Iri, Literal};
use std::collections::btree_set::BTreeSet;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use crate::bundle_model::symbol::Symbol;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProjectInfo {
    /// The IRI identifying the project, if specified.
    iri: Option<Iri>,

    /// LV2 symbol identifying the project, if specified.
    symbol: Option<Symbol>,

    /// Human-readable project names. Multiple language-tagged literals can be used. These should be
    /// extracted from the doap:name RDF property.
    names: BTreeSet<Literal>
}

impl ProjectInfo {
    /// Gets the IRI identifying the project. Returns [`None`](std::option::Option::None) if the
    /// bundle does not specify an IRI for the project.
    pub fn iri(&self) -> Option<&Iri> {
        self.iri.as_ref()
    }

    /// Gets the LV2 symbol identifying the project. Returns [`None`](std::option::Option::None) if
    /// the bundle does not specify a symbol for the project.
    pub fn symbol(&self) -> Option<&Symbol> {
        self.symbol.as_ref()
    }

    /// Gets an iterator over the human-readable name literals for the project. A project may have
    /// multiple language-tagged name literals to provide multilingual naming.
    pub fn names(&self) -> impl ParallelIterator<Item = &Literal> {
        self.names.par_iter()
    }
}