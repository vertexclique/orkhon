use crate::service::Service;
use crate::reqrep::{OResponse, ORequest};
use std::path::PathBuf;


#[derive(Default)]
pub struct TFModel {
    pub name: String,
    pub file: PathBuf,
}

impl TFModel {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

impl Service for TFModel {
    fn process(&mut self, request: ORequest) -> OResponse {
        unimplemented!()
    }
}
