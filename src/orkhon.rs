use crate::config::{OrkhonConfig};
use crate::dispatch::PooledModel;
use crate::service::Service;
use crate::tensorflow::TFModel;
use std::collections::HashMap;

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

    pub fn tensorflow(mut self, model: TFModel) -> Self {
        self.services.insert(model.name.to_owned(), Box::new(model));
        self
    }

    pub fn pymodel(mut self, model: PooledModel) -> Self {
        self.services.insert(model.name.to_owned(), Box::new(model));
        self
    }
}
