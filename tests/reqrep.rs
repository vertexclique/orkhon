use nuclei::prelude::*;
use orkhon::prelude::*;
use orkhon::tcore::prelude::*;
use orkhon::ttensor::prelude::*;
use rand::*;
use std::path::PathBuf;

#[test]
#[cfg(feature = "tfmodel")]
fn test_sync_request_for_tensorflow_model() {
    use orkhon::reqrep::*;

    let _ = env_logger::builder().is_test(true).try_init();

    let o = Orkhon::new()
        .config(
            OrkhonConfig::new()
                .with_default_tf_input_fact_shape(InferenceFact::dt_shape(f32::datum_type(), tvec![10, 100])),
        )
        .tensorflow(
            "model_which_will_be_tested",
            PathBuf::from("tests/protobuf/manual_input_infer/my_model.pb"),
        )
        .build();

    let mut rng = thread_rng();
    let vals: Vec<_> = (0..1000).map(|_| rng.gen::<f32>()).collect();
    let input = tract_ndarray::arr1(&vals).into_shape((10, 100)).unwrap();

    let resp = o
        .tensorflow_request(
            "model_which_will_be_tested",
            ORequest::with_body(TFRequest::new().body(input.into())),
        )
        .unwrap();
    dbg!(&resp);
    assert_eq!(resp.body.output.len(), 1);
}

#[test]
#[cfg(feature = "onnxmodel")]
fn test_sync_request_for_onnx_model() {
    use orkhon::reqrep::*;

    let _ = env_logger::builder().is_test(true).try_init();

    let o = Orkhon::new()
        .config(
            OrkhonConfig::new()
                .with_default_onnx_input_fact_shape(InferenceFact::dt_shape(f32::datum_type(), tvec![10, 100])),
        )
        .onnx(
            "model_which_will_be_tested",
            PathBuf::from("tests/protobuf/onnx_model/example.onnx"),
        )
        .build();

    let mut rng = thread_rng();
    let vals: Vec<_> = (0..1000).map(|_| rng.gen::<f32>()).collect();
    let input = tract_ndarray::arr1(&vals).into_shape((10, 100)).unwrap();

    let resp = o
        .onnx_request(
            "model_which_will_be_tested",
            ORequest::with_body(ONNXRequest::new().body(input.into())),
        )
        .unwrap();
    assert_eq!(resp.body.output.len(), 1);
}

#[test]
#[cfg(feature = "tfmodel")]
fn test_async_request_for_tensorflow_model() {
    use orkhon::reqrep::*;

    let _ = env_logger::builder().is_test(true).try_init();

    let o = Orkhon::new()
        .config(
            OrkhonConfig::new()
                .with_default_tf_input_fact_shape(InferenceFact::dt_shape(f32::datum_type(), tvec![10, 100])),
        )
        .tensorflow(
            "model_which_will_be_tested",
            PathBuf::from("tests/protobuf/manual_input_infer/my_model.pb"),
        )
        .shareable();

    let mut rng = thread_rng();
    let vals: Vec<_> = (0..1000).map(|_| rng.gen::<f32>()).collect();
    let input = tract_ndarray::arr1(&vals).into_shape((10, 100)).unwrap();

    let o = o.get();
    let handle = async move {
        let processor = o.tensorflow_request_async(
            "model_which_will_be_tested",
            ORequest::with_body(TFRequest::new().body(input.into())),
        );
        processor.await
    };

    let resp = block_on(handle).unwrap();
    dbg!(&resp);
    assert_eq!(resp.body.output.len(), 1);
}

#[test]
#[cfg(feature = "onnxmodel")]
fn test_async_request_for_onnx_model() {
    use orkhon::reqrep::*;

    let _ = env_logger::builder().is_test(true).try_init();

    let o = Orkhon::new()
        .config(
            OrkhonConfig::new()
                .with_default_onnx_input_fact_shape(InferenceFact::dt_shape(f32::datum_type(), tvec![10, 100])),
        )
        .onnx(
            "model_which_will_be_tested",
            PathBuf::from("tests/protobuf/onnx_model/example.onnx"),
        )
        .shareable();

    let mut rng = thread_rng();
    let vals: Vec<_> = (0..1000).map(|_| rng.gen::<f32>()).collect();
    let input = tract_ndarray::arr1(&vals).into_shape((10, 100)).unwrap();

    let o = o.get();
    let handle = async move {
        let processor = o.onnx_request_async(
            "model_which_will_be_tested",
            ORequest::with_body(ONNXRequest::new().body(input.into())),
        );
        processor.await
    };

    let resp = block_on(handle).unwrap();
    assert_eq!(resp.body.output.len(), 1);
}
