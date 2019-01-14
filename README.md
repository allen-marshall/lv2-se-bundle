# lv2-se-bundle

[![Build Status](https://travis-ci.org/allen-marshall/lv2-se-bundle.svg?branch=master)](https://travis-ci.org/allen-marshall/lv2-se-bundle)

A library for reading/writing [LV2](http://lv2plug.in) bundle RDF data to/from data structures that can be more easily
processed by Rust code. The 'se' stands for *safety* and *ease of use*, which are two of this project's main priorities.

**Note:** This library does not attempt to be completely comprehensive, i.e., it does not model all the possible forms
of data that may appear in an LV2 bundle. In particular, non-standard extensions may not be well-supported. This is
partly due to the extremely extensible nature of LV2, which makes it difficult to model all possible cases while
maintaining ease of use. This library *does* attempt to cover all commonly used, non-deprecated features of standard
LV2, including features from the [standard extensions](http://lv2plug.in/ns/). If you need features beyond what this
library provides, you may need to read or write the RDF bundle data directly (though feel free to request a change to
this library).