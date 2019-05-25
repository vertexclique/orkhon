use crate::service::{Service, AsyncService};
use crate::reqrep::{OResponse, ORequest, TFRequest, TFResponse};
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

    pub(crate) fn process(&mut self, request: ORequest<TFRequest>) -> Result<OResponse<TFResponse>> {
        let model = self.model.clone().into_optimized()?;
        let plan = SimplePlan::new(&model)?;

        plan.run(tvec!(request.body.input)).map_err(|e| {
            let err_msg: String = format!("Call failed\n\
            \twith traceback {:?}", e);
            ErrorKind::OrkhonPyModuleError(err_msg.to_owned()).into()
        }).map(|result| {
            OResponse::with_body(
                TFResponse::new().with_output(result)
            )
        })
    }
}

impl Service for TFModel {
    fn load(&mut self) -> Result<()> {
        self.model = tract_tensorflow::tensorflow().model_for_path(self.file.as_path())?;
        Ok(())
    }
}

impl AsyncService for TFModel where {
    type FutType = FutureObj<'static, Result<OResponse<TFResponse>>>;

    fn async_process(&mut self, request: ORequest<TFRequest>)
        -> FutureObj<'static, Result<OResponse<TFResponse>>> {
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
