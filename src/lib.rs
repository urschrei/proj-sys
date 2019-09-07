#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[cfg(not(feature = "docs-rs"))]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// on docs.rs, switch to the "backup" bindings
#[cfg(feature = "docs-rs")]
include!("bindings.rs");
