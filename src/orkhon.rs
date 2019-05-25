use std::collections::HashMap;

use crate::config::{OrkhonConfig};
use crate::pooled::PooledModel;
use crate::service::{Service, AsyncService};
use crate::tensorflow::TFModel;
use crate::reqrep::{ORequest, OResponse, PyModelRequest};
use crate::service_macros::*;
use crate::errors::*;

use log::*;
use std::path::PathBuf;
use futures::prelude::future::FutureObj;
use std::any::Any;

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

    pub fn pymodel(mut self, model_name: &'static str, module_path: &'static str, module: &'static str) -> Self {
        let model_spec = PooledModel::new(self.config)
            .with_name(model_name)
            .with_module_path(PathBuf::from(module_path));

        self.py_services.insert(model_name.to_owned(), model_spec);

        self
    }

    pub fn request<K, R, T>(
        mut self, model_name: &str,
        request: ORequest<R>) -> Result<OResponse<T>> where K: Service<R, T> {
        request_sync_for!(self.py_services, model_name, request);
        request_sync_for!(self.tf_services, model_name, request)
    }

    pub async fn request_async<K, R: Send, T: Send>(
        mut self, model_name: &str,
        request: ORequest<R>) -> Result<OResponse<T>> where K: AsyncService<R, T> {
        request_async_for!(self.py_services, model_name, request);
        request_async_for!(self.tf_services, model_name, request)
    }

    pub fn build(mut self) -> Self {
        warn!("Building model storage.");
        for (model_name, model_service) in &mut self.services {
            warn!("Loading model :: {}", model_name);
            model_service.load().unwrap();
        }

        self
    }
}
