//! Utilities related to RDF.

use std::str::FromStr;
use std::fmt;
use std::fmt::{Display, Formatter};

use language_tags;

use sophia::term::iri_rfc3987::is_valid_iri;

/// Error type returned when trying to parse an invalid IRI.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InvalidIriError {
    /// The invalid IRI string that we tried to parse.
    pub attempted_iri: String
}

/// Error type returned when trying to parse an invalid language tag.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InvalidLangTagError {
    /// The invalid tag string that we tried to parse.
    pub attempted_tag: String
}

/// Represents an IRI.
///
/// Note: This type's implementations of [`Ord`](std::cmp::Ord) and
/// [`PartialOrd`](std::cmp::PartialOrd) have little semantic meaning, and exist mainly for use with
/// collections that require an ordered element type. Additionally, the implementations of
/// [`Eq`](std::cmp::Eq) and [`PartialEq`](std::cmp::PartialEq) will always treat two IRIs as
/// different if their text is different.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Iri {
    /// Constructor(s) should guarantee this is a valid IRI.
    iri: String
}

impl Iri {
    /// Constructs an IRI from the given string. Returns an error if the string is not a valid IRI.
    pub fn new(iri: String) -> Result<Self, InvalidIriError> {
        if !is_valid_iri(&iri) {
            Err(InvalidIriError { attempted_iri: iri })
        }
        else {
            Ok(Iri { iri })
        }
    }
}

impl Display for Iri {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str(&self.iri)
    }
}

/// Represents a language tag for an RDF literal.
///
/// Note: This type's implementations of [`Ord`](std::cmp::Ord) and
/// [`PartialOrd`](std::cmp::PartialOrd) have little semantic meaning, and exist mainly for use with
/// collections that require an ordered element type.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LangTag {
    /// Constructor(s) should guarantee this is a valid, canonicalized language tag string. All
    /// semantically equivalent language tags should have the same string.
    tag: String
}

impl LangTag {
    /// Constructs a language tag from the given string. Returns an error if the string is not a
    /// valid language tag.
    pub fn new(tag: String) -> Result<Self, InvalidLangTagError> {
        // We use a language_tags::LanguageTag object to perform the canonicalization and error
        // checking.
        match language_tags::LanguageTag::from_str(&tag) {
            Ok(tag_obj) => Ok(LangTag { tag: tag_obj.canonicalize().to_string() }),
            Err(_) => Err(InvalidLangTagError { attempted_tag: tag })
        }
    }
}

impl Display for LangTag {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str(&self.tag)
    }
}

// TODO: Figure out if lang_tagged_literal_data_type and non_lang_tagged_literal_default_data_type
// can be replaced with static constants. This would probably help with the efficiency of
// Literal::with_data_type. The main difficulty is that their initialization requires heap
// allocation.

/// Generates the data type IRI that automatically applies to all language-tagged RDF literals.
pub fn lang_tagged_literal_data_type() -> Iri {
    Iri::new("http://www.w3.org/1999/02/22-rdf-syntax-ns#langString".to_string()).unwrap()
}

/// Generates the default data type IRI for non-language-tagged RDF literals.
pub fn non_lang_tagged_literal_default_data_type() -> Iri {
    Iri::new("http://www.w3.org/2001/XMLSchema#string".to_string()).unwrap()
}

/// Represents an RDF literal.
///
/// Note: This type's implementations of [`Ord`](std::cmp::Ord) and
/// [`PartialOrd`](std::cmp::PartialOrd) have little semantic meaning, and exist mainly for use with
/// collections that require an ordered element type. Additionally, the implementations of
/// [`Eq`](std::cmp::Eq) and [`PartialEq`](std::cmp::PartialEq) do not take any data-type-specific
/// equivalence into account. For example, literals with value "0" and "0.0" are considered
/// different even if they have a floating point data type.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Literal {
    /// Literal value text.
    value: String,

    /// IRI identifying the data type. Must be the same IRI as returned by
    /// [`lang_tagged_literal_data_type`](self::lang_tagged_literal_data_type) if and only if a
    /// language tag is present.
    data_type: Iri,

    /// Optional language tag.
    lang_tag: Option<LangTag>
}

impl Literal {
    /// Constructs a literal with the given text value, no language tag, and the default data type
    /// for non-language-tagged literals.
    pub fn new(value: String) -> Self {
        Literal {
            value,
            data_type: non_lang_tagged_literal_default_data_type(),
            lang_tag: None
        }
    }

    /// Constructs a literal with the given text value and language tag. The data type will be the
    /// type required for language-tagged literals.
    pub fn with_lang_tag(value: String, lang_tag: LangTag) -> Self {
        Literal {
            value,
            data_type: lang_tagged_literal_data_type(),
            lang_tag: Some(lang_tag)
        }
    }

    /// Constructs a literal with the given text value, no language tag, and the given data type.
    /// Returns [`None`](std::option::Option::None) if the provided data type requires a language
    /// tag.
    pub fn with_data_type(value: String, data_type: Iri) -> Option<Self> {
        if data_type == lang_tagged_literal_data_type() {
            None
        }
        else {
            Some(Literal {
                value,
                data_type,
                lang_tag: None
            })
        }
    }
}