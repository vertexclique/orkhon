use lifeguard::{Recycleable};
use crate::reqrep::{OResponse, ORequest};
use std::fmt::{Debug, Formatter, Error};
use pyo3::{GILGuard, Python};

//requester: Box<dyn Process>

#[derive(Default)]
pub struct Model {
    gil: Option<GILGuard>,
}

impl Model {
    pub fn new() -> Self {
        Model { ..Default::default() }
    }
}

impl Recycleable for Model {
    fn new() -> Self {
        let mut model = Model::new();
        model.gil = Some(Python::acquire_gil());
        model
    }

    fn reset(&mut self) {}
}
