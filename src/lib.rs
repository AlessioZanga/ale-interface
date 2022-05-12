// Suppress warning due to "C" naming conventions.
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// Include bindings.
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
