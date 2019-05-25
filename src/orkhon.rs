use std::collections::HashMap;

use crate::config::{OrkhonConfig};
use crate::pooled::PooledModel;
use crate::service::{Service, AsyncService, PythonAsyncService};
use crate::tensorflow::TFModel;
use crate::reqrep::{ORequest, OResponse, PyModelRequest, TFRequest, TFResponse};
use crate::service_macros::*;
use crate::errors::*;

use log::*;
use std::path::PathBuf;
use futures::prelude::future::FutureObj;
use std::any::Any;
use pyo3::{PyTypeInfo, ToPyObject, PyObject};
use pyo3::types::PyDict;
use std::{cmp, hash};

#[derive(Default)]
pub struct Orkhon {
    config: OrkhonConfig,
    py_services: HashMap<String, PooledModel>,
    tf_services: HashMap<String, TFModel>,
}

impl Orkhon {
    pub fn new() -> Self {
        Orkhon { ..Default::default() }
    }

    pub fn config(mut self, config: OrkhonConfig) -> Self {
        self.config = config;
        self
    }

    pub fn tensorflow(mut self, model_name: &'static str, model_file: PathBuf) -> Self {
        let model_spec = TFModel::new()
            .with_name(model_name)
            .with_model_file(model_file);

        self.tf_services.insert(model_name.to_owned(), model_spec);

        self
    }

    pub fn pymodel(mut self,
                   model_name: &'static str,
                   module_path: &'static str,
                   module: &'static str,
                   requester_hook: &'static str) -> Self {
        let model_spec = PooledModel::new(self.config)
            .with_name(model_name)
            .with_module_path(PathBuf::from(module_path))
            .with_module(module)
            .with_requester_hook(requester_hook);

        self.py_services.insert(model_name.to_owned(), model_spec);

        self
    }

    pub fn pymodel_request<K: 'static + Send, R: 'static + Send, T: 'static + Send>(
        mut self, model_name: &str,
        request: ORequest<PyModelRequest<K, R, T>>) -> Result<OResponse<PyObject>>
        where K: hash::Hash + cmp::Eq + Default + ToPyObject + Send,
              R: Default + ToPyObject + Send,
              T: Default + ToPyObject + Send {
        if let Some(modelbox) = self.py_services.get_mut(model_name) {
            modelbox.process::<K, R, T>(request)
        } else {
            Err(ErrorKind::OrkhonModelNotFoundError("Can't find model.".to_string()).into())
        }
    }

    pub fn tensorflow_request(
        mut self, model_name: &str,
        request: ORequest<TFRequest>) -> Result<OResponse<TFResponse>> {
        if let Some(modelbox) = self.tf_services.get_mut(model_name) {
            modelbox.process(request)
        } else {
            Err(ErrorKind::OrkhonModelNotFoundError("Can't find model.".to_string()).into())
        }
    }

    pub async fn pymodel_request_async<K: 'static + Send, R: 'static + Send, T: 'static + Send>(
        mut self, model_name: &str,
        request: ORequest<PyModelRequest<K, R, T>>) -> Result<OResponse<PyObject>>
        where K: hash::Hash + cmp::Eq + Default + ToPyObject + Send,
              R: Default + ToPyObject + Send,
              T: Default + ToPyObject + Send {
        if let Some(modelbox) = self.py_services.get_mut(model_name) {
            modelbox.async_process(request).await
        } else {
            Err(ErrorKind::OrkhonModelNotFoundError("Can't find model.".to_string()).into())
        }
    }

    pub async fn tensorflow_request_async(
        mut self, model_name: &str,
        request: ORequest<TFRequest>) -> Result<OResponse<TFResponse>> {
        if let Some(modelbox) = self.tf_services.get_mut(model_name) {
            modelbox.async_process(request).await
        } else {
            Err(ErrorKind::OrkhonModelNotFoundError("Can't find model.".to_string()).into())
        }
    }

    pub fn build(mut self) -> Self {
        warn!("Building model storage.");
        for (model_name, model_service) in &mut self.py_services {
            warn!("Loading Python model :: {}", model_name);
            model_service.load().unwrap();
        }

        for (model_name, model_service) in &mut self.tf_services {
            warn!("Loading Tensorflow model :: {}", model_name);
            model_service.load().unwrap();
        }

        self
    }
}
