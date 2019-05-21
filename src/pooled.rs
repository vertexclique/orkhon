use lifeguard::*;
mod model;

use crate::pooled::model::Model;

use crate::config::OrkhonConfig;
use crate::service::Service;
use crate::reqrep::{ORequest, OResponse};
use crate::errors::*;

use std::path::PathBuf;
use std::error::Error;

use pyo3::types::*;


#[derive(Default)]
pub struct PooledModel {
    pub name: &'static str,
    pub pool_module_path: PathBuf,
    pub pool_module:  &'static str,
    pub requester_hook: String,
    config: OrkhonConfig,
    pool: Option<Pool<Model>>
}

impl PooledModel {
    pub fn new(config: OrkhonConfig) -> Self {
        PooledModel {
            config,
            ..Default::default()
        }
    }

    pub fn with_name(mut self, name: &'static str) -> Self {
        self.name = name;
        self
    }

    pub fn with_module_path(mut self, module_path: PathBuf) -> Self {
        self.pool_module_path = module_path;
        self
    }

    pub fn with_module(mut self, module: &'static str) -> Self {
        self.pool_module = module;
        self
    }
}

//    pub fn dispense_gil(mut self) -> Result<Recycled<'static, Model>> {
//        match self.pool {
//            Some(modelpool) => Ok(modelpool.new()),
//            _ => Err(ErrorKind::OrkhonAcquireGILError("Model Pool isn't initialized.".to_string()).into())
//        }
//    }

impl Service for PooledModel {
    fn load(&mut self) -> Result<()> {
        if !self.pool_module_path.exists() {
            return Err(ErrorKind::OrkhonPyModuleError("Module path doesn't exist".to_owned()).into())
        }

        let module_path =
            self.pool_module_path.clone().into_os_string().into_string().unwrap();

        let pool_instance = pool()
            .with(StartingSize(self.config.pool_config.pool_size))
            .with(Supplier(||
                Model::new()
//                    .with_module(self.pool_module.clone())
//                    .with_module_path(module_path.as_str().clone())
                    .supplier()
            ))
          .build();

        self.pool = Some(pool_instance);
//        let k = PyTuple::new(py, &["123"]);
//        datam.call("data", k, None).unwrap();
        Ok(())
    }

    fn process(&mut self, request: ORequest) -> Result<OResponse> {
        unimplemented!()
    }
}
