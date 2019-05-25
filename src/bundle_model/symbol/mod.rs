//! Representation of LV2 symbols.

use regex::Regex;

lazy_static! {
    /// Regex defining the allowed syntax for an LV2 symbol. A string is a valid LV2 symbol if and
    /// only if it contains a match of this regex. (The regex uses ^$ to force a full match of the
    /// string.)
    static ref SYMBOL_REGEX: Regex = Regex::new("^[_a-zA-Z][_a-zA-Z0-9]*$").unwrap();
}

/// Type of error generated when constructing a symbol from an invalid string.
type SymbolError = ();

/// Represents an LV2 symbol. In LV2, a symbol is a machine-readable and human-readable string that
/// identifies an entity, such as a port or plugin.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Symbol {
    /// The raw symbol string.
    string: String
}

impl Symbol {
    /// Constructs a new LV2 symbol.
    ///
    /// # Parameters
    /// - `string`: The raw symbol literal value.
    ///
    /// # Errors
    /// Returns an error if `string` is not a valid LV2 symbol string.
    pub fn new(string: String) -> Result<Symbol, SymbolError> {
        if SYMBOL_REGEX.is_match(&string) {
            Ok(Symbol {
                string
            })
        }
        else {
            Err(())
        }
    }

    /// Gets the raw symbol string. The returned string will be a valid LV2 symbol string.
    pub fn string(&self) -> &str {
        &self.string
    }
}