use std::collections::HashMap;

use crate::config::OrkhonConfig;
use crate::errors::*;
use crate::reqrep::{ORequest, OResponse, TFRequest, TFResponse};
use crate::service::{Service, TensorflowAsyncService};
use crate::tensorflow::TFModel;

use log::*;

use std::path::PathBuf;

cfg_if::cfg_if! {
    if #[cfg(feature = "pymodel")] {
        use crate::pooled::PooledModel;
        use crate::service::PythonAsyncService;
    } else if #[cfg(feature = "onnxmodel")] {
        use crate::onnx::ONNXModel;
        use crate::service::ONNXAsyncService;
        use crate::reqrep::{ONNXRequest, ONNXResponse};
    }
}


#[derive(Default, Clone)]
pub struct Orkhon {
    config: OrkhonConfig,
    tf_services: HashMap<String, TFModel>,
    #[cfg(feature = "pymodel")]
    py_services: HashMap<String, PooledModel>,
    #[cfg(feature = "onnxmodel")]
    onnx_services: HashMap<String, ONNXModel>,
}

impl Orkhon {
    pub fn new() -> Self {
        Orkhon {
            ..Default::default()
        }
    }

    pub fn config(mut self, config: OrkhonConfig) -> Self {
        self.config = config;
        self
    }

    pub fn tensorflow(mut self, model_name: &'static str, model_file: PathBuf) -> Self {
        let model_spec = TFModel::new(self.config.clone())
            .with_name(model_name)
            .with_model_file(model_file);

        self.tf_services.insert(model_name.to_owned(), model_spec);

        self
    }

    pub fn tensorflow_request(
        mut self,
        model_name: &str,
        request: ORequest<TFRequest>,
    ) -> Result<OResponse<TFResponse>> {
        request_sync_for!(self.tf_services, model_name, request)
    }

    pub async fn tensorflow_request_async(
        mut self,
        model_name: &str,
        request: ORequest<TFRequest>,
    ) -> Result<OResponse<TFResponse>> {
        request_async_for!(self.tf_services, model_name, request)
    }

    pub fn build(mut self) -> Self {
        warn!("Building model storage.");
        cfg_if::cfg_if! {
            if #[cfg(feature = "pymodel")] {
                for (model_name, model_service) in &mut self.py_services {
                    warn!("Loading Python model :: {}", model_name);
                    model_service.load().unwrap();
                }
            } else if #[cfg(feature = "onnxmodel")] {
                for (model_name, model_service) in &mut self.onnx_services {
                    warn!("Loading ONNX model :: {}", model_name);
                    model_service.load().unwrap();
                }
            }
        }

        for (model_name, model_service) in &mut self.tf_services {
            warn!("Loading Tensorflow model :: {}", model_name);
            model_service.load().unwrap();
        }

        self
    }

    cfg_if::cfg_if! {
        if #[cfg(feature = "pymodel")] {
            pub fn pymodel(mut self,
                           model_name: &'static str,
                           module_path: &'static str,
                           module: &'static str,
                           requester_hook: &'static str) -> Self {
                let model_spec = PooledModel::new(self.config)
                    .with_name(model_name)
                    .with_module_path(PathBuf::from(module_path))
                    .with_module(module)
                    .with_requester_hook(requester_hook);

                self.py_services.insert(model_name.to_owned(), model_spec);

                self
            }

            pub fn pymodel_request<K: 'static + Send, R: 'static + Send, T: 'static + Send>(
                mut self, model_name: &str,
                request: ORequest<PyModelRequest<K, R, T>>) -> Result<OResponse<PyObject>>
                where K: hash::Hash + cmp::Eq + Default + ToPyObject + Send,
                      R: Default + ToPyObject + Send,
                      T: Default + ToPyObject + Send {
                if let Some(modelbox) = self.py_services.get_mut(model_name) {
                    modelbox.process::<K, R, T>(request)
                } else {
                    Err(OrkhonError::ModelNotFoundError("Can't find model.".to_string()))
                }
            }

            pub async fn pymodel_request_async<K: 'static + Send, R: 'static + Send, T: 'static + Send>(
                mut self, model_name: &str,
                request: ORequest<PyModelRequest<K, R, T>>) -> Result<OResponse<PyObject>>
                where K: hash::Hash + cmp::Eq + Default + ToPyObject + Send,
                      R: Default + ToPyObject + Send,
                      T: Default + ToPyObject + Send {
                request_async_for!(self.py_services, model_name, request)
            }
        } else if #[cfg(feature = "onnxmodel")] {
            pub fn onnx<T>(mut self, model_name: T, model_file: PathBuf) -> Self
            where
                T: AsRef<str>
            {
                let model_spec = ONNXModel::new(self.config.clone())
                    .with_name(model_name.as_ref().to_owned())
                    .with_model_file(model_file);

                self.onnx_services.insert(model_name.as_ref().to_owned(), model_spec);

                self
            }

            pub fn onnx_request(
                mut self,
                model_name: &str,
                request: ORequest<ONNXRequest>,
            ) -> Result<OResponse<ONNXResponse>> {
                request_sync_for!(self.onnx_services, model_name, request)
            }

            pub async fn onnx_request_async(
                mut self,
                model_name: &str,
                request: ORequest<ONNXRequest>,
            ) -> Result<OResponse<ONNXResponse>> {
                request_async_for!(self.onnx_services, model_name, request)
            }
        }
    }
}
