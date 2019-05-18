#[cfg(test)]
mod tests {
    use orkhon::orkhon::Orkhon;
    use orkhon::config::OrkhonConfig;

    #[test]
    fn initialize_orkhon() {
        Orkhon::new();
    }

    #[test]
    fn pass_config_to_orkhon() {
        Orkhon::new()
            .config(OrkhonConfig::new());
    }
}
