//! Defines data structures representing information available in an LV2 bundle's RDF data.

use num_bigint::BigUint;
use rayon::iter::{Chain, ParallelIterator};
use crate::rdf_util::{Iri, Literal};
use std::borrow::Borrow;

pub mod constants;
pub mod implications;
pub mod unknowns;
pub mod symbol;
pub mod project;
pub mod port;
pub mod plugin;
pub mod dyn_manifest;

pub(crate) mod impl_util;

/// Represents a version specification for an LV2 resource, such as a plugin. A resource version
/// consists of a minor version number and a micro version number. There is no major version number,
/// because LV2 uses the resource's identification IRI instead of a version number to track
/// non-backwards-compatible changes. A value of zero for the minor version indicates a pre-release.
/// An odd value for either the minor version or the micro version indicates a development release.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ResourceVersion {
    minor_version: BigUint,
    micro_version: BigUint
}

impl ResourceVersion {
    /// Constructs a resource version object.
    ///
    /// # Parameters
    /// - `minor_version`: The minor version number.
    /// - `micro_version`: The micro version number.
    pub fn new(minor_version: BigUint, micro_version: BigUint) -> Self {
        ResourceVersion {
            minor_version,
            micro_version
        }
    }

    /// Gets the minor version number.
    pub fn minor_version(&self) -> &BigUint {
        &self.minor_version
    }

    /// Gets the micro version number.
    pub fn micro_version(&self) -> &BigUint {
        &self.micro_version
    }

    /// Checks if this version represents a pre-release.
    pub fn is_pre_release(&self) -> bool {
        self.minor_version == 0u8.into()
    }

    /// Checks if this version represents a development release other than a pre-release.
    pub fn is_dev_release(&self) -> bool {
        // Check if the minor version or micro version is odd.
        &self.minor_version % &2u8 == 1u8.into() || &self.micro_version % &2u8 == 1u8.into()
    }
}

/// Trait for types that are required to have an "identifier" of the specified type.
///
/// # Parameters
/// - `T`: Type of identifier.
pub trait IdentifiedBy<T> {
    /// Gets this object's identifier.
    fn id(&self) -> &T;
}

/// Trait for types that may, but are not required to, have an "identifier" of the specified type.
///
/// # Parameters
/// - `T`: Type of identifier.
pub trait OptionallyIdentifiedBy<T> {
    /// Gets this object's identifier, if it has one.
    fn id(&self) -> Option<&T>;
}

/// Defines basic behavior for loadable LV2 entities described in an LV2 bundle, such as plugins,
/// UIs, and dynamic manifest generators.
pub trait Loadable {
    /// Gets an IRI pointing to the shared library that implements the entity. If the IRI is
    /// relative, it should be interpreted relative to the bundle path.
    fn binary(&self) -> Option<&Iri>;
}

/// Trait for types that are in some way related to a set of values. The type of relationship is
/// defined by the `R` parameter.
///
/// # Parameters
/// - `R`: Defines the way in which the set elements relate to the instance that "has" the set.
/// - `T`: Type for elements in the set.
pub trait HasRelatedSet<R, T: Sync> {
    /// Type of borrowed element returned by the set iterator.
    type BorrowedElt: Borrow<T> + Send;

    /// Type of (parallel) iterator returned by [`set_iter`](self::HasRelatedSet::set_iter).
    type SetIter: ParallelIterator<Item = Self::BorrowedElt>;

    /// Gets a (parallel) iterator over the elements of the set. The returned iterator must not
    /// repeat elements, and may be empty.
    fn set_iter(&self) -> Self::SetIter;

    /// Checks if the specified element is in the set. The default implementation calls
    /// [`set_iter`](self::HasRelatedSet::set_iter) and searches for the specified element. A more
    /// efficient implementation is likely possible for most implementing types.
    fn has_elt(&self, to_check: &T) -> bool
        where T: Eq
    {
        self.set_iter().any(|elt| elt.borrow() == to_check)
    }
}

/// Relation type for [`HasRelatedSet`](self::HasRelatedSet), which indicates that the set elements
/// are names for the instance that "has" the set. Multiple language-tagged names are allowed in
/// order to support multilingual naming.
pub enum NameRelation {}

/// Relation type for [`HasRelatedSet`](self::HasRelatedSet), which indicates that the set elements
/// are short names for the instance that "has" the set. This is similar to
/// [`NameRelation`](self::NameRelation), except that the name strings are expected to be no more
/// than 16 characters.
pub enum ShortNameRelation {}

/// Relation type for [`HasRelatedSet`](self::HasRelatedSet), which indicates that the set elements
/// are documentation strings for the instance that "has" the set. Multiple language-tagged
/// documentation strings are allowed in order to support multilingual documentation.
///
/// The LV2 specification specifies that each documentation string's contents should be "a valid
/// XHTML Basic 1.1 fragment suitable for use as the content of the \<body\> element."
pub enum DocRelation {}

/// Relation type for [`HasRelatedSet`](self::HasRelatedSet), which indicates that the set elements
/// are type/class tags for the instance that "has" the set.
pub enum TypeRelation {}

/// Relation type for [`HasRelatedSet`](self::HasRelatedSet), which indicates that the set elements
/// are identifiers for items that are in some way provided by the instance that "has" the set.
pub enum ProvidesRelation {}

/// Relation type for [`HasRelatedSet`](self::HasRelatedSet), which indicates that the set elements
/// are identifiers for items that are in some way required by the instance that "has" the set.
pub enum RequiresRelation {}

/// Relation type for [`HasRelatedSet`](self::HasRelatedSet), which indicates that the set elements
/// are identifiers for items that are in some way optionally supported by the instance that "has"
/// the set.
pub enum OptionallySupportsRelation {}