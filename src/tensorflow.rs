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
