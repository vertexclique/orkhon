use lifeguard::*;
mod model;

use model::Model;

use crate::config::OrkhonConfig;
use crate::service::Service;
use crate::reqrep::{ORequest, OResponse};
use crate::errors::*;

use std::path::PathBuf;
use std::error::Error;
use pyo3::Python;


#[derive(Default)]
pub struct PooledModel<'a> {
    pub name: &'a str,
    pub module_path: PathBuf,
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

//    pub fn dispense_gil(mut self) -> Result<Recycled<'static, Model>> {
//        match self.pool {
//            Some(modelpool) => Ok(modelpool.new()),
//            _ => Err(ErrorKind::OrkhonAcquireGILError("Model Pool isn't initialized.".to_string()).into())
//        }
//    }
}

impl<'a> Service for PooledModel<'a> {
    fn load(&mut self) -> Result<()> {

        Ok(())
    }

    fn process(&mut self, request: ORequest) -> Result<OResponse> {
        unimplemented!()
    }
}
