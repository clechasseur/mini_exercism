mod get_solution_tests {
    use assert_matches::assert_matches;
    use mini_exercism::api;
    use mini_exercism::Error;
    use reqwest::StatusCode;

    #[tokio::test]
    async fn test_anonymous() {
        let client = api::v1::Client::new();
        let solution_response = client
            .get_solution("00c717b68e1b4213b316df82636f5e0f")
            .await;

        // Querying a solution anonymously fails.
        assert_matches!(solution_response,
            Err(Error::ApiError(error)) if error.status() == Some(StatusCode::UNAUTHORIZED));
    }
}

mod get_latest_solution_tests {
    use assert_matches::assert_matches;
    use mini_exercism::api;
    use mini_exercism::Error;
    use reqwest::StatusCode;

    #[tokio::test]
    async fn test_anonymous() {
        let client = api::v1::Client::new();
        let solution_response = client.get_latest_solution("rust", "poker").await;

        // Querying the latest solution anonymously fails.
        assert_matches!(solution_response,
            Err(Error::ApiError(error)) if error.status() == Some(StatusCode::UNAUTHORIZED));
    }
}

mod get_file_tests {
    use assert_matches::assert_matches;
    use futures::StreamExt;
    use mini_exercism::api;
    use mini_exercism::Error;
    use reqwest::StatusCode;

    #[tokio::test]
    async fn test_anonymous() {
        let client = api::v1::Client::new();
        let mut file_response_stream = client
            .get_file("00c717b68e1b4213b316df82636f5e0f", "Cargo.toml")
            .await;

        // Fetching the contents of a file anonymously fails.
        let file_response = file_response_stream.next().await;
        assert_matches!(file_response,
            Some(Err(Error::ApiError(error))) if error.status() == Some(StatusCode::UNAUTHORIZED));
    }
}

mod get_track_tests {
    use assert_matches::assert_matches;
    use mini_exercism::api;
    use mini_exercism::Error;
    use reqwest::StatusCode;

    #[tokio::test]
    async fn test_anonymous() {
        let client = api::v1::Client::new();
        let track_response = client.get_track("rust").await;

        // Querying a track anonymously fails. (This is a bit strange as the
        // returned object does not actually contain any private information.)
        assert_matches!(track_response,
            Err(Error::ApiError(error)) if error.status() == Some(StatusCode::UNAUTHORIZED));
    }
}

mod validate_token_tests {
    use assert_matches::assert_matches;
    use mini_exercism::api;

    #[tokio::test]
    async fn test_anonymous() {
        let client = api::v1::Client::new();
        let validate_token_response = client.validate_token().await;

        // Validating token anonymously fails, but the method should silently returns `false`.
        assert_matches!(validate_token_response, Ok(false));
    }
}

mod ping_tests {
    use mini_exercism::api;

    #[tokio::test]
    async fn test_ping() {
        let client = api::v1::Client::new();
        let ping_response = client.ping().await;
        let status = ping_response.unwrap().status;
        assert!(status.website);
        assert!(status.database);
    }
}
