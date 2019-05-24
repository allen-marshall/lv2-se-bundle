//! Defines data structures representing information available in an LV2 bundle's RDF data.

use num_bigint::BigUint;
use crate::rdf_util::{Iri, Literal};
use crate::bundle_model::symbol::Symbol;
use crate::bundle_model::constants::{ExtensionData, HostFeature};
use std::iter::Chain;

pub mod constants;
pub mod symbol;
pub mod project;
pub mod port;
pub mod plugin;
pub mod dyn_manifest;

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

/// Defines basic behavior for loadable LV2 entities described in an LV2 bundle, such as plugins,
/// UIs, and dynamic manifest generators.
pub trait LoadableEntity {
    /// Gets the IRI that identifies the entity. Returns [`None`](std::option::Option::None) if the
    /// LV2 bundle data does not specify an IRI for the entity.
    fn iri(&self) -> Option<&Iri>;

    /// Gets an IRI pointing to the shared library that implements the entity. If the IRI is
    /// relative, it should be interpreted relative to the bundle path.
    fn binary(&self) -> Option<&Iri>;

    /// Gets the LV2 symbol identifying the entity. Note that the usual way of identifying a
    /// loadable entity is by its IRI. However, the symbol may be useful e.g. for plugin searching
    /// in a host's UI. Returns [`None`](std::option::Option::None) if the bundle does not specify a
    /// symbol for the entity.
    fn symbol(&self) -> Option<&Symbol>;
}

/// Trait for types that can contain LV2 name information, including multilingual names.
pub trait Nameable<'a> {
    /// Type of iterator returned by [`names_iter`](self::Nameable::names_iter).
    type NamesIter: Iterator<Item = &'a Literal>;

    /// Type of iterator returned by [`short_names_iter`](self::Nameable::short_names_iter).
    type ShortNamesIter: Iterator<Item = &'a Literal>;

    /// Gets an iterator over the human-readable name literals for the entity. An entity may have
    /// multiple language-tagged name literals to provide multilingual naming. It also may have no
    /// name literals at all, in which case the returned iterator must be empty.
    fn names_iter(&'a self) -> Self::NamesIter;

    /// Gets an iterator over the short name literals for the entity. An entity may have multiple
    /// language-tagged short name literals to provide multilingual naming. It also may have no
    /// short name literals at all, in which case the returned iterator must be empty. Note that the
    /// set of languages supported by the short name literals doesn't have to correspond to the set
    /// of languages supported by the long name literals.
    ///
    /// Implementors must ensure that each returned literal's contents (excluding any language tag
    /// or data type information) does not exceed 16 Unicode grapheme clusters.
    fn short_names_iter(&'a self) -> Self::ShortNamesIter;
}

/// Trait for types that can specify a set of provided LV2 extension data interfaces.
pub trait ExtensionDataProvider {
    /// Type of iterator returned by
    /// [`extension_data_iter`](self::ExtensionDataProvider::extension_data_iter).
    type ExtensionDataIter: Iterator<Item = ExtensionData>;

    /// Gets an iterator over the types of extension data provided. The iterator must not repeat
    /// items.
    fn extension_data_iter(&self) -> Self::ExtensionDataIter;

    /// Checks if the specified type of extension data is provided. Should return true if and only
    /// if `extension_data` is in the set that would be returned by
    /// [`extension_data_iter`](self::ExtensionDataProvider::extension_data_iter). The default
    /// implementation calls
    /// [`extension_data_iter`](self::ExtensionDataProvider::extension_data_iter) and searches
    /// sequentially for `extension_data`; a more efficient implementation is likely to be possible
    /// for most implementing types.
    ///
    /// # Parameters
    /// - `extension_data`: The type of extension data to check for.
    fn has_extension_data(&self, extension_data: ExtensionData) -> bool {
        self.extension_data_iter().any(|ed| ed == extension_data)
    }
}

/// Trait for types that can specify supported and required LV2 host features.
pub trait HostFeatureSupporter {
    /// Type of iterator returned by
    /// [`required_host_features_iter`](self::HostFeatureSupporter::required_host_features_iter).
    type RequiredHostFeaturesIter: Iterator<Item = HostFeature>;

    /// Type of iterator returned by
    /// [`optional_host_features_iter`](self::HostFeatureSupporter::optional_host_features_iter).
    type OptionalHostFeaturesIter: Iterator<Item = HostFeature>;

    /// Gets an iterator over the required host features. The iterator must not repeat items.
    fn required_host_features_iter(&self) -> Self::RequiredHostFeaturesIter;

    /// Checks if the specified host feature is required. Should return true if and only if
    /// `host_feature` is in the set that would be returned by
    /// [`required_host_features_iter`](self::HostFeatureSupporter::required_host_features_iter).
    /// The default implementation calls
    /// [`required_host_features_iter`](self::HostFeatureSupporter::required_host_features_iter) and
    /// searches sequentially for `host_feature`; a more efficient implementation is likely to be
    /// possible for most implementing types.
    ///
    /// # Parameters
    /// - `host_feature`: The host feature to check for.
    fn requires_host_feature(&self, host_feature: HostFeature) -> bool {
        self.required_host_features_iter().any(|hf| hf == host_feature)
    }

    /// Gets an iterator over the host features that are supported but not required. The iterator
    /// must not repeat items, and must define a set that is disjoint from that defined by
    /// [`required_host_features_iter`](self::HostFeatureSupporter::required_host_features_iter).
    fn optional_host_features_iter(&self) -> Self::OptionalHostFeaturesIter;

    /// Checks if the specified host feature is supported. Should return true if and only if
    /// `host_feature` is in the set that would be returned by
    /// [`optional_host_features_iter`](self::HostFeatureSupporter::optional_host_features_iter).
    /// The default implementation calls
    /// [`optional_host_features_iter`](self::HostFeatureSupporter::optional_host_features_iter) and
    /// searches sequentially for `host_feature`; a more efficient implementation is likely to be
    /// possible for most implementing types.
    ///
    /// # Parameters
    /// - `host_feature`: The host feature to check for.
    fn optionally_supports_host_feature(&self, host_feature: HostFeature) -> bool {
        self.optional_host_features_iter().any(|hf| hf == host_feature)
    }

    /// Gets an iterator over the host features that are supported, as either required features or
    /// optional features. The iterator must not repeat items, and must define a set equal to the
    /// union of the sets defined by
    /// [`required_host_features_iter`](self::HostFeatureSupporter::required_host_features_iter) and
    /// [`optional_host_features_iter`](self::HostFeatureSupporter::optional_host_features_iter).
    fn supported_host_features_iter(&self) -> Chain<Self::RequiredHostFeaturesIter, Self::OptionalHostFeaturesIter> {
        self.required_host_features_iter().chain(self.optional_host_features_iter())
    }

    /// Checks if the specified host feature is supported, as either a required feature or an
    /// optional feature. Should return true if and only if `host_feature` is in the set that would
    /// be returned by
    /// [`supported_host_features_iter`](self::HostFeatureSupporter::supported_host_features_iter).
    ///
    /// # Parameters
    /// - `host_feature`: The host feature to check for.
    fn supports_host_feature(&self, host_feature: HostFeature) -> bool {
        self.requires_host_feature(host_feature)
            || self.optionally_supports_host_feature(host_feature)
    }
}