mod client {
    use assert_matches::assert_matches;
    use mini_exercism::api;
    use mini_exercism::api::v1::{
        PingResponse, ServiceStatus, Solution, SolutionExercise, SolutionResponse,
        SolutionSubmission, SolutionTrack, SolutionUser, TrackResponse,
    };
    use mini_exercism::core::Credentials;
    use reqwest::StatusCode;
    use wiremock::http::Method::Get;
    use wiremock::matchers::{bearer_token, method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    const API_TOKEN: &str = "some_api_token";

    #[tokio::test]
    async fn test_get_solution() {
        let mock_server = MockServer::start().await;

        let solution_uuid = "00c717b68e1b4213b316df82636f5e0f";
        let solution_response = SolutionResponse {
            solution: Solution {
                uuid: solution_uuid.to_string(),
                url: "https://exercism.org/tracks/rust/exercises/poker".to_string(),
                user: SolutionUser { handle: "clechasseur".to_string(), is_requester: true },
                exercise: SolutionExercise {
                    name: "poker".to_string(),
                    instructions_url: "https://exercism.org/tracks/rust/exercises/poker"
                        .to_string(),
                    track: SolutionTrack { name: "rust".to_string(), title: "Rust".to_string() },
                },
                file_download_base_url: format!(
                    "https://exercism.org/api/v1/solutions/{}/files/",
                    solution_uuid
                ),
                files: vec![
                    ".exercism/config.json".to_string(),
                    "README.md".to_string(),
                    "HELP.md".to_string(),
                    ".gitignore".to_string(),
                    "Cargo.toml".to_string(),
                    "src/lib.rs".to_string(),
                    "tests/poker.rs".to_string(),
                    "src/detail.rs".to_string(),
                    "src/detail/slice_utils.rs".to_string(),
                    "src/detail/slice_utils/group_by.rs".to_string(),
                ],
                submission: Some(SolutionSubmission {
                    submitted_at: "2023-05-07T05:35:43.366Z".to_string(),
                }),
            },
        };
        Mock::given(method(Get))
            .and(path(format!("/solutions/{}", solution_uuid)))
            .and(bearer_token(API_TOKEN))
            .respond_with(ResponseTemplate::new(StatusCode::OK).set_body_json(solution_response))
            .mount(&mock_server)
            .await;

        let client = api::v1::Client::builder()
            .api_base_url(mock_server.uri().as_str())
            .credentials(Credentials::from_api_token(API_TOKEN))
            .build();
        let solution_response = client.get_solution(solution_uuid).await;
        assert_matches!(solution_response, Ok(_));

        let solution = solution_response.unwrap().solution;
        assert_eq!(solution_uuid, solution.uuid);
        assert_eq!("poker", solution.exercise.name);
        assert_eq!("rust", solution.exercise.track.name);
    }

    #[tokio::test]
    async fn test_get_latest_solution() {
        let mock_server = MockServer::start().await;

        let solution_uuid = "00c717b68e1b4213b316df82636f5e0f";
        let solution_response = SolutionResponse {
            solution: Solution {
                uuid: solution_uuid.to_string(),
                url: "https://exercism.org/tracks/rust/exercises/poker".to_string(),
                user: SolutionUser { handle: "clechasseur".to_string(), is_requester: true },
                exercise: SolutionExercise {
                    name: "poker".to_string(),
                    instructions_url: "https://exercism.org/tracks/rust/exercises/poker"
                        .to_string(),
                    track: SolutionTrack { name: "rust".to_string(), title: "Rust".to_string() },
                },
                file_download_base_url: format!(
                    "https://exercism.org/api/v1/solutions/{}/files/",
                    solution_uuid
                ),
                files: vec![
                    ".exercism/config.json".to_string(),
                    "README.md".to_string(),
                    "HELP.md".to_string(),
                    ".gitignore".to_string(),
                    "Cargo.toml".to_string(),
                    "src/lib.rs".to_string(),
                    "tests/poker.rs".to_string(),
                    "src/detail.rs".to_string(),
                    "src/detail/slice_utils.rs".to_string(),
                    "src/detail/slice_utils/group_by.rs".to_string(),
                ],
                submission: Some(SolutionSubmission {
                    submitted_at: "2023-05-07T05:35:43.366Z".to_string(),
                }),
            },
        };
        Mock::given(method(Get))
            .and(path("/solutions/latest"))
            .and(query_param("track_id", "rust"))
            .and(query_param("exercise_id", "poker"))
            .and(bearer_token(API_TOKEN))
            .respond_with(ResponseTemplate::new(StatusCode::OK).set_body_json(solution_response))
            .mount(&mock_server)
            .await;

        let client = api::v1::Client::builder()
            .api_base_url(mock_server.uri().as_str())
            .credentials(Credentials::from_api_token(API_TOKEN))
            .build();
        let solution_response = client.get_latest_solution("rust", "poker").await;
        assert_matches!(solution_response, Ok(_));

        let solution = solution_response.unwrap().solution;
        assert_eq!(solution_uuid, solution.uuid);
        assert_eq!("poker", solution.exercise.name);
        assert_eq!("rust", solution.exercise.track.name);
    }

    #[tokio::test]
    async fn test_get_track() {
        let mock_server = MockServer::start().await;

        let track_response = TrackResponse {
            track: SolutionTrack { name: "rust".to_string(), title: "Rust".to_string() },
        };
        Mock::given(method(Get))
            .and(path("/tracks/rust"))
            .and(bearer_token(API_TOKEN))
            .respond_with(ResponseTemplate::new(StatusCode::OK).set_body_json(track_response))
            .mount(&mock_server)
            .await;

        let client = api::v1::Client::builder()
            .api_base_url(mock_server.uri().as_str())
            .credentials(Credentials::from_api_token(API_TOKEN))
            .build();
        let track_response = client.get_track("rust").await;
        assert_matches!(track_response, Ok(_));

        let track = track_response.unwrap().track;
        assert_eq!("rust", track.name);
        assert_eq!("Rust", track.title);
    }

    #[tokio::test]
    async fn test_validate_token() {
        let mock_server = MockServer::start().await;

        Mock::given(method(Get))
            .and(path("/validate_token"))
            .and(bearer_token(API_TOKEN))
            .respond_with(ResponseTemplate::new(StatusCode::OK))
            .mount(&mock_server)
            .await;

        let client = api::v1::Client::builder()
            .api_base_url(mock_server.uri().as_str())
            .credentials(Credentials::from_api_token(API_TOKEN))
            .build();
        let validate_token_response = client.validate_token().await;
        assert_matches!(validate_token_response, Ok(true));
    }

    #[tokio::test]
    async fn test_ping() {
        let mock_server = MockServer::start().await;

        let ping_response =
            PingResponse { status: ServiceStatus { website: true, database: true } };
        Mock::given(method(Get))
            .and(path("/ping"))
            .respond_with(ResponseTemplate::new(StatusCode::OK).set_body_json(ping_response))
            .mount(&mock_server)
            .await;

        let client = api::v1::Client::builder()
            .api_base_url(mock_server.uri().as_str())
            .build();
        let ping_response = client.ping().await;
        assert_matches!(ping_response, Ok(_));

        let status = ping_response.unwrap().status;
        assert!(status.website);
        assert!(status.database);
    }
}

mod solution_response {
    mod deserialize {
        use mini_exercism::api::v1::{
            Solution, SolutionExercise, SolutionResponse, SolutionSubmission, SolutionTrack,
            SolutionUser,
        };

        #[test]
        fn test_all() {
            let json = r#"{
                "solution": {
                    "id": "00c717b68e1b4213b316df82636f5e0f",
                    "url": "https://exercism.org/tracks/rust/exercises/poker",
                    "user": {
                        "handle": "clechasseur",
                        "is_requester": true
                    },
                    "exercise": {
                        "id": "poker",
                        "instructions_url": "https://exercism.org/tracks/rust/exercises/poker",
                        "track": {
                            "id": "rust",
                            "language": "Rust"
                        }
                    },
                    "file_download_base_url": "https://exercism.org/api/v1/solutions/00c717b68e1b4213b316df82636f5e0f/files/",
                    "files": [
                        ".exercism/config.json",
                        "README.md",
                        "HELP.md",
                        ".gitignore",
                        "Cargo.toml",
                        "src/lib.rs",
                        "tests/poker.rs",
                        "src/detail.rs",
                        "src/detail/slice_utils.rs",
                        "src/detail/slice_utils/group_by.rs"
                    ],
                    "submission": {
                        "submitted_at": "2023-05-07T05:35:43.366Z"
                    }
                }
            }"#;

            let expected = SolutionResponse {
                solution: Solution {
                    uuid: "00c717b68e1b4213b316df82636f5e0f".to_string(),
                    url: "https://exercism.org/tracks/rust/exercises/poker".to_string(),
                    user: SolutionUser {
                        handle: "clechasseur".to_string(),
                        is_requester: true,
                    },
                    exercise: SolutionExercise {
                        name: "poker".to_string(),
                        instructions_url: "https://exercism.org/tracks/rust/exercises/poker".to_string(),
                        track: SolutionTrack {
                            name: "rust".to_string(),
                            title: "Rust".to_string(),
                        },
                    },
                    file_download_base_url: "https://exercism.org/api/v1/solutions/00c717b68e1b4213b316df82636f5e0f/files/".to_string(),
                    files: vec![
                        ".exercism/config.json".to_string(),
                        "README.md".to_string(),
                        "HELP.md".to_string(),
                        ".gitignore".to_string(),
                        "Cargo.toml".to_string(),
                        "src/lib.rs".to_string(),
                        "tests/poker.rs".to_string(),
                        "src/detail.rs".to_string(),
                        "src/detail/slice_utils.rs".to_string(),
                        "src/detail/slice_utils/group_by.rs".to_string(),
                    ],
                    submission: Some(SolutionSubmission {
                        submitted_at: "2023-05-07T05:35:43.366Z".to_string(),
                    }),
                },
            };
            let actual = serde_json::from_str(json).unwrap();
            assert_eq!(expected, actual);
        }
    }
}

mod solution {
    mod deserialize {
        use mini_exercism::api::v1::{
            Solution, SolutionExercise, SolutionSubmission, SolutionTrack, SolutionUser,
        };

        #[test]
        fn test_all() {
            let json = r#"{
                "id": "00c717b68e1b4213b316df82636f5e0f",
                "url": "https://exercism.org/tracks/rust/exercises/poker",
                "user": {
                    "handle": "clechasseur",
                    "is_requester": true
                },
                "exercise": {
                    "id": "poker",
                    "instructions_url": "https://exercism.org/tracks/rust/exercises/poker",
                    "track": {
                        "id": "rust",
                        "language": "Rust"
                    }
                },
                "file_download_base_url": "https://exercism.org/api/v1/solutions/00c717b68e1b4213b316df82636f5e0f/files/",
                "files": [
                    ".exercism/config.json",
                    "README.md",
                    "HELP.md",
                    ".gitignore",
                    "Cargo.toml",
                    "src/lib.rs",
                    "tests/poker.rs",
                    "src/detail.rs",
                    "src/detail/slice_utils.rs",
                    "src/detail/slice_utils/group_by.rs"
                ],
                "submission": {
                    "submitted_at": "2023-05-07T05:35:43.366Z"
                }
            }"#;

            let expected = Solution {
                uuid: "00c717b68e1b4213b316df82636f5e0f".to_string(),
                url: "https://exercism.org/tracks/rust/exercises/poker".to_string(),
                user: SolutionUser { handle: "clechasseur".to_string(), is_requester: true },
                exercise: SolutionExercise {
                    name: "poker".to_string(),
                    instructions_url: "https://exercism.org/tracks/rust/exercises/poker"
                        .to_string(),
                    track: SolutionTrack { name: "rust".to_string(), title: "Rust".to_string() },
                },
                file_download_base_url:
                    "https://exercism.org/api/v1/solutions/00c717b68e1b4213b316df82636f5e0f/files/"
                        .to_string(),
                files: vec![
                    ".exercism/config.json".to_string(),
                    "README.md".to_string(),
                    "HELP.md".to_string(),
                    ".gitignore".to_string(),
                    "Cargo.toml".to_string(),
                    "src/lib.rs".to_string(),
                    "tests/poker.rs".to_string(),
                    "src/detail.rs".to_string(),
                    "src/detail/slice_utils.rs".to_string(),
                    "src/detail/slice_utils/group_by.rs".to_string(),
                ],
                submission: Some(SolutionSubmission {
                    submitted_at: "2023-05-07T05:35:43.366Z".to_string(),
                }),
            };
            let actual = serde_json::from_str(json).unwrap();
            assert_eq!(expected, actual);
        }
    }
}

mod solution_user {
    mod deserialize {
        use mini_exercism::api::v1::SolutionUser;

        #[test]
        fn test_all() {
            let json = r#"{
                "handle": "clechasseur",
                "is_requester": true
            }"#;

            let expected = SolutionUser { handle: "clechasseur".to_string(), is_requester: true };
            let actual = serde_json::from_str(json).unwrap();
            assert_eq!(expected, actual);
        }
    }
}

mod solution_exercise {
    mod deserialize {
        use mini_exercism::api::v1::{SolutionExercise, SolutionTrack};

        #[test]
        fn test_all() {
            let json = r#"{
                "id": "poker",
                "instructions_url": "https://exercism.org/tracks/rust/exercises/poker",
                "track": {
                    "id": "rust",
                    "language": "Rust"
                }
            }"#;

            let expected = SolutionExercise {
                name: "poker".to_string(),
                instructions_url: "https://exercism.org/tracks/rust/exercises/poker".to_string(),
                track: SolutionTrack { name: "rust".to_string(), title: "Rust".to_string() },
            };
            let actual = serde_json::from_str(json).unwrap();
            assert_eq!(expected, actual);
        }
    }
}

mod solution_track {
    mod deserialize {
        use mini_exercism::api::v1::SolutionTrack;

        #[test]
        fn test_all() {
            let json = r#"{
                "id": "rust",
                "language": "Rust"
            }"#;

            let expected = SolutionTrack { name: "rust".to_string(), title: "Rust".to_string() };
            let actual = serde_json::from_str(json).unwrap();
            assert_eq!(expected, actual);
        }
    }
}

mod solution_submission {
    mod deserialize {
        use mini_exercism::api::v1::SolutionSubmission;

        #[test]
        fn test_all() {
            let json = r#"{
                "submitted_at": "2023-05-07T05:35:43.366Z"
            }"#;

            let expected =
                SolutionSubmission { submitted_at: "2023-05-07T05:35:43.366Z".to_string() };
            let actual = serde_json::from_str(json).unwrap();
            assert_eq!(expected, actual);
        }
    }
}

mod track_response {
    mod deserialize {
        use mini_exercism::api::v1::{SolutionTrack, TrackResponse};

        #[test]
        fn test_all() {
            let json = r#"{
                "track": {
                    "id": "awk",
                    "language": "AWK"
                }
            }"#;

            let expected = TrackResponse {
                track: SolutionTrack { name: "awk".to_string(), title: "AWK".to_string() },
            };
            let actual = serde_json::from_str(json).unwrap();
            assert_eq!(expected, actual);
        }
    }
}

mod ping_response {
    mod deserialize {
        use mini_exercism::api::v1::{PingResponse, ServiceStatus};

        #[test]
        fn test_all() {
            let json = r#"{
                "status": {
                    "website": true,
                    "database": true
                }
            }"#;

            let expected = PingResponse { status: ServiceStatus { website: true, database: true } };
            let actual = serde_json::from_str(json).unwrap();
            assert_eq!(expected, actual);
        }
    }
}

mod service_status {
    mod deserialize {
        use mini_exercism::api::v1::ServiceStatus;

        #[test]
        fn test_all() {
            let json = r#"{
                "website": true,
                "database": true
            }"#;

            let expected = ServiceStatus { website: true, database: true };
            let actual = serde_json::from_str(json).unwrap();
            assert_eq!(expected, actual);
        }
    }
}
