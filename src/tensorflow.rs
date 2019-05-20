use crate::service::Service;
use crate::reqrep::{OResponse, ORequest};
use crate::errors::*;
use std::path::PathBuf;

use tract_core::ndarray;
use tract_core::framework::*;
use tract_core::prelude::*;

#[derive(Default)]
pub struct TFModel<'a> {
    pub name: &'a str,
    pub file: PathBuf,
    model: Model<TensorFact>
}

impl<'a> TFModel<'a> {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn with_name(mut self, name: &'a str) -> Self {
        self.name = name;
        self
    }

    pub fn with_model_file(mut self, model_file: PathBuf) -> Self {
        self.file = model_file;
        self
    }
}

impl<'a> Service for TFModel<'a> {
    fn load(&mut self) -> Result<()> {
        self.model = tract_tensorflow::tensorflow().model_for_path(self.file.as_path())?;
        Ok(())
    }

    fn process(&mut self, request: ORequest) -> Result<OResponse> {
        unimplemented!()
    }
}
