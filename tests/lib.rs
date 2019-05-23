#![feature(async_await)]

#[cfg(test)]
mod tests {
    use orkhon::orkhon::Orkhon;
    use orkhon::config::OrkhonConfig;
    use orkhon::tensorflow::TFModel;
    use std::path::PathBuf;
    use std::{env, fs};
    use log::*;
    use orkhon::pooled::PooledModel;
    use orkhon::reqrep::ORequest;

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
            .tensorflow("mobilenet",
                        PathBuf::from("tests/protobuf/mobilenet_v2_1.4_224_frozen.pb")
            );
    }

    #[test]
    fn load_configured_tf_model() {
        init();
        Orkhon::new()
            .config(OrkhonConfig::new())
            .tensorflow("mobilenet",
                        PathBuf::from("tests/protobuf/mobilenet_v2_1.4_224_frozen.pb")
            );
    }

    #[test]
    fn build_configured_tf_model() {
        init();

        Orkhon::new()
            .config(OrkhonConfig::new())
            .tensorflow("mobilenet",
                        PathBuf::from("tests/protobuf/mobilenet_v2_1.4_224_frozen.pb")
            )
            .build();
    }

    #[test]
    fn build_configured_python_model() {
        init();

        Orkhon::new()
            .config(OrkhonConfig::new())
            .pymodel("mobilenet", "tests/pymodels", "prefix")
            .build();
    }
}
