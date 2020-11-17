use crate::errors::*;

#[macro_export]
macro_rules! request_sync_for {
    ( $( $services:expr, $model_name:expr, $request:expr ),* ) => {
        {
            $(
            if let Some(modelbox) = $services.get_mut($model_name) {
                modelbox.process($request)
            } else {
                Err(OrkhonError::ModelNotFound("Can't find model.".to_string()))
            }
            )*
        }
    };
}

#[macro_export]
macro_rules! request_async_for {
    ( $( $services:expr, $model_name:expr, $request:expr ),* ) => {
        {
            $(
            if let Some(modelbox) = $services.get_mut($model_name) {
                modelbox.async_process($request).await
            } else {
                Err(OrkhonError::ModelNotFound("Can't find model.".to_string()))
            }
            )*
        }
    };
}
