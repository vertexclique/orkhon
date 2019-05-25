#![feature(async_await)]

use crate::reqrep::{ORequest, OResponse};
use crate::errors::*;
use std::future::Future;

pub(crate) trait AsyncService<R, T> where
    R: std::marker::Send,
    T: std::marker::Send {
    type FutType: Future<Output = Result<OResponse<T>>>;

    fn async_process(&mut self, request: ORequest<R>) -> Self::FutType;
}

pub(crate) trait Service<R, T> {
    fn load(&mut self) -> Result<()>;
    fn process(&mut self, request: ORequest<R>) -> Result<OResponse<T>>;
}
