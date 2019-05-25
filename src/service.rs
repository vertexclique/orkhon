#![feature(async_await)]

use crate::reqrep::{ORequest, OResponse, PyModelRequest};
use crate::errors::*;
use std::future::Future;
use pyo3::{PyObject, ToPyObject};
use std::{hash, cmp};

pub(crate) trait AsyncService<R, T> where
    R: std::marker::Send,
    T: std::marker::Send {
    type FutType: Future<Output = Result<OResponse<T>>>;

    fn async_process(&mut self, request: ORequest<R>) -> Self::FutType;
}

pub(crate) trait PythonAsyncService {
    type FutType: Future<Output = Result<OResponse<PyObject>>>;

    fn async_process<K: 'static, V: 'static, T: 'static>
    (&mut self, request: ORequest<PyModelRequest<K,V,T>>) -> Self::FutType
        where
            K: hash::Hash + cmp::Eq + Default + ToPyObject + Send,
            V: Default + ToPyObject + Send,
            T: Default + ToPyObject + Send;
}

pub(crate) trait Service {
    fn load(&mut self) -> Result<()>;
}
