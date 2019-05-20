use std::collections::HashMap;

use crate::config::{OrkhonConfig};
use crate::pooled::PooledModel;
use crate::service::Service;
use crate::tensorflow::TFModel;
use crate::api::OrkhonAPI;
use crate::reqrep::{ORequest, OResponse};
use crate::errors::*;
use core::borrow::BorrowMut;

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

    pub fn tensorflow(mut self, model: TFModel<'static>) -> Self {
        self.services.insert(model.name.to_owned(), Box::new(model));
        self
    }

    pub fn pymodel(mut self, model: PooledModel<'static>) -> Self {
        self.services.insert(model.name.to_owned(), Box::new(model));
        self
    }
}

impl OrkhonAPI for Orkhon {
    fn request(&mut self, model_name: &str, request: ORequest) -> Result<OResponse> {
        if let Some(modelbox) = self.services.get_mut(model_name) {
            modelbox.process(request)
        } else {
            Err(ErrorKind::OrkhonRequestError("Can't return a response.".to_string()).into())
        }
    }

    fn build(mut self) -> Self {
        for model_service in self.services.values_mut() {
            model_service.load();
        }

        self
    }
}
