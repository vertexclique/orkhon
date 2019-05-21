use lifeguard::{Recycleable};
use crate::reqrep::{OResponse, ORequest};
use std::fmt::{Debug, Formatter, Error};

use pyo3::prelude::*;
use pyo3::types::*;

use crate::errors::*;
use std::path::PathBuf;

use log::*;

//requester: Box<dyn Process>

#[derive(Default)]
pub struct Model {
    // Pool Manager passes model parameters
    module_path: &'static str,
    module_name: &'static str,
    // Supplier will assign the model below.
    gil: Option<GILGuard>,
    module: Option<&'static PyModule>,
}

impl Model {
    pub fn new() -> Self {
        Model { ..Default::default() }
    }

    pub fn with_module(mut self, module_name: &'static str) -> Self {
        self.module_name = module_name;
        self
    }

    pub fn with_module_path(mut self, module_path: &'static str) -> Self {
        self.module_path = module_path;
        self
    }

    pub fn supplier(mut self) -> Self {
        let gilblock = Python::acquire_gil();
        let py = gilblock.python();

        let syspath: &PyList = py.import("sys")
            .unwrap()
            .get("path")
            .unwrap()
            .try_into()
            .unwrap();

        syspath.insert(0, self.module_path).unwrap();
        warn!("SYSPATH => \n{:?}", syspath);
        let datamod: &PyModule = py.import(self.module_name).unwrap();

        // Assign acquired utilities
//        self.gil = Some(gilblock);
        self.module = Some(datamod);

        self
    }
}

impl Recycleable for Model {
    fn new() -> Self {
        // Supplier provided in the top level.
        // Shouldn't hit this.
        error!("Invalid Recycleable instantiation");
        Model::new()
    }

    fn reset(&mut self) {}
}
