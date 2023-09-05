mod error {
    use std::collections::HashMap;
    #[cfg(feature = "cli")]
    use std::io;

    use assert_matches::assert_matches;
    use mini_exercism::core::Error;

    #[test]
    #[cfg(feature = "cli")]
    fn test_config_read_error_from() {
        let error: Error = io::Error::from(io::ErrorKind::NotFound).into();

        assert_matches!(error, Error::ConfigReadError(_));
    }

    #[test]
    #[cfg(feature = "cli")]
    fn test_config_parse_error_from() {
        let invalid_json = "{hello: world}";
        let error: Error = serde_json::from_str::<serde_json::Value>(invalid_json)
            .unwrap_err()
            .into();

        assert_matches!(error, Error::ConfigParseError(_));
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

        assert_matches!(error, Error::ApiError(_));
    }
}
