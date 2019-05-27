#[derive(Default, Clone, Copy)]
pub struct OrkhonConfig {
}


impl OrkhonConfig {
    pub fn new() -> Self {
        OrkhonConfig {..Default::default()}
    }
}
