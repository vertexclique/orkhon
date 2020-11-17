use rand::*;
use orkhon::prelude::*;
use nuclei::prelude::*;
use orkhon::tcore::prelude::*;
use std::path::PathBuf;
use orkhon::ttensor::prelude::*;

#[test]
fn test_async_request_for_tensorflow_model() {
    use orkhon::reqrep::*;

    let _ = env_logger::builder().is_test(true).try_init();

    let o = Orkhon::new()
        .config(
            OrkhonConfig::new().with_input_fact_shape(InferenceFact::dt_shape(
                f32::datum_type(),
                tvec![10, 100],
            ))
        )
        .tensorflow("model_which_will_be_tested",
              PathBuf::from("tests/protobuf/manual_input_infer/my_model.pb"),
        )
        .build();

    let mut rng = thread_rng();
    let vals: Vec<_> = (0..1000).map(|_| rng.gen::<f32>()).collect();
    let input = tract_ndarray::arr1(&vals).into_shape((10, 100)).unwrap();

    let handle =
        o.tensorflow_request_async(
            "model_which_will_be_tested",
            ORequest::with_body(
                TFRequest::new()
                    .body(input.into())
            )
        );

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
            OrkhonConfig::new().with_input_fact_shape(InferenceFact::dt_shape(
                f32::datum_type(),
                tvec![10, 100],
            ))
        )
        .onnx("model_which_will_be_tested",
              PathBuf::from("tests/protobuf/onnx_model/example.onnx"),
        )
        .build();

    let mut rng = thread_rng();
    let vals: Vec<_> = (0..1000).map(|_| rng.gen::<f32>()).collect();
    let input = tract_ndarray::arr1(&vals).into_shape((10, 100)).unwrap();

    let handle =
        o.onnx_request_async(
            "model_which_will_be_tested",
            ORequest::with_body(
                ONNXRequest::new()
                        .body(input.into())
            )
        );

    let resp = block_on(handle).unwrap();
    assert_eq!(resp.body.output.len(), 1);
}