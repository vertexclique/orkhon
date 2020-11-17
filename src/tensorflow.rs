use crate::service::{Service, TensorflowAsyncService};
use crate::reqrep::{OResponse, ORequest, TFRequest, TFResponse};
use crate::errors::*;
use std::path::PathBuf;

use tract_core::framework::*;
use tract_core::prelude::*;
use tract_tensorflow::prelude::*;

use std::thread;

use futures::channel::oneshot;
use futures::prelude::future::FutureObj;


#[derive(Default, Clone)]
pub struct TFModel {
    pub name: String,
    pub file: PathBuf,
    model: TypedModel
}

impl TFModel {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn with_name<T>(mut self, name: T) -> Self
    where
        T: AsRef<str>
    {
        self.name = name.as_ref().into();
        self
    }

    pub fn with_model_file(mut self, model_file: PathBuf) -> Self {
        self.file = model_file;
        self
    }

    pub(crate) fn process(&mut self, request: ORequest<TFRequest>) -> Result<OResponse<TFResponse>> {
        let plan = self.model.clone().into_runnable()?;

        plan.run(tvec!(request.body.input)).map_err(|e| {
            let err_msg: String = format!("Call failed\n\
            \twith traceback {:?}", e);
            OrkhonError::PyModuleError(err_msg.to_owned())
        }).map(|result| {
            OResponse::with_body(
                TFResponse::new().with_output(result)
            )
        })
    }
}

impl Service for TFModel {
    fn load(&mut self) -> Result<()> {
        let unoptimized = tract_tensorflow::tensorflow()
            .model_for_path(self.file.as_path())?;

        unoptimized.into_optimized()?;
        Ok(())
    }
}

impl TensorflowAsyncService for TFModel where {
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
