mod get_solution {
    use assert_matches::assert_matches;
    use mini_exercism::Error;
    use mini_exercism::api;
    use mini_exercism::http::StatusCode;
    use serial_test::file_serial;

    #[tokio::test]
    #[file_serial(real_endpoints)]
    async fn test_anonymous() {
        let client = api::v1::Client::new().unwrap();
        let solution_response = client
            .get_solution("00c717b68e1b4213b316df82636f5e0f")
            .await;

        // Querying a solution anonymously fails.
        assert_matches!(solution_response,
            Err(Error::ApiError(error)) if error.status() == Some(StatusCode::UNAUTHORIZED));
    }
}

mod get_latest_solution {
    use assert_matches::assert_matches;
    use mini_exercism::Error;
    use mini_exercism::api;
    use mini_exercism::http::StatusCode;
    use serial_test::file_serial;

    #[tokio::test]
    #[file_serial(real_endpoints)]
    async fn test_anonymous() {
        let client = api::v1::Client::new().unwrap();
        let solution_response = client.get_latest_solution("rust", "poker").await;

        // Querying the latest solution anonymously fails.
        assert_matches!(solution_response,
            Err(Error::ApiError(error)) if error.status() == Some(StatusCode::UNAUTHORIZED));
    }
}

mod get_file {
    use assert_matches::assert_matches;
    use futures::StreamExt;
    use mini_exercism::Error;
    use mini_exercism::api;
    use mini_exercism::http::StatusCode;
    use serial_test::file_serial;

    #[tokio::test]
    #[file_serial(real_endpoints)]
    async fn test_anonymous() {
        let client = api::v1::Client::new().unwrap();
        let mut file_response_stream = client
            .get_file("00c717b68e1b4213b316df82636f5e0f", "Cargo.toml")
            .await;

        // Fetching the contents of a file anonymously fails.
        let file_response = file_response_stream.next().await;
        assert_matches!(file_response,
            Some(Err(Error::ApiError(error))) if error.status() == Some(StatusCode::UNAUTHORIZED));
    }
}

mod get_track {
    use assert_matches::assert_matches;
    use mini_exercism::Error;
    use mini_exercism::api;
    use mini_exercism::http::StatusCode;
    use serial_test::file_serial;

    #[tokio::test]
    #[file_serial(real_endpoints)]
    async fn test_anonymous() {
        let client = api::v1::Client::new().unwrap();
        let track_response = client.get_track("rust").await;

        // Querying a track anonymously fails. (This is a bit strange as the
        // returned object does not actually contain any private information.)
        assert_matches!(track_response,
            Err(Error::ApiError(error)) if error.status() == Some(StatusCode::UNAUTHORIZED));
    }
}

mod validate_token {
    use assert_matches::assert_matches;
    use mini_exercism::api;
    use serial_test::file_serial;

    #[tokio::test]
    #[file_serial(real_endpoints)]
    async fn test_anonymous() {
        let client = api::v1::Client::new().unwrap();
        let validate_token_response = client.validate_token().await;

        // Validating token anonymously fails, but the method should silently returns `false`.
        assert_matches!(validate_token_response, Ok(false));
    }
}

mod ping {
    use mini_exercism::api;
    use serial_test::file_serial;

    #[tokio::test]
    #[file_serial(real_endpoints)]
    async fn test_ping() {
        let client = api::v1::Client::new().unwrap();
        let ping_response = client.ping().await;
        let status = ping_response.unwrap().status;
        assert!(status.website);
        assert!(status.database);
    }
}
