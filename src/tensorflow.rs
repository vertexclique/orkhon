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
pub struct TFModel {
    pub name: &'static str,
    pub file: PathBuf,
    model: Model<TensorFact>
}

impl TFModel {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn with_name(mut self, name: &'static str) -> Self {
        self.name = name;
        self
    }

    pub fn with_model_file(mut self, model_file: PathBuf) -> Self {
        self.file = model_file;
        self
    }

    pub(crate) fn process<R, T>(&mut self, request: ORequest<R>) -> Result<OResponse<T>> {
        unimplemented!()
    }
}

impl Service for TFModel {
    fn load(&mut self) -> Result<()> {
        self.model = tract_tensorflow::tensorflow().model_for_path(self.file.as_path())?;
        Ok(())
    }
}

impl<R: 'static, T: 'static> AsyncService<R, T> for TFModel where
    R: std::marker::Send,
    T: std::marker::Send {
    type FutType = FutureObj<'static, Result<OResponse<T>>>;

    fn async_process(&mut self, request: ORequest<R>) -> FutureObj<'static, Result<OResponse<T>>>
        where
            R: std::marker::Send,
            T: std::marker::Send {
        let mut klone = self.clone();
        FutureObj::new(Box::new(
            async move {
                let (sender, receiver) = oneshot::channel();
                let _ = thread::spawn(move || {
                    let resp = klone.process(request);

                    let _ = sender.send(resp);
                });

                receiver.await.unwrap()
            }
        ))
    }
}
