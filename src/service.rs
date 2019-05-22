#![feature(async_await)]

use crate::reqrep::{ORequest, OResponse};
use crate::errors::*;
use std::future::Future;

pub(crate) trait AsyncService: Service {
    type FutType: Future<Output = Result<OResponse>>;

    fn async_process(&mut self, request: ORequest) -> Self::FutType;
}

pub(crate) trait Service {
    fn load(&mut self) -> Result<()>;
    fn process(&mut self, request: ORequest) -> Result<OResponse>;
}
