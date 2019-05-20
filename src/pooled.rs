use lifeguard::*;
mod model;

use model::Model;

use crate::config::OrkhonConfig;
use crate::service::Service;
use crate::reqrep::{ORequest, OResponse};
use crate::errors::*;

use std::path::PathBuf;
use std::error::Error;

#[derive(Default)]
pub struct PooledModel<'a> {
    pub name: &'a str,
    pub file: PathBuf,
    pub requester_hook: String,
    pool: Option<Pool<Model>>
}

impl<'a> PooledModel<'a> {
    pub fn new(config: OrkhonConfig) -> Self {
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

impl<'a> Service for PooledModel<'a> {
    fn load(&mut self) -> Result<()> {
        unimplemented!()
    }

    fn process(&mut self, request: ORequest) -> Result<OResponse> {
        unimplemented!()
    }
}
