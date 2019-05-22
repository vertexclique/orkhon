use std::collections::HashMap;

use crate::config::{OrkhonConfig};
use crate::pooled::PooledModel;
use crate::service::Service;
use crate::tensorflow::TFModel;
use crate::api::OrkhonAPI;
use crate::reqrep::{ORequest, OResponse};
use crate::errors::*;

use log::*;
use std::path::PathBuf;

#[derive(Default)]
pub struct Orkhon {
    config: OrkhonConfig,
    services: HashMap<String, Box<dyn Service>>,
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
        self.services.insert(model_name.to_owned(), Box::new(
            TFModel::new()
                .with_name(model_name)
                .with_model_file(model_file)
        ));
        self
    }

    pub fn pymodel(mut self, model_name: &'static str, module_path: &'static str, module: &'static str) -> Self {
        self.services.insert(model_name.to_owned(), Box::new(
            PooledModel::new(self.config)
                .with_name(model_name)
                .with_module_path(PathBuf::from(module))
        ));
        self
    }

    pub fn request(mut self, model_name: &str, request: ORequest) -> Result<OResponse> {
        if let Some(modelbox) = self.services.get_mut(model_name) {
            modelbox.process(request)
        } else {
            Err(ErrorKind::OrkhonRequestError("Can't return a response.".to_string()).into())
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
