//! A library for reading RDF data from an [LV2](http://lv2plug.in) bundle into a data structure
//! that can be more easily processed by Rust code.
//!
//! Note: This library does not attempt to be completely comprehensive, i.e., it does not model all
//! the possible forms of data that may appear in an LV2 bundle. In particular, non-standard
//! extensions may not be well-supported. This is mostly due to the extremely extensible nature of
//! LV2, which makes it difficult to model all possible extensions in a data structure while keeping
//! that data structure easy to process. This library *does* attempt to cover all commonly used,
//! non-deprecated features of LV2, including the [standard extensions](http://lv2plug.in/ns/).

pub mod bundle_model;