use crate::errors::*;
use crate::reqrep::{ORequest, OResponse, ONNXRequest, ONNXResponse};
use crate::service::{Service, ONNXAsyncService};
use std::path::PathBuf;

use tract_core::framework::*;
use tract_core::prelude::*;
use tract_onnx::prelude::*;

use crate::config::OrkhonConfig;

use async_trait::async_trait;
use tract_tensorflow::tract_hir::infer::InferenceOp;

#[derive(Default, Clone)]
pub struct ONNXModel {
    pub name: String,
    pub file: PathBuf,
    config: OrkhonConfig,
    model: TypedModel,
}

impl ONNXModel {
    pub fn new(config: OrkhonConfig) -> Self {
        Self {
            config,
            ..Default::default()
        }
    }

    pub fn with_name<T>(mut self, name: T) -> Self
        where
            T: AsRef<str>,
    {
        self.name = name.as_ref().into();
        self
    }

    pub fn with_model_file(mut self, model_file: PathBuf) -> Self {
        self.file = model_file;
        self
    }

    pub(crate) fn process(&self, request: ORequest<ONNXRequest>) -> Result<OResponse<ONNXResponse>> {
        let plan = self.model.clone().into_runnable()?;

        plan.run(tvec!(request.body.input))
            .map_err(|e| OrkhonError::General(e.to_string()))
            .map(|result| OResponse::with_body(ONNXResponse::new().with_output(result)))
    }
}

type InferenceGraph = Graph<InferenceFact, Box<dyn InferenceOp>>;

impl Service for ONNXModel {
    fn load(&mut self) -> Result<()> {
        let unoptimized: InferenceGraph =
            tract_onnx::onnx().model_for_path(self.file.as_path())?;

        let input_loaded =
            unoptimized.with_input_fact(
                0,
                self.config.input_facts_shape.to_owned().ok_or_else(|| {
                    OrkhonError::General(
                        "Inference shape should be given when no auto infer is in place.".into(),
                    )
                })?,
            )?;

        self.model = input_loaded.into_optimized()?;
        Ok(())
    }
}

#[async_trait]
impl ONNXAsyncService for ONNXModel {
    async fn async_process(
        &self,
        request: ORequest<ONNXRequest>,
    ) -> Result<OResponse<ONNXResponse>> {
        self.process(request)
    }
}
