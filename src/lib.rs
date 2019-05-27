//! [![Orkhon](https://raw.githubusercontent.com/vertexclique/orkhon/master/doc/logo/orkhon.png)](https://github.com/vertexclique/orkhon)
//!
//!
//! # Orkhon: ML Inference Framework and Server Runtime
//!
//! ## What is it?
//! Orkhon is Rust framework for Machine Learning to run/use inference/prediction code written in Python, frozen models and process unseen data. It is mainly focused on serving models and processing unseen data in a performant manner. Instead of using Python directly and having scalability problems for servers this framework tries to solve them with built-in async API.
//!
//! ## Main features
//!
//! * Sync & Async API for models.
//! * Easily embeddable engine for well-known Rust web frameworks.
//! * API contract for interacting with Python code.
//! * High processing throughput
//!
//! ## Installation
//!
//! You can include Orkhon into your project with;
//!
//! ```toml
//! [dependencies]
//! orkhon = "0.1.0"
//! ```
//!
//! ## Dependencies
//! You will need:
//! * Rust Nightly needed (for now. until async support fully lands in)
//! * Python dev dependencies installed and have proper python runtime to use Orkhon with your project.
//! * Point out your `PYTHONHOME` environment variable to your Python installation.
//!
//! ## Python API contract
//!
//! Python API contract is hook based. If you want to call a method for prediction you should write
//! Python code with `args` and `**kwargs`.
//!
//! ```python
//! def model_hook(args, **kwargs):
//!     print("Doing prediction...")
//!     return args
//! ```
//!
//! #### Python Hook Input
//! Both args and kwargs are [`HashSet`]s. `args` can take any acceptable hashset key and passes down to python level.
//! But `kwargs` keys are restricted to [`&str`] for keeping it only for option passing.
//! `args` can contain your data for making prediction. Input contract is opinionated for making interpreter work without
//! unknown type conversions.
//!
//! #### Python Hook Output
//! Python hook output is passed up without downcasting or casting. Python bindings are still exposed to make sure you get the type you wanted.
//! By default; python passes [`PyObject`] to Rust interface. You can extract the type from the object that Python passed with
//! ```rust
//! pyobj.extract()?
//! ```
//! This api uses [PyO3 bindings] for Python <-> Rust. You can look for PyO3's documentation to make conversions.
//! Auto conversion methods soon will be added.
//!
//! [PyO3 bindings]: https://github.com/PyO3/pyo3

#![doc(html_logo_url = "https://raw.githubusercontent.com/vertexclique/orkhon/master/doc/logo/icon.png")]

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

