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
//! orkhon = "*"
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
//! ```ignore
//! pyobj.extract()?
//! ```
//! This api uses [PyO3 bindings] for Python <-> Rust. You can look for PyO3's documentation to make conversions.
//! Auto conversion methods soon will be added.
//!
//! ## Examples
//! #### Creating Orkhon
//!
//! ```
//! # #[macro_use] extern crate orkhon;
//! # use orkhon::orkhon::Orkhon;
//! # use orkhon::config::OrkhonConfig;
//! # use std::path::PathBuf;
//! Orkhon::new()
//!    .config(OrkhonConfig::new())
//!    .pymodel("model_which_will_be_tested", // Unique identifier of the model
//!             "tests/pymodels",             // Python module directory
//!             "model_test",                 // Python module file name
//!        "model_hook"                       // Hook(Python method) that will be called by Orkhon
//!    )
//!    .build();
//! ```
//!
//! #### Requesting to Orkhon
//!
//! ```
//! # #[macro_use] extern crate orkhon;
//! # use orkhon::orkhon::Orkhon;
//! # use orkhon::config::OrkhonConfig;
//! # use std::path::PathBuf;
//! # use std::collections::HashMap;
//! # use orkhon::reqrep::{ORequest, PyModelRequest};
//! #
//! # let o = Orkhon::new()
//! #    .config(OrkhonConfig::new())
//! #    .pymodel("model_which_will_be_tested", // Unique identifier of the model
//! #             "tests/pymodels",             // Python module directory
//! #             "model_test",                 // Python module file name
//! #        "model_hook"                       // Hook(Python method) that will be called by Orkhon
//! #    )
//! #    .build();
//! // Args for the request hook
//! let mut request_args = HashMap::new();
//! request_args.insert("is", 10);
//! request_args.insert("are", 6);
//! request_args.insert("you", 5);
//!
//! // Kwargs for the request hook
//! let mut request_kwargs = HashMap::<&str, &str>::new();
//!
//! // Future handle (await over it... if you want)
//! let handle =
//!     o.pymodel_request_async(
//!         "model_which_will_be_tested",
//!             ORequest::with_body(
//!                 PyModelRequest::new()
//!                     .with_args(request_args)
//!                     .with_kwargs(request_kwargs)
//!             )
//!     );
//!
//! ```
//!
//! ## License
//!
//! License is [MIT]
//!
//! ## Discussion and Development
//! We use [Gitter] for development discussions. Also please don't hesitate to open issues on GitHub ask for features, report bugs, comment on design and more!
//! More interaction and more ideas are better!
//!
//! ## Contributing to Orkhon [![Open Source Helpers](https://www.codetriage.com/vertexclique/orkhon/badges/users.svg)](https://www.codetriage.com/vertexclique/orkhon)
//!
//! All contributions, bug reports, bug fixes, documentation improvements, enhancements and ideas are welcome.
//!
//! A detailed overview on how to contribute can be found in the  [CONTRIBUTING guide] on GitHub.
//!
//!
//! [PyO3 bindings]: https://github.com/PyO3/pyo3
//! [`HashSet`]: https://doc.rust-lang.org/std/collections/struct.HashSet.html
//! [`PyObject`]: https://docs.rs/pyo3/0.7.0/pyo3/struct.PyObject.html
//! [MIT]: https://github.com/vertexclique/orkhon/blob/master/LICENSE
//! [CONTRIBUTING guide]: https://github.com/vertexclique/orkhon/blob/master/.github/CONTRIBUTING.md
//! [Gitter]: https://gitter.im/orkhonml/community

#![doc(html_logo_url = "https://raw.githubusercontent.com/vertexclique/orkhon/master/doc/logo/icon.png")]

pub mod config;
pub mod pooled;
pub mod reqrep;
pub mod service;
pub mod tensorflow;

#[macro_use]
mod service_macros;
pub mod errors;

pub mod orkhon;
