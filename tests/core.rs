mod error {
    use std::collections::HashMap;

    use assert_matches::assert_matches;
    use mini_exercism::Error;

    #[test]
    #[cfg(feature = "cli")]
    fn test_config_read_error_from() {
        let error: Error = std::io::Error::from(std::io::ErrorKind::NotFound).into();

        assert_matches!(error,
            Error::ConfigReadError(io_error) if io_error.kind() == std::io::ErrorKind::NotFound);
    }

    #[test]
    #[cfg(feature = "cli")]
    fn test_config_parse_error_from() {
        let invalid_json = "{hello: world}";
        let error: Error = serde_json::from_str::<serde_json::Value>(invalid_json)
            .unwrap_err()
            .into();

        assert_matches!(error,
            Error::ConfigParseError(serde_error) if serde_error.is_syntax());
    }

    #[test]
    fn test_api_error_from() {
        // There's no way to create a reqwest::Error outside of the reqwest crate, so we'll
        // have to trigger an actual error to test this.
        let map_with_non_string_keys: HashMap<_, _> = [(true, "hello"), (false, "world")].into();
        let client = reqwest::Client::new();
        let reqwest_error = client
            .get("/test")
            .json(&map_with_non_string_keys)
            .build()
            .unwrap_err();
        let error: Error = reqwest_error.into();

        assert_matches!(error, Error::ApiError(error) if error.is_builder());
    }
}
