use crate::config::{OrkhonConfig};
use crate::dispatch::PooledModel;
use crate::service::Service;
use crate::tensorflow::TFModel;

#[derive(Default)]
pub struct Orkhon {
    config: OrkhonConfig,
    services: Vec<Box<dyn Service>>,
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
        self.services.push(Box::new(model));
        self
    }

    pub fn pymodel(mut self, model: PooledModel) -> Self {
        self.services.push(Box::new(model));
        self
    }
}
