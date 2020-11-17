use orkhon::prelude::*;
use orkhon::tcore::prelude::*;
use orkhon::ttensor::prelude::*;
use rand::*;
use std::path::PathBuf;

#[cfg(feature = "onnxmodel")]
fn main() {
    let _ = env_logger::builder().is_test(true).try_init();

    let o = Orkhon::new()
        .config(
            OrkhonConfig::new()
                .with_input_fact_shape(InferenceFact::dt_shape(f32::datum_type(), tvec![10, 100])),
        )
        .onnx(
            "onnx_model",
            PathBuf::from("tests/protobuf/onnx_model/example.onnx"),
        )
        .build();

    let mut rng = thread_rng();
    let vals: Vec<_> = (0..1000).map(|_| rng.gen::<f32>()).collect();
    let input = tract_ndarray::arr1(&vals).into_shape((10, 100)).unwrap();

    println!(
        "\n==============\nRequesting inference with tensor to ONNX backend:\n==============\n\n{}",
        input
    );

    let handle = o.onnx_request(
        "onnx_model",
        ORequest::with_body(ONNXRequest::new().body(input.into())),
    );

    let resp = handle.unwrap();
    println!(
        "\n==============\nInference output:\n==============\n\n{:?}",
        resp.body.output
    );
}

#[cfg(not(feature = "onnxmodel"))]
fn main() {}
