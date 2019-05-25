use tract_core::internal::HashMap;
use std::{hash, cmp};
use pyo3::{ToPyObject, PyTypeInfo};
use pyo3::types::PyDict;

pub enum Types {
    PyModel,
    TFModel
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct ORequest<T>
{
    pub body: T
}

impl<T> ORequest<T> {
    pub fn with_body(body: T) -> Self {
        ORequest { body }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct OResponse<T>
{
    pub body: T
}

impl<T> OResponse<T> {
    pub fn with_body(body: T) -> Self {
        OResponse { body }
    }
}

pub(crate) trait ORequestBase<T> {}
pub(crate) trait OResponseBase<T> {}

impl<T> ORequestBase<T> for T {}
impl<T> OResponseBase<T> for T {}

#[derive(Default, Debug, Clone)]
pub struct PyModelRequest<K, V, T>
    where K: hash::Hash + cmp::Eq + Default + ToPyObject,
          V: Default + ToPyObject,
          T: Default + ToPyObject {
    pub args: HashMap<K, V>,
    pub kwargs: HashMap<&'static str, T>
}

impl<K, V, T> PyModelRequest<K, V, T>
    where K: hash::Hash + cmp::Eq + Default + ToPyObject,
          V: Default + ToPyObject,
          T: Default + ToPyObject {
    pub fn new() -> Self {
        PyModelRequest { ..Default::default() }
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

//#[derive(Default, Debug, Clone)]
//pub struct PyModelResponse {
//    response: PyDict,
//}
//
//impl PyModelResponse {
//    pub fn new() -> Self { PyModelResponse { ..Default::default() } }
//}

#[derive(Default, Debug, PartialEq, PartialOrd)]
pub struct TFRequest {}

impl TFRequest {
    pub fn new() ->  Self { TFRequest { .. Default::default()} }
}

#[derive(Default, Debug, PartialEq, PartialOrd)]
pub struct TFResponse {}

impl TFResponse {
    pub fn new() ->  Self { TFResponse  { .. Default::default()} }
}
