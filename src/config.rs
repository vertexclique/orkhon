use tract_tensorflow::tract_hir::infer::InferenceFact;

#[derive(Default, Clone)]
pub struct OrkhonConfig {
    pub auto_load_input_facts: bool,
    pub input_facts_shape: Option<InferenceFact>,
}


impl OrkhonConfig {
    pub fn new() -> Self {
        OrkhonConfig {
            ..Self::default()
        }
    }

    pub fn with_auto_load_input_facts(mut self) -> Self {
        self.auto_load_input_facts = true;
        self
    }

    pub fn with_input_fact_shape(mut self, inference_shape: InferenceFact) -> Self {
        self.input_facts_shape = Option::from(inference_shape);
        self
    }
}
