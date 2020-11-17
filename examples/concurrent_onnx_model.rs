use futures_util::future::join_all;
use nuclei::prelude::*;
use orkhon::prelude::*;
use orkhon::tcore::ndarray::ArrayBase;
use orkhon::tcore::prelude::*;
use orkhon::ttensor::prelude::*;
use rand::*;
use rayon::prelude::*;
use std::path::PathBuf;
use nuclei::join_handle::JoinHandle;

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
        .shareable();

    let mut rng = thread_rng();

    let requests = (0..1_000)
        .map(|_| {
            let vals: Vec<_> = (0..1000).map(|_| rng.gen::<f32>()).collect();
            (
                tract_ndarray::arr1(&vals).into_shape((10, 100)).unwrap(),
                o.get(),
            )
        })
        .collect::<Vec<(ArrayBase<_, _>, Arc<Orkhon>)>>();

    let responses: Vec<JoinHandle<_>> = requests
        .into_par_iter()
        .map(|(input, o)| {
            let processor = async move {
                o.onnx_request_async(
                    "onnx_model",
                    ORequest::with_body(ONNXRequest::new().body(input.into())),
                )
                .await
            };
            spawn(processor)
        })
        .collect();

    let data = block_on(join_all(responses));
    data.iter().enumerate().for_each(|(i, r)| {
        println!(
            "\n\nRequest number {} =>\n{:?}",
            i,
            r.as_ref().unwrap().body.output
        );
    })
}

#[cfg(not(feature = "onnxmodel"))]
fn main() {}
