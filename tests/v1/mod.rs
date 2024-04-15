mod ping;
mod solution;
mod track;

mod client {
    use mini_exercism::api;
    use mini_exercism::core::Credentials;
    use wiremock::matchers::{bearer_token, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    const API_TOKEN: &str = "some_api_token";

    mod debug {
        use super::*;

        #[test]
        fn test_derive() {
            // Note: this test is necessary because of a bug in cargo-tarpaulin, see
            // https://github.com/xd009642/tarpaulin/issues/351#issuecomment-1722148936
            let client = api::v1::Client::new();
            assert!(!format!("{:?}", client).is_empty());
        }
    }

    mod get_solution {
        use mini_exercism::api::v1::solution;
        use mini_exercism::api::v1::solution::{Exercise, Solution, Submission, User};
        use mini_exercism::api::v1::track::Track;

        use super::*;

        #[tokio::test]
        async fn test_all() {
            let mock_server = MockServer::start().await;

            let solution_uuid = "00c717b68e1b4213b316df82636f5e0f";
            let solution_response = solution::Response {
                solution: Solution {
                    uuid: solution_uuid.into(),
                    url: "https://exercism.org/tracks/rust/exercises/poker".into(),
                    user: User { handle: "clechasseur".into(), is_requester: true },
                    exercise: Exercise {
                        name: "poker".into(),
                        instructions_url: "https://exercism.org/tracks/rust/exercises/poker".into(),
                        track: Track { name: "rust".into(), title: "Rust".into() },
                    },
                    file_download_base_url: format!(
                        "https://exercism.org/api/v1/solutions/{}/files/",
                        solution_uuid
                    ),
                    files: vec![
                        ".exercism/config.json".into(),
                        "README.md".into(),
                        "HELP.md".into(),
                        ".gitignore".into(),
                        "Cargo.toml".into(),
                        "src/lib.rs".into(),
                        "tests/poker.rs".into(),
                        "src/detail.rs".into(),
                        "src/detail/slice_utils.rs".into(),
                        "src/detail/slice_utils/group_by.rs".into(),
                    ],
                    submission: Some(Submission {
                        submitted_at: "2023-05-07T05:35:43.366Z".into(),
                    }),
                },
            };
            Mock::given(method(http::Method::GET))
                .and(path(format!("/solutions/{}", solution_uuid)))
                .and(bearer_token(API_TOKEN))
                .respond_with(
                    ResponseTemplate::new(http::StatusCode::OK).set_body_json(solution_response),
                )
                .mount(&mock_server)
                .await;

            let client = api::v1::Client::builder()
                .api_base_url(mock_server.uri().as_str())
                .credentials(Credentials::from_api_token(API_TOKEN))
                .build()
                .unwrap();
            let solution_response = client.get_solution(solution_uuid).await;
            let solution = solution_response.unwrap().solution;
            assert_eq!(solution_uuid, solution.uuid);
            assert_eq!("poker", solution.exercise.name);
            assert_eq!("rust", solution.exercise.track.name);
        }
    }

    mod get_latest_solution {
        use mini_exercism::api::v1::solution;
        use mini_exercism::api::v1::solution::{Exercise, Solution, Submission, User};
        use mini_exercism::api::v1::track::Track;
        use wiremock::matchers::query_param;

        use super::*;

        #[tokio::test]
        async fn test_all() {
            let mock_server = MockServer::start().await;

            let solution_uuid = "00c717b68e1b4213b316df82636f5e0f";
            let solution_response = solution::Response {
                solution: Solution {
                    uuid: solution_uuid.into(),
                    url: "https://exercism.org/tracks/rust/exercises/poker".into(),
                    user: User { handle: "clechasseur".into(), is_requester: true },
                    exercise: Exercise {
                        name: "poker".into(),
                        instructions_url: "https://exercism.org/tracks/rust/exercises/poker".into(),
                        track: Track { name: "rust".into(), title: "Rust".into() },
                    },
                    file_download_base_url: format!(
                        "https://exercism.org/api/v1/solutions/{}/files/",
                        solution_uuid
                    ),
                    files: vec![
                        ".exercism/config.json".into(),
                        "README.md".into(),
                        "HELP.md".into(),
                        ".gitignore".into(),
                        "Cargo.toml".into(),
                        "src/lib.rs".into(),
                        "tests/poker.rs".into(),
                        "src/detail.rs".into(),
                        "src/detail/slice_utils.rs".into(),
                        "src/detail/slice_utils/group_by.rs".into(),
                    ],
                    submission: Some(Submission {
                        submitted_at: "2023-05-07T05:35:43.366Z".into(),
                    }),
                },
            };
            Mock::given(method(http::Method::GET))
                .and(path("/solutions/latest"))
                .and(query_param("track_id", "rust"))
                .and(query_param("exercise_id", "poker"))
                .and(bearer_token(API_TOKEN))
                .respond_with(
                    ResponseTemplate::new(http::StatusCode::OK).set_body_json(solution_response),
                )
                .mount(&mock_server)
                .await;

            let client = api::v1::Client::builder()
                .api_base_url(mock_server.uri().as_str())
                .credentials(Credentials::from_api_token(API_TOKEN))
                .build()
                .unwrap();
            let solution_response = client.get_latest_solution("rust", "poker").await;
            let solution = solution_response.unwrap().solution;
            assert_eq!(solution_uuid, solution.uuid);
            assert_eq!("poker", solution.exercise.name);
            assert_eq!("rust", solution.exercise.track.name);
        }
    }

    mod get_file {
        use std::io::Write;

        use assert_matches::assert_matches;
        use futures::StreamExt;
        use mini_exercism::Error;
        use wiremock::matchers::path_regex;

        use super::*;

        #[tokio::test]
        async fn test_valid_file() {
            let mock_server = MockServer::start().await;

            let solution_uuid = "00c717b68e1b4213b316df82636f5e0f";
            let file_path = "Cargo.toml";
            let file_content = r#"[package]
edition = "2021"
name = "poker"
version = "1.1.0"

[dependencies]
"#;
            Mock::given(method(http::Method::GET))
                .and(path(format!("/solutions/{}/files/{}", solution_uuid, file_path)))
                .and(bearer_token(API_TOKEN))
                .respond_with(
                    ResponseTemplate::new(http::StatusCode::OK).set_body_string(file_content),
                )
                .mount(&mock_server)
                .await;

            let client = api::v1::Client::builder()
                .api_base_url(mock_server.uri().as_str())
                .credentials(Credentials::from_api_token(API_TOKEN))
                .build()
                .unwrap();
            let mut file_response = client.get_file(solution_uuid, file_path).await;
            let mut output: Vec<u8> = Vec::new();
            while let Some(bytes) = file_response.next().await {
                output.write_all(&(bytes.unwrap())).unwrap();
            }
            assert_eq!(file_content, String::from_utf8(output).unwrap());
        }

        #[tokio::test]
        async fn test_invalid_file() {
            let mock_server = MockServer::start().await;

            let solution_uuid = "00c717b68e1b4213b316df82636f5e0f";
            let file_path = "Cargo.toml";
            Mock::given(method(http::Method::GET))
                .and(path_regex(r"^/solutions/[^/]+/files/.+$"))
                .and(bearer_token(API_TOKEN))
                .respond_with(ResponseTemplate::new(http::StatusCode::NOT_FOUND))
                .mount(&mock_server)
                .await;

            let client = api::v1::Client::builder()
                .api_base_url(mock_server.uri().as_str())
                .credentials(Credentials::from_api_token(API_TOKEN))
                .build()
                .unwrap();
            let mut file_response = client.get_file(solution_uuid, file_path).await;

            let file_result = file_response.next().await.unwrap();
            assert_matches!(file_result,
                Err(Error::ApiError(api_error)) if api_error.status() == Some(reqwest::StatusCode::NOT_FOUND));
        }
    }

    mod get_track {
        use mini_exercism::api::v1::track;
        use mini_exercism::api::v1::track::Track;

        use super::*;

        #[tokio::test]
        async fn test_all() {
            let mock_server = MockServer::start().await;

            let track_response =
                track::Response { track: Track { name: "rust".into(), title: "Rust".into() } };
            Mock::given(method(http::Method::GET))
                .and(path("/tracks/rust"))
                .and(bearer_token(API_TOKEN))
                .respond_with(
                    ResponseTemplate::new(http::StatusCode::OK).set_body_json(track_response),
                )
                .mount(&mock_server)
                .await;

            let client = api::v1::Client::builder()
                .api_base_url(mock_server.uri().as_str())
                .credentials(Credentials::from_api_token(API_TOKEN))
                .build()
                .unwrap();
            let track_response = client.get_track("rust").await;
            let track = track_response.unwrap().track;
            assert_eq!("rust", track.name);
            assert_eq!("Rust", track.title);
        }
    }

    mod validate_token {
        use assert_matches::assert_matches;
        use mini_exercism::Error;

        use super::*;

        #[tokio::test]
        async fn test_valid_token() {
            let mock_server = MockServer::start().await;

            Mock::given(method(http::Method::GET))
                .and(path("/validate_token"))
                .and(bearer_token(API_TOKEN))
                .respond_with(ResponseTemplate::new(http::StatusCode::OK))
                .mount(&mock_server)
                .await;

            let client = api::v1::Client::builder()
                .api_base_url(mock_server.uri().as_str())
                .credentials(Credentials::from_api_token(API_TOKEN))
                .build()
                .unwrap();
            let validate_token_response = client.validate_token().await;
            assert_matches!(validate_token_response, Ok(true));
        }

        #[tokio::test]
        async fn test_invalid_token() {
            let mock_server = MockServer::start().await;

            Mock::given(method(http::Method::GET))
                .and(path("/validate_token"))
                .respond_with(ResponseTemplate::new(http::StatusCode::UNAUTHORIZED))
                .mount(&mock_server)
                .await;

            let client = api::v1::Client::builder()
                .api_base_url(mock_server.uri().as_str())
                .credentials(Credentials::from_api_token(API_TOKEN))
                .build()
                .unwrap();
            let validate_token_response = client.validate_token().await;
            assert_matches!(validate_token_response, Ok(false));
        }

        #[tokio::test]
        async fn test_internal_server_error() {
            let mock_server = MockServer::start().await;

            Mock::given(method(http::Method::GET))
                .and(path("/validate_token"))
                .respond_with(ResponseTemplate::new(http::StatusCode::INTERNAL_SERVER_ERROR))
                .mount(&mock_server)
                .await;

            let client = api::v1::Client::builder()
                .api_base_url(mock_server.uri().as_str())
                .credentials(Credentials::from_api_token(API_TOKEN))
                .build()
                .unwrap();
            let validate_token_response = client.validate_token().await;
            assert_matches!(validate_token_response,
                Err(Error::ApiError(error)) if error.status() == Some(reqwest::StatusCode::INTERNAL_SERVER_ERROR));
        }
    }

    mod ping {
        use mini_exercism::api::v1::ping::ServiceStatus;

        use super::*;

        #[tokio::test]
        async fn test_all() {
            let mock_server = MockServer::start().await;

            let ping_response =
                api::v1::ping::Response { status: ServiceStatus { website: true, database: true } };
            Mock::given(method(http::Method::GET))
                .and(path("/ping"))
                .respond_with(
                    ResponseTemplate::new(http::StatusCode::OK).set_body_json(ping_response),
                )
                .mount(&mock_server)
                .await;

            let client = api::v1::Client::builder()
                .api_base_url(mock_server.uri().as_str())
                .build()
                .unwrap();
            let ping_response = client.ping().await;
            let status = ping_response.unwrap().status;
            assert!(status.website);
            assert!(status.database);
        }
    }
}
