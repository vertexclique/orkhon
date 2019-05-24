#![feature(async_await)]

use crate::reqrep::{ORequest, OResponse};
use crate::errors::*;
use std::future::Future;

pub(crate) trait AsyncService {
    type FutType: Future<Output = Result<T>>;

    fn async_process<R>(&mut self, request: R) -> Self::FutType;
}

pub(crate) trait PythonAsyncService<T>: AsyncService {

}

pub(crate) trait Service<T> {
    fn load(&mut self) -> Result<()>;
    fn process<R>(&mut self, request: R) -> Result<T>;
}
