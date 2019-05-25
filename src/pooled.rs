use crate::config::OrkhonConfig;
use crate::service::{Service, AsyncService};
use crate::reqrep::{ORequest, OResponse, PyModelResponse, PyModelRequest};
use crate::errors::*;

use std::path::PathBuf;
use std::error::Error;

use pyo3::prelude::*;
use pyo3::types::*;
use log::*;

use std::thread;

use futures::channel::oneshot;
use std::future::Future;
use futures::prelude::future::FutureObj;

#[derive(Default, Clone)]
pub struct PooledModel {
    pub name: &'static str,
    pub module_path: PathBuf,
    pub module:  &'static str,
    pub requester_hook: &'static str,
    config: OrkhonConfig
}

impl PooledModel {
    pub fn new(config: OrkhonConfig) -> Self {
        PooledModel {
            config,
            ..Default::default()
        }
    }

    pub fn with_name(mut self, name: &'static str) -> Self {
        self.name = name;
        self
    }

    pub fn with_module_path(mut self, module_path: PathBuf) -> Self {
        self.module_path = module_path;
        self
    }

    pub fn with_module(mut self, module: &'static str) -> Self {
        self.module = module;
        self
    }

    pub fn with_requester_hook(mut self, requester_hook: &'static str) -> Self {
        self.requester_hook = requester_hook;
        self
    }
}

impl<R, T> Service<R, T> for PooledModel {
    fn load(&mut self) -> Result<()> {
        if !self.module_path.exists() {
            let mp = format!("Module path doesn't exist {}", self.module_path.to_str().unwrap());
            return Err(ErrorKind::OrkhonPyModuleError(mp).into())
        }

        Ok(())
    }

    fn process(&mut self, request: ORequest<R>) -> Result<OResponse<T>> {
        let gilblock = Python::acquire_gil();
        let py = gilblock.python();

        let syspath: &PyList = py.import("sys")
            .unwrap()
            .get("path")
            .unwrap()
            .try_into()
            .unwrap();

        let module_path =
            self.module_path.clone().into_os_string().into_string().unwrap();

        syspath.insert(0, module_path).unwrap();
        warn!("SYSPATH => \n{:?}", syspath);
        let datamod: &PyModule = py.import(self.module).unwrap();

        let args = PyTuple::new(py, &["123"]);
        let kwargs = None;
        datamod.call(self.requester_hook, args, kwargs).map_err::<ErrorKind, _>(|e| {
            let err_msg: String = format!("Call failed over {:?}\n\
            \twith args {:?}\n\twith kwargs {:?}", self.requester_hook, "", "");
            ErrorKind::OrkhonPyModuleError(err_msg.to_owned()).into()
        });

        Ok(OResponse::ForPyModel(PyModelResponse::new()))
    }
}

impl<R: 'static, T: 'static> AsyncService<R, T> for PooledModel where
    R: std::marker::Send,
    T: std::marker::Send {
    type FutType = FutureObj<'static, Result<OResponse<T>>>;

    fn async_process(&mut self, request: ORequest<R>) -> FutureObj<'static, Result<OResponse<T>>>
        where
            R: std::marker::Send,
            T: std::marker::Send {
        let mut klone = self.clone();
        FutureObj::new(Box::new(
            async move {
                let (sender, receiver) = oneshot::channel();

                let _ = thread::spawn(move || {
                    let resp = klone.process(request);

                    let _ = sender.send(resp);
                });

                receiver.await.unwrap()
            }
        ))
    }
}
