#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

#[cfg(test)]
extern crate lazy_static;

include!(concat!(env!("OUT_DIR"), "/ffi.rs"));

pub use crate::ffi::*;
