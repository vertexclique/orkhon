#![feature(async_await)]

use crate::reqrep::{ORequest, OResponse};
use crate::errors::*;
use std::future::Future;
use pyo3::PyObject;

pub(crate) trait AsyncService<R, T> where
    R: std::marker::Send,
    T: std::marker::Send {
    type FutType: Future<Output = Result<OResponse<T>>>;

    fn async_process(&mut self, request: ORequest<R>) -> Self::FutType;
}

pub(crate) trait PythonAsyncService<R>
    where
        R: std::marker::Send {
    type FutType: Future<Output = Result<OResponse<PyObject>>>;

    fn async_process(&mut self, request: ORequest<R>) -> Self::FutType;
}

pub(crate) trait Service {
    fn load(&mut self) -> Result<()>;
}
