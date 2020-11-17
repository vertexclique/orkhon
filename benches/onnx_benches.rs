#[macro_use]
extern crate criterion;
use criterion::{Benchmark, Criterion};

use criterion::*;

extern crate orkhon;
use futures_util::future::join_all;
use nuclei::join_handle::JoinHandle;
use nuclei::prelude::*;
use orkhon::prelude::*;
use orkhon::tcore::ndarray::ArrayBase;
use orkhon::tcore::prelude::*;
use orkhon::ttensor::prelude::*;
use rand::thread_rng;
use rand::*;
use rayon::prelude::*;
use std::path::PathBuf;

fn bench_onnx(c: &mut Criterion) {
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

    const OUTPUT_SIZE: usize = 10 * 100;
    const RUN_SIZE: usize = 3_000;

    let requests = (0..RUN_SIZE)
        .map(|_| {
            let vals: Vec<_> = (0..1_000).map(|_| rng.gen::<f32>()).collect();
            (
                tract_ndarray::arr1(&vals).into_shape((10, 100)).unwrap(),
                o.get(),
            )
        })
        .collect::<Vec<(ArrayBase<_, _>, Arc<Orkhon>)>>();

    c.bench(
        "bench_onnx",
        Benchmark::new("bench_onnx", move |b| {
            b.iter(|| {
                let responses: Vec<JoinHandle<_>> = requests
                    .clone()
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
                criterion::black_box(block_on(join_all(responses)));
            })
        })
        .throughput(Throughput::Bytes(
            ((OUTPUT_SIZE * RUN_SIZE * std::mem::size_of::<f64>()) as u32).into(),
        )),
    );
}

criterion_group!(benches, bench_onnx);
criterion_main!(benches);
