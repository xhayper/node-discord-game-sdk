#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(clippy::disallowed_names)]
#![allow(clippy::uninlined_format_args)]

#[macro_use]
extern crate napi_derive;

#[cfg(feature = "snmalloc")]
#[global_allocator]
static ALLOC: snmalloc_rs::SnMalloc = snmalloc_rs::SnMalloc;

mod client;
mod user;
