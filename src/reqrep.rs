//!
//! Request response structures for Orkhon

use smallvec::SmallVec;
use std::sync::Arc;

use tract_core::prelude::*;

pub enum Types {
    PyModel,
    TFModel,
}

/// Orkhon request container
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct ORequest<T> {
    pub body: T,
}

impl<T> ORequest<T> {
    /// Orkhon request container that takes backend specific response
    pub fn with_body(body: T) -> Self {
        ORequest { body }
    }
}

/// Orkhon response container
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct OResponse<T> {
    pub body: T,
}

impl<T> OResponse<T> {
    /// Orkhon response container that takes backend specific response
    pub fn with_body(body: T) -> Self {
        OResponse { body }
    }
}

pub(crate) trait ORequestBase<T> {}
pub(crate) trait OResponseBase<T> {}

impl<T> ORequestBase<T> for T {}
impl<T> OResponseBase<T> for T {}

cfg_if::cfg_if! {
    if #[cfg(feature = "pymodel")] {
        use pyo3::ToPyObject;

        #[derive(Default, Debug, Clone)]
        pub struct PyModelRequest<K, V, T>
        where
            K: hash::Hash + cmp::Eq + Default + ToPyObject,
            V: Default + ToPyObject,
            T: Default + ToPyObject,
        {
            pub args: HashMap<K, V>,
            pub kwargs: HashMap<&'static str, T>,
        }

        impl<K, V, T> PyModelRequest<K, V, T>
        where
            K: hash::Hash + cmp::Eq + Default + ToPyObject,
            V: Default + ToPyObject,
            T: Default + ToPyObject,
        {
            pub fn new() -> Self {
                PyModelRequest {
                    ..Default::default()
                }
            }

            pub fn with_args(mut self, args: HashMap<K, V>) -> Self {
                self.args = args;
                self
            }

            pub fn with_kwargs(mut self, kwargs: HashMap<&'static str, T>) -> Self {
                self.kwargs = kwargs;
                self
            }
        }

        #[derive(Default, Debug, Clone)]
        pub struct PyModelResponse {
           response: PyDict,
        }

        impl PyModelResponse {
           pub fn new() -> Self { PyModelResponse { ..Default::default() } }
        }
    } else if #[cfg(feature = "onnxmodel")] {
        /// ONNX request
        #[derive(Default, Debug)]
        pub struct ONNXRequest {
            pub input: Tensor,
        }

        impl ONNXRequest {
            /// Creates a new ONNX inference request
            pub fn new() -> Self {
                ONNXRequest {
                    ..Default::default()
                }
            }

            /// Append body to the ONNX request
            pub fn body(mut self, request: Tensor) -> Self {
                self.input = request;
                self
            }
        }

        /// ONNX response
        #[derive(Default, Debug)]
        pub struct ONNXResponse {
            pub output: SmallVec<[Arc<Tensor>; 4]>,
        }

        impl ONNXResponse {
            /// Creates a new ONNX inference response
            pub fn new() -> Self {
                ONNXResponse {
                    ..Default::default()
                }
            }

            /// Give output coming out from ONNX
            pub fn with_output(mut self, output: SmallVec<[Arc<Tensor>; 4]>) -> Self {
                self.output = output;
                self
            }
        }
    }
}

/// Tensorflow request
#[derive(Default, Debug)]
pub struct TFRequest {
    pub input: Tensor,
}

impl TFRequest {
    /// Creates a new tensorflow inference request
    pub fn new() -> Self {
        TFRequest {
            ..Default::default()
        }
    }

    /// Append body to the tensorflow request
    pub fn body(mut self, request: Tensor) -> Self {
        self.input = request;
        self
    }
}

/// Tensorflow response
#[derive(Default, Debug)]
pub struct TFResponse {
    pub output: SmallVec<[Arc<Tensor>; 4]>,
}

impl TFResponse {
    /// Creates a new tensorflow inference response
    pub fn new() -> Self {
        TFResponse {
            ..Default::default()
        }
    }

    /// Give output coming out from tensorflow
    pub fn with_output(mut self, output: SmallVec<[Arc<Tensor>; 4]>) -> Self {
        self.output = output;
        self
    }
}
