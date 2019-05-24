use std::collections::HashMap;
use pyo3::prelude::*;
use pyo3::types::*;
use std::hash::Hash;
use std::marker::PhantomData;
use std::{cmp, hash};
use std::cmp::Ordering;

//#[derive(Default, Debug, PartialEq, PartialOrd)]
//pub struct ORequest
//{
//    py_request: PyModelRequest,
//    tf_request: TFRequest
//}
//
//impl ORequest {
//    pub fn new() -> Self {
//        ORequest {
//            ..Default::default()
//        }
//    }
//}

//impl<'a, T: ?Sized> ToPyObject for PhantomData<T>
//    where T: ToPyObject {
//    fn to_object(&self, py: Python) -> PyObject {
//        IntoPyDict::into_py_dict("", py).into()
//    }
//}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum OResponse<K, V>
{
    ForPyModel(PyModelResponse<K, V>),
    ForTFModel(TFResponse),
}

pub(crate) trait ORequestBase<T> {}
pub(crate) trait OResponseBase<T> {}

impl<T> ORequestBase<T> for T {}
impl<T> OResponseBase<T> for T {}

#[derive(Default, Debug, PartialEq)]
pub struct PyModelRequest<K, V, T>
    where K: hash::Hash + cmp::Eq + Default + ToPyObject,
          V: Default + ToPyObject,
          T: Default + ToPyObject {
    args: HashMap<K, V>,
    kwargs: HashMap<&'static str, T>
}

impl<K, V, T> PyModelRequest<K, V, T>
    where K: hash::Hash + cmp::Eq + Default + ToPyObject,
          V: Default + ToPyObject,
          T: Default + ToPyObject {
    pub fn new() -> Self {
        PyModelRequest::<K, V, T> { ..Default::default() }
    }
}

impl<K, V, T> PartialOrd for PyModelRequest<K, V, T> {
    fn partial_cmp(&self, other: &PyModelRequest<K,V,T>) -> Option<Ordering> {
        let lhs = self.args.capacity() + self.kwargs.capacity();
        let rhs = other.args.capacity() + other.kwargs.capacity();
        Some(lhs.cmp(&rhs))
    }
}

#[derive(Default, Debug, PartialEq, PartialOrd)]
pub struct PyModelResponse<K, V>
    where K: hash::Hash + cmp::Eq + ToPyObject,
          V: ToPyObject {
    response: HashMap<K, V>
}

impl<K, V> PyModelResponse<K, V>
    where K: hash::Hash + cmp::Eq + ToPyObject,
          V: ToPyObject {
    pub fn new() -> Self { PyModelResponse::<K, V> { ..Default::default() } }
}

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
