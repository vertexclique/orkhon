
#[derive(Default, Clone, Copy)]
pub struct PoolConfig {
    pub(crate) pool_size: usize
}

#[derive(Default, Clone, Copy)]
pub struct OrkhonConfig {
    pub(crate) pool_config: PoolConfig,
}


impl OrkhonConfig {
    pub fn new() -> Self {
        OrkhonConfig {..Default::default()}
    }
}
