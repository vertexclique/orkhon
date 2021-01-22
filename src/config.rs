//!
//! Orkhon configuration structure


cfg_if::cfg_if! {
    if #[cfg(feature = "onnxmodel")] {
        use tract_onnx::tract_hir::infer::InferenceFact as OnnxInferenceFact;
    } else if #[cfg(feature = "tfmodel")] {
        use tract_tensorflow::tract_hir::infer::InferenceFact as TFInferenceFact;
    }
}


#[derive(Default, Clone)]
pub struct OrkhonConfig {
    pub auto_load_input_facts: bool,
    #[cfg(feature = "tfmodel")]
    pub default_tf_input_fact_shape: Option<TFInferenceFact>,
    #[cfg(feature = "onnxmodel")]
    pub default_onnx_input_fact_shape: Option<OnnxInferenceFact>,
}

impl OrkhonConfig {
    pub fn new() -> Self {
        OrkhonConfig { ..Self::default() }
    }

    pub fn with_auto_load_input_facts(mut self) -> Self {
        self.auto_load_input_facts = true;
        self
    }

    cfg_if::cfg_if! {
        if #[cfg(feature = "onnxmodel")] {
            pub fn with_default_onnx_input_fact_shape(mut self, inference_shape: OnnxInferenceFact) -> Self {
                self.default_onnx_input_fact_shape = Option::from(inference_shape);
                self
            }
        } else if #[cfg(feature = "tfmodel")] {
            pub fn with_default_tf_input_fact_shape(mut self, inference_shape: TFInferenceFact) -> Self {
                self.default_tf_input_fact_shape = Option::from(inference_shape);
                self
            }
        }
    }
}
