use crate::service::{Service, AsyncService};
use crate::reqrep::{OResponse, ORequest};
use crate::errors::*;
use std::path::PathBuf;

use tract_core::ndarray;
use tract_core::framework::*;
use tract_core::prelude::*;

use log::*;
use std::thread;

use futures::channel::oneshot;
use std::future::Future;
use futures::prelude::future::FutureObj;

#[derive(Default, Clone)]
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

impl<'a> AsyncService for TFModel<'a> {
    type FutType = FutureObj<'static, Result<OResponse>>;

    fn async_process(&mut self, request: ORequest) -> FutureObj<'static, Result<OResponse>> {
        FutureObj::new(Box::new(
            async move {
                // Do async things
                // You might get a lifetime issue here if trying to access auth,
                // since it's borrowed.
                let (sender, receiver) = oneshot::channel();
                let _ = thread::spawn(move || {
                    let _ = sender.send(
                        Ok(OResponse::new())
                    );
                });

                receiver.await.unwrap()
            }
        ))
    }
}
