#[cfg(test)]
mod tests {
    use orkhon::orkhon::Orkhon;
    use orkhon::config::OrkhonConfig;
    use orkhon::tensorflow::TFModel;
    use std::path::PathBuf;
    use std::{env, fs};
    use log::*;

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
        Orkhon::new()
            .config(OrkhonConfig::new());
    }

    #[test]
    fn load_tf_model() {
        init();
        Orkhon::new()
            .config(OrkhonConfig::new())
            .tensorflow(TFModel::new());
    }

    #[test]
    fn load_configured_tf_model() {
        init();
        Orkhon::new()
            .config(OrkhonConfig::new())
            .tensorflow(
                TFModel::new()
                    .with_name("mobilenet")
                    .with_model_file(PathBuf::from("mobilenet_v2_1.4_224_frozen.pb"))
            );
    }

    #[test]
    fn build_configured_tf_model() {
        init();

        Orkhon::new()
            .config(OrkhonConfig::new())
            .tensorflow(
                TFModel::new()
                    .with_name("mobilenet")
                    .with_model_file(PathBuf::from(
                        "tests/protobuf/mobilenet_v2_1.4_224_frozen.pb"))
            )
            .build();
    }
}
