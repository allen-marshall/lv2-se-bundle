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

/// Trait for types that can contain LV2 name information, including multilingual names.
pub trait Named<'a> {
    /// Type of (parallel) iterator returned by [`names_iter`](self::Named::names_iter).
    type NamesIter: ParallelIterator<Item = &'a Literal>;

    /// Type of (parallel) iterator returned by [`short_names_iter`](self::Named::short_names_iter).
    type ShortNamesIter: ParallelIterator<Item = &'a Literal>;

    /// Gets a (parallel) iterator over the human-readable name literals for the entity. An entity
    /// may have multiple language-tagged name literals to provide multilingual naming. It also may
    /// have no name literals at all, in which case the returned iterator must be empty.
    fn names_iter(&'a self) -> Self::NamesIter;

    /// Gets a (parallel) iterator over the short name literals for the entity. An entity may have
    /// multiple language-tagged short name literals to provide multilingual naming. It also may
    /// have no short name literals at all, in which case the returned iterator must be empty. Note
    /// that the set of languages supported by the short name literals doesn't have to correspond to
    /// the set of languages supported by the long name literals.
    ///
    /// Implementors must ensure that each returned literal's contents (excluding any language tag
    /// or data type information) does not exceed 16 Unicode grapheme clusters.
    fn short_names_iter(&'a self) -> Self::ShortNamesIter;
}

/// Trait for types that can contain LV2 documentation information, including multilingual
/// documentation.
pub trait Documented<'a> {
    /// Type of (parallel) iterator returned by
    /// [`documentation_iter`](self::Documented::documentation_iter).
    type DocumentationIter: ParallelIterator<Item = &'a Literal>;

    /// Gets a (parallel) iterator over the documentation literals for the entity. An entity may
    /// have multiple language-tagged documentation literals to provide multilingual documentation.
    /// It also may have no documentation literals at all, in which case the returned iterator must
    /// be empty. The LV2 specification specifies that the contents of each literal should be "a
    /// valid XHTML Basic 1.1 fragment suitable for use as the content of the \<body\> element."
    fn documentation_iter(&'a self) -> Self::DocumentationIter;
}

/// Defines basic behavior for loadable LV2 entities described in an LV2 bundle, such as plugins,
/// UIs, and dynamic manifest generators.
pub trait Loadable {
    /// Gets an IRI pointing to the shared library that implements the entity. If the IRI is
    /// relative, it should be interpreted relative to the bundle path.
    fn binary(&self) -> Option<&Iri>;
}

/// Trait for types that can specify a set of "provided" elements.
///
/// # Parameters
/// - `T`: Type of provided element.
pub trait Provider<'a, T: Sync> {
    /// Type of borrowed element returned by the iterators produced by this trait's methods.
    type BorrowedElt: Borrow<T> + Send;

    /// Type of (parallel) iterator returned by [`provided_iter`](self::Provider::provided_iter).
    type ProvidedIter: ParallelIterator<Item = Self::BorrowedElt>;

    /// Gets a (parallel) iterator over the provided elements. The iterator must not repeat items.
    fn provided_iter(&'a self) -> Self::ProvidedIter;

    /// Checks if the specified element is provided. Must return true if and only if `to_check` is
    /// in the set that would be returned by [`provided_iter`](self::Provider::provided_iter). The
    /// default implementation calls [`provided_iter`](self::Provider::provided_iter) and searches
    /// for `to_check`; a more efficient implementation is likely possible for most implementing
    /// types.
    ///
    /// # Parameters
    /// - `to_check`: The providable element to check for.
    fn provides(&'a self, to_check: &T) -> bool
        where T: Eq
    {
        self.provided_iter().any(|elt| elt.borrow() == to_check)
    }
}

/// Trait for types that can specify sets of "required" and "optionally supported" elements.
///
/// # Parameters
/// - `T`: Type of supported element.
pub trait Requirer<'a, T: Sync> {
    /// Type of borrowed element returned by the iterators produced by this trait's methods.
    type BorrowedElt: Borrow<T> + Send;

    /// Type of (parallel) iterator returned by
    /// [`required_iter`](self::Requirer::required_iter).
    type RequiredIter: ParallelIterator<Item = Self::BorrowedElt>;

    /// Type of (parallel) iterator returned by
    /// [`optionally_supported_iter`](self::Requirer::optionally_supported_iter).
    type OptionallySupportedIter: ParallelIterator<Item = Self::BorrowedElt>;

    /// Gets a (parallel) iterator over the required elements. The iterator must not repeat items,
    /// and its contents must be disjoint from those returned by
    /// [`optionally_supported_iter`](self::Requirer::optionally_supported_iter).
    fn required_iter(&'a self) -> Self::RequiredIter;

    /// Checks if the specified element is required. Must return true if and only if
    /// `to_check` is in the set that would be returned by
    /// [`required_iter`](self::Requirer::required_iter).
    /// The default implementation calls [`required_iter`](self::Requirer::required_iter) and
    /// searches for `to_check`; a more efficient implementation is likely to be possible for most
    /// implementing types.
    ///
    /// # Parameters
    /// - `to_check`: The element to check for.
    fn requires(&'a self, to_check: &T) -> bool
        where T: Eq
    {
        self.required_iter().any(|elt| elt.borrow() == to_check)
    }

    /// Gets a (parallel) iterator over the optionally supported elements. The iterator must not
    /// repeat items, and its contents must be disjoint from those returned by
    /// [`required_iter`](self::Requirer::required_iter).
    fn optionally_supported_iter(&'a self) -> Self::OptionallySupportedIter;

    /// Checks if the specified element is optionally required. Must return true if and only if
    /// `to_check` is in the set that would be returned by
    /// [`optionally_supported_iter`](self::Requirer::optionally_supported_iter).
    /// The default implementation calls
    /// [`optionally_supported_iter`](self::Requirer::optionally_supported_iter) and searches for
    /// `to_check`; a more efficient implementation is likely to be possible for most implementing
    /// types.
    ///
    /// # Parameters
    /// - `to_check`: The element to check for.
    fn optionally_supports(&'a self, to_check: &T) -> bool
        where T: Eq
    {
        self.optionally_supported_iter().any(|elt| elt.borrow() == to_check)
    }

    /// Gets a (parallel) iterator over the elements that are supported, as either required elements
    /// or optional elements. The iterator must not repeat items, and must contain a set equal to
    /// the union of the sets returned by [`required_iter`](self::Requirer::required_iter) and
    /// [`optionally_supported_iter`](self::Requirer::optionally_supported_iter).
    fn supported_iter(&'a self) -> Chain<Self::RequiredIter, Self::OptionallySupportedIter> {
        self.required_iter().chain(self.optionally_supported_iter())
    }

    /// Checks if the specified element is supported, as either a required element or an optional
    /// element. Must return true if and only if `to_check` is in the set that would be returned by
    /// [`supported_iter`](self::Requirer::supported_iter).
    ///
    /// # Parameters
    /// - `to_check`: The element to check for.
    fn supports(&'a self, to_check: &T) -> bool
        where T: Eq
    {
        self.requires(to_check) || self.optionally_supports(to_check)
    }
}