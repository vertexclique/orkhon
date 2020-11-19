//!
//! Tensorflow model interface

use crate::errors::*;
use crate::reqrep::{ORequest, OResponse, TFRequest, TFResponse};
use crate::service::{Service, TensorflowAsyncService};
use std::path::PathBuf;

use tract_core::framework::*;
use tract_core::prelude::*;
use tract_tensorflow::prelude::*;

use crate::config::OrkhonConfig;

use async_trait::async_trait;
use tract_tensorflow::tract_hir::infer::InferenceOp;

#[derive(Default, Clone)]
pub struct TFModel {
    pub name: String,
    pub file: PathBuf,
    config: OrkhonConfig,
    model: TypedModel,
}

impl TFModel {
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

    pub(crate) fn process(&self, request: ORequest<TFRequest>) -> Result<OResponse<TFResponse>> {
        let plan = self.model.clone().into_runnable()?;

        plan.run(tvec!(request.body.input))
            .map_err(|e| {
                let err_msg: String = format!("Call failed: {:?}", e);
                OrkhonError::RequestError(err_msg.to_owned())
            })
            .map(|result| OResponse::with_body(TFResponse::new().with_output(result)))
    }

    fn auto_load_input_facts(&self, loaded_model: InferenceGraph) -> Result<InferenceGraph> {
        use crate::tensorflow::tensorflow::SessionOptions;
        use crate::tensorflow::tensorflow::{
            DataType, DEFAULT_SERVING_SIGNATURE_DEF_KEY, PREDICT_INPUTS,
        };

        let mut graph = tensorflow::Graph::new();
        let parent_dir = self.file.parent().map_or_else(
            || {
                Err(OrkhonError::General(
                    "Parent directory traversal failed".into(),
                ))
            },
            |e| Ok(e),
        )?;
        let parent_dir = std::fs::canonicalize(parent_dir)?;
        let saved_model = tensorflow::SavedModelBundle::load(
            &SessionOptions::default(),
            vec!["train", "serve", "serve_default"],
            &mut graph,
            parent_dir,
        )
        .map_err(|e| OrkhonError::TFModelBackendError(e.to_string()))?;
        let mgd = saved_model.meta_graph_def();
        let sig = mgd
            .get_signature(DEFAULT_SERVING_SIGNATURE_DEF_KEY)
            .map_err(|e| OrkhonError::TFModelBackendError(e.to_string()))?;
        let input_info = sig
            .get_input(PREDICT_INPUTS)
            .map_err(|e| OrkhonError::TFModelBackendError(e.to_string()))?;
        let input_op = graph
            .operation_by_name_required(&input_info.name().name)
            .map_err(|e| OrkhonError::TFModelBackendError(e.to_string()))?;
        let input_index = input_info.name().index;
        let input_type = match input_op.input_type(input_index as _) {
            DataType::UnrecognizedEnumValue(_) => {
                return Err(OrkhonError::General(
                    "Unrecognized dtype for the frozen model.".into(),
                ));
            }
            DataType::Float => DatumType::F32,
            DataType::Double => DatumType::F64,
            DataType::Int32 => DatumType::I32,
            DataType::UInt8 => DatumType::U8,
            DataType::Int16 => DatumType::I16,
            DataType::Int8 => DatumType::I8,
            DataType::String => DatumType::String,
            DataType::Complex64 => DatumType::TDim,
            DataType::Int64 => DatumType::I64,
            DataType::Bool => DatumType::Bool,
            DataType::QInt8 => DatumType::I8,
            DataType::QUInt8 => DatumType::U8,
            DataType::QInt32 => DatumType::I32,
            DataType::BFloat16 => DatumType::F32, // TODO: Fix it, right now upscaling with no serialization input.
            DataType::QInt16 => DatumType::I16,
            DataType::QUInt16 => DatumType::U16,
            DataType::UInt16 => DatumType::U16,
            DataType::Complex128 => DatumType::TDim,
            DataType::Half => DatumType::TDim,
            DataType::Resource => DatumType::TDim,
            DataType::Variant => DatumType::TDim,
            DataType::UInt32 => DatumType::U32,
            DataType::UInt64 => DatumType::U64,
        };
        let dim = input_op
            .get_attr_shape("shape")
            .map_err(|e| OrkhonError::TFModelBackendError(e.to_string()))?
            .dims()
            .ok_or_else(|| OrkhonError::General("Dimension fetching failed".into()))?;

        Ok(loaded_model.with_input_fact(
            input_index as usize,
            InferenceFact::dt_shape(input_type, tvec!(dim)),
        )?)
    }
}

type InferenceGraph = Graph<InferenceFact, Box<dyn InferenceOp>>;

impl Service for TFModel {
    fn load(&mut self) -> Result<()> {
        let unoptimized: InferenceGraph =
            tract_tensorflow::tensorflow().model_for_path(self.file.as_path())?;

        let input_loaded = if self.config.auto_load_input_facts {
            self.auto_load_input_facts(unoptimized)?
        } else {
            unoptimized.with_input_fact(
                0,
                self.config.default_input_fact_shape.to_owned().ok_or_else(|| {
                    OrkhonError::General(
                        "Inference shape should be given when no auto infer is in place.".into(),
                    )
                })?,
            )?
        };

        self.model = input_loaded.into_optimized()?;
        Ok(())
    }
}

#[async_trait]
impl TensorflowAsyncService for TFModel {
    async fn async_process(&self, request: ORequest<TFRequest>) -> Result<OResponse<TFResponse>> {
        self.process(request)
    }
}
