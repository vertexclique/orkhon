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
//! orkhon = "0.2"
//! ```
//!
//! ## Dependencies
//! You will need:
//! * If you use `pymodel` feature, Python dev dependencies should be installed and have proper python runtime to use Orkhon with your project.
//! * If you want to have tensorflow inference. Installing tensorflow as library for linking is required.
//! * ONNX interface doesn't need extra dependencies from the system side.
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
//! #### Request a Tensorflow prediction asynchronously
//!
//! ```no_run
//! # use nuclei::prelude::*;
//! use orkhon::prelude::*;
//! use orkhon::tcore::prelude::*;
//! use orkhon::ttensor::prelude::*;
//! use rand::*;
//! use std::path::PathBuf;
//!
//!let o = Orkhon::new()
//!    .config(
//!        OrkhonConfig::new()
//!            .with_input_fact_shape(InferenceFact::dt_shape(f32::datum_type(), tvec![10, 100])),
//!    )
//!    .tensorflow(
//!        "model_which_will_be_tested",
//!        PathBuf::from("tests/protobuf/manual_input_infer/my_model.pb"),
//!    )
//!    .shareable();
//!
//!let mut rng = thread_rng();
//!let vals: Vec<_> = (0..1000).map(|_| rng.gen::<f32>()).collect();
//!let input = tract_ndarray::arr1(&vals).into_shape((10, 100)).unwrap();
//!
//!let o = o.get();
//!let handle = async move {
//!    let processor = o.tensorflow_request_async(
//!       "model_which_will_be_tested",
//!       ORequest::with_body(TFRequest::new().body(input.into())),
//!    );
//!    processor.await
//!};
//!let resp = block_on(handle).unwrap();
//! ```
//!
//! #### Request an ONNX prediction synchronously
//!
//! This example needs `onnxmodel` feature enabled.
//!
//! ```ignore
//! use orkhon::prelude::*;
//! use orkhon::tcore::prelude::*;
//! use orkhon::ttensor::prelude::*;
//! use rand::*;
//! use std::path::PathBuf;
//!
//!     let o = Orkhon::new()
//!         .config(
//!             OrkhonConfig::new()
//!                 .with_input_fact_shape(InferenceFact::dt_shape(f32::datum_type(), tvec![10, 100])),
//!         )
//!         .onnx(
//!             "model_which_will_be_tested",
//!             PathBuf::from("tests/protobuf/onnx_model/example.onnx"),
//!         )
//!         .build();
//!
//!     let mut rng = thread_rng();
//!     let vals: Vec<_> = (0..1000).map(|_| rng.gen::<f32>()).collect();
//!     let input = tract_ndarray::arr1(&vals).into_shape((10, 100)).unwrap();
//!
//!     let resp = o
//!         .onnx_request(
//!             "model_which_will_be_tested",
//!             ORequest::with_body(ONNXRequest::new().body(input.into())),
//!         )
//!         .unwrap();
//!     assert_eq!(resp.body.output.len(), 1);
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

pub mod config;
pub mod errors;
pub mod reqrep;
pub mod service;
pub mod tensorflow;

pub mod orkhon;

pub use tract_core as tcore;
pub use tract_tensorflow as ttensor;

pub mod prelude {
    pub use super::config::*;
    pub use super::reqrep::*;

    pub use super::tcore::*;
    pub use super::tensorflow::*;
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
