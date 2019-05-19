use lifeguard::*;
mod model;

use model::Model;
use crate::config::OrkhonConfig;
use crate::service::Service;
use crate::reqrep::{ORequest, OResponse};
use std::path::PathBuf;

#[derive(Default)]
pub struct PooledModel {
    file: PathBuf,
    requester_hook: String,
    pool: Option<Pool<Model>>
}

impl PooledModel {
    pub fn new(config: &OrkhonConfig) -> Self {
        PooledModel {
            pool: Some (
                pool()
                .with(StartingSize(config.pool_config.pool_size))
                .build()
            ),
            ..Default::default()
        }
    }
}

impl Service for PooledModel {
    fn process(&mut self, request: ORequest) -> OResponse {
        unimplemented!()
    }
}
