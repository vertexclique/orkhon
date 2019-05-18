use lifeguard::*;
mod model;

use model::Model;
use crate::config::OrkhonConfig;

#[derive(Default)]
pub struct DispatchPool {
    pool: Option<Pool<Model>>
}

impl DispatchPool {
    pub fn new(config: &OrkhonConfig) -> Self {
        DispatchPool {
            pool: Some (
                pool()
                .with(StartingSize(config.pool_config.pool_size))
                .build()
            )
        }
    }
}

