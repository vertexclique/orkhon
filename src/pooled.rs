use crate::config::OrkhonConfig;
use crate::service::{Service, PythonAsyncService};
use crate::reqrep::{ORequest, OResponse, PyModelRequest};
use crate::errors::*;

use std::path::PathBuf;

use pyo3::prelude::*;
use pyo3::types::*;
use log::*;

use std::{thread, cmp, hash, fs};
use std::any::Any;

use futures::channel::oneshot;
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

    pub(crate) fn process<K:'static, V: 'static, T:'static>(
        &mut self, request: ORequest<PyModelRequest<K, V, T>>) -> Result<OResponse<PyObject>>
        where
            K: hash::Hash + cmp::Eq + Default + ToPyObject + Send,
            V: Default + ToPyObject + Send,
            T: Default + ToPyObject + Send {
        let mut module_path = self.module_path.clone();
        let syspath_module_path =
            self.module_path.clone().into_os_string().into_string().unwrap();
        let module = format!("{}.py", self.module);

        module_path.push(module);

        let module_file =
            module_path.clone().into_os_string().into_string().unwrap();

        let source = fs::read_to_string(module_file.as_str()).unwrap();

        let gilblock = Python::acquire_gil();
        let py = gilblock.python();

        let syspath: &PyList = py.import("sys")
            .unwrap()
            .get("path")
            .unwrap()
            .try_into()
            .unwrap();

        let syspath_entry = syspath.get_item(0).downcast_ref::<PyString>().unwrap().to_string_lossy();

        if syspath_entry.as_ref() != syspath_module_path {
            syspath.insert(0, syspath_module_path).unwrap();
        }

        let datamod = PyModule::from_code(py, source.as_str(), self.name, self.name)
            .map_err(|e| {
                e.print(py);
                let err_msg: String = format!("Import failed in {}\n\
                \twith traceback", self.requester_hook);
                OrkhonError::PyModuleError(err_msg.to_owned())
            }).unwrap();
        warn!("SYS PATH => \n{:?}", syspath);

        let args_data = request.body.args.into_py_dict(py);
        let args = PyTuple::new(py, &[args_data]);

        let kwargs = request.body.kwargs.into_py_dict(py);

        datamod.call(self.requester_hook, args, Some(kwargs)).map_err(|e| {
            e.print(py);
            let err_msg: String = format!("Call failed over {:?}\n\
            \twith traceback", self.requester_hook);
            OrkhonError::PyModuleError(err_msg.to_owned())
        })
        .map(|resp| {
            OResponse::<PyObject> {
                body: resp.to_object(py)
            }
        })
    }
}

impl Service for PooledModel {
    fn load(&mut self) -> Result<()> {
        if !self.module_path.exists() {
            let mp = format!("Module path doesn't exist {}", self.module_path.to_str().unwrap());
            return Err(OrkhonError::OrkhonPyModuleError(mp))
        }

        Ok(())
    }
}

impl PythonAsyncService for PooledModel {
    type FutType = FutureObj<'static, Result<OResponse<PyObject>>>;

    fn async_process<K: 'static, V: 'static, T: 'static>(
        &mut self, request: ORequest<PyModelRequest<K, V, T>>)
        -> FutureObj<'static, Result<OResponse<PyObject>>>
        where
            K: hash::Hash + cmp::Eq + Default + ToPyObject + Send,
            V: Default + ToPyObject + Send,
            T: Default + ToPyObject + Send {
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
