use crate::config::OrkhonConfig;
use crate::dispatch::DispatchPool;

#[derive(Default)]
pub struct Orkhon {
    config: OrkhonConfig,
    dpool: DispatchPool
}

impl Orkhon {
    pub fn new() -> Self {
        Orkhon { ..Default::default() }
    }

    pub fn config(mut self, config: OrkhonConfig) -> Self {
        self.config = config;
        self.dpool = DispatchPool::new(&config);
        self
    }


}
