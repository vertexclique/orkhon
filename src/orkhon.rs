use std::collections::HashMap;

use crate::config::{OrkhonConfig};
use crate::pooled::PooledModel;
use crate::service::{Service, AsyncService};
use crate::tensorflow::TFModel;
use crate::api::OrkhonAPI;
use crate::reqrep::{ORequest, OResponse};
use crate::errors::*;

use log::*;
use std::path::PathBuf;
use futures::prelude::future::FutureObj;

#[derive(Default)]
pub struct Orkhon {
    config: OrkhonConfig,
    services: HashMap<String, Box<dyn Service + Send>>,
    async_services: HashMap<String, Box<dyn AsyncService<FutType=FutureObj<'static, Result<OResponse>>> + Send>>,
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

        {
            let model_spec = model_spec.clone();
            self.services.insert(model_name.to_owned(), Box::new(model_spec));
        }

        self.async_services.insert(model_name.to_owned(), Box::new(model_spec));

        self
    }

    pub fn pymodel(mut self, model_name: &'static str, module_path: &'static str, module: &'static str) -> Self {
        let model_spec = PooledModel::new(self.config)
            .with_name(model_name)
            .with_module_path(PathBuf::from(module_path));

        {
            let model_spec = model_spec.clone();
            self.services.insert(model_name.to_owned(), Box::new(model_spec));
        }

        self.async_services.insert(model_name.to_owned(), Box::new(model_spec));

        self
    }

    pub fn request<R, T>(mut self, model_name: &str, request: R) -> Result<T> {
        if let Some(modelbox) = self.services.get_mut(model_name) {
            modelbox.process(request)
        } else {
            Err(ErrorKind::OrkhonModelNotFoundError("Can't find model.".to_string()).into())
        }
    }

    pub async fn request_async<R, T>(mut self, model_name: &str, request: R) -> Result<T> {
        if let Some(modelbox) = self.async_services.get_mut(model_name) {
            modelbox.async_process(request).await
        } else {
            Err(ErrorKind::OrkhonModelNotFoundError("Can't find model.".to_string()).into())
        }
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
