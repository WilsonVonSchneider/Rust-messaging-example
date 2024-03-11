#![allow(proc_macro_derive_resolution_fallback)]
#![recursion_limit = "256"]

#[allow(unused_imports)]
#[macro_use]
pub extern crate diesel;

pub mod schema;
pub mod db;
pub mod state;