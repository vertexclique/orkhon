//! [![Orkhon](https://raw.githubusercontent.com/vertexclique/orkhon/master/doc/logo/orkhon.png)](https://github.com/vertexclique/orkhon)

#![feature(async_await)]
#[macro_use]
extern crate error_chain;

pub mod config;
pub mod pooled;
pub mod reqrep;
pub mod service;
pub mod tensorflow;

#[macro_use]
mod service_macros;
pub mod errors;

pub mod orkhon;

