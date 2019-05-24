//! Representation of LV2 symbols.

use regex::Regex;

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
        // TODO: Avoid compiling the regex every time new() is called. Might have to wait for more
        // compile-time function evaluation to make it into Rust.
        let symbol_regex = Regex::new("^[_a-zA-Z][_a-zA-Z0-9]*$").unwrap();

        if symbol_regex.is_match(&string) {
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