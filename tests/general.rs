use orkhon::config::OrkhonConfig;
use orkhon::orkhon::Orkhon;
use orkhon::tcore::prelude::*;
use orkhon::tcore::tract_data::prelude::Datum;
use orkhon::ttensor::prelude::*;
use std::path::PathBuf;

fn init() {
    let _ = env_logger::builder().is_test(true).try_init();
}

#[test]
fn initialize_orkhon() {
    init();
    Orkhon::new();
}

#[test]
fn pass_config_to_orkhon() {
    init();
    Orkhon::new().config(OrkhonConfig::new());
}

#[test]
fn load_tf_model() {
    init();
    Orkhon::new().config(OrkhonConfig::new()).tensorflow(
        "mobilenet",
        PathBuf::from("tests/protobuf/mobilenet_v2_1.4_224_frozen.pb"),
    );
}

#[test]
fn load_configured_tf_model() {
    init();
    Orkhon::new().config(OrkhonConfig::new()).tensorflow(
        "mobilenet",
        PathBuf::from("tests/protobuf/mobilenet_v2_1.4_224_frozen.pb"),
    );
}

#[test]
#[ignore = "Needs a model to test"]
fn build_auto_input_inferred_tf_model() {
    init();
    // If you want to infer input tensor shapes you need to use the saved model in directory coming out of `model.save()`.
    // Since tf backend forces us to use "saved_model" naming. Always give a file with that name to it.
    Orkhon::new()
        .config(OrkhonConfig::new().with_auto_load_input_facts())
        .tensorflow(
            "auto_input_infer",
            PathBuf::from("tests/protobuf/auto_input_infer/my_model/saved_model.pb"),
        )
        .build();
}

#[test]
fn build_manual_input_tf_model() {
    init();
    // If you want to infer input tensor shapes you need to use the saved model in directory coming out of `model.save()`.
    // Since tf backend forces us to use "saved_model" naming. Always give a file with that name to it.
    Orkhon::new()
        .config(
            OrkhonConfig::new()
                .with_default_input_fact_shape(InferenceFact::dt_shape(f32::datum_type(), tvec![10, 100])),
        )
        .tensorflow(
            "manual_input_infer",
            PathBuf::from("tests/protobuf/manual_input_infer/my_model.pb"),
        )
        .build();
}

#[test]
#[cfg(feature = "onnxmodel")]
fn build_manual_input_onnx_model() {
    init();
    // If you want to infer input tensor shapes you need to use the saved model in directory coming out of `model.save()`.
    // Since tf backend forces us to use "saved_model" naming. Always give a file with that name to it.
    Orkhon::new()
        .config(
            OrkhonConfig::new()
                .with_default_input_fact_shape(InferenceFact::dt_shape(f32::datum_type(), tvec![10, 100])),
        )
        .onnx(
            "onnx_model",
            PathBuf::from("tests/protobuf/onnx_model/example.onnx"),
        )
        .build();
}

#[test]
#[cfg(feature = "pymodel")]
fn build_configured_python_model() {
    init();

    Orkhon::new()
        .config(OrkhonConfig::new())
        .pymodel("mobilenet", "tests/pymodels", "prefix", "data")
        .build();
}

#[test]
#[cfg(feature = "pymodel")]
fn sync_request_python_model() {
    init();

    Orkhon::new()
        .config(OrkhonConfig::new())
        .pymodel("mobilenet", "tests/pymodels", "prefix", "data")
        .build();
}
