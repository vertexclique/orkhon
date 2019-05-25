#[macro_export]
macro_rules! request_sync_for {
    ( $( $services:expr, $model_name:expr, $request:expr ),* ) => {
        {
            $(
            if let Some(modelbox) = $services.get_mut($model_name) {
                modelbox.process($request)
            } else {
                Err(ErrorKind::OrkhonModelNotFoundError("Can't find model.".to_string()).into())
            }
            )*
        }
    };
}

#[macro_export]
macro_rules! request_async_for {
    ( $( $r:ty, $t:ty, $services:expr, $model_name:expr, $request:expr ),* ) => {
        {
            $(
            if let Some(modelbox) = $services.get_mut($model_name) {
                modelbox.async_process($request).await
            } else {
                Err(ErrorKind::OrkhonModelNotFoundError("Can't find model.".to_string()).into())
            }
            )*
        }
    };
}
