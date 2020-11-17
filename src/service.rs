use crate::errors::*;
use crate::reqrep::{ORequest, OResponse, TFRequest, TFResponse, ONNXRequest, ONNXResponse};
use async_trait::async_trait;

pub(crate) trait Service {
    fn load(&mut self) -> Result<()>;
}

#[async_trait]
pub(crate) trait TensorflowAsyncService {
    async fn async_process(
        &mut self,
        request: ORequest<TFRequest>,
    ) -> Result<OResponse<TFResponse>>;
}

#[async_trait]
pub(crate) trait ONNXAsyncService {
    async fn async_process(
        &mut self,
        request: ORequest<ONNXRequest>,
    ) -> Result<OResponse<ONNXResponse>>;
}


cfg_if::cfg_if! {
    if #[cfg(feature = "pymodel")] {
        use pyo3::{PyObject, ToPyObject};
        use crate::reqrep::PyModelRequest;

        #[async_trait]
        pub(crate) trait PythonAsyncService {
            async fn async_process<K: 'static, V: 'static, T: 'static>(
                &mut self,
                request: ORequest<PyModelRequest<K, V, T>>,
            ) -> Self::FutType
            where
                K: hash::Hash + cmp::Eq + Default + ToPyObject + Send,
                V: Default + ToPyObject + Send,
                T: Default + ToPyObject + Send;
        }
    }
}
