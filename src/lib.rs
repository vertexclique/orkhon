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
//! * If you use `pymodel` feature, Python dev dependencies should be installed and have proper python runtime to use Orkhon with your project.
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
//! ```
//!
//! #### Requesting to Orkhon
//!
//! ```
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

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/vertexclique/orkhon/master/doc/logo/icon.png"
)]

cfg_if::cfg_if! {
    if #[cfg(feature = "pymodel")] {
        pub mod pooled;
    } else if #[cfg(feature = "onnxmodel")] {
        pub mod onnx;
    }
}

pub mod tensorflow;
pub mod config;
pub mod reqrep;
pub mod service;
pub mod errors;

pub mod orkhon;

pub use tract_core as tcore;
pub use tract_tensorflow as ttensor;

pub mod prelude {
    pub use super::config::*;
    pub use super::reqrep::*;

    pub use super::tensorflow::*;
    pub use super::tcore::*;
    pub use super::ttensor::*;

    cfg_if::cfg_if! {
        if #[cfg(feature = "pymodel")] {
            pub use super::pooled::*;
        } else if #[cfg(feature = "onnxmodel")] {
            pub use super::onnx::*;
        }
    }

    pub use super::orkhon::*;
}
