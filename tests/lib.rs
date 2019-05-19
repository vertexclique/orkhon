#[cfg(test)]
mod tests {
    use orkhon::orkhon::Orkhon;
    use orkhon::config::OrkhonConfig;
    use orkhon::tensorflow::TFModel;

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
}
