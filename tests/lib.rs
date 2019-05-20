#[cfg(test)]
mod tests {
    use orkhon::orkhon::Orkhon;
    use orkhon::config::OrkhonConfig;
    use orkhon::tensorflow::TFModel;
    use std::path::PathBuf;

    #[test]
    fn initialize_orkhon() {
        Orkhon::new();
    }

    #[test]
    fn pass_config_to_orkhon() {
        Orkhon::new()
            .config(OrkhonConfig::new());
    }

    #[test]
    fn load_tf_model() {
        Orkhon::new()
            .config(OrkhonConfig::new())
            .tensorflow(TFModel::new());
    }

    #[test]
    fn load_configured_tf_model() {
        Orkhon::new()
            .config(OrkhonConfig::new())
            .tensorflow(TFModel {
                name: "mobilenet",
                file: PathBuf::from("mobilenet_v2_1.4_224_frozen.pb")
            });
    }
}
