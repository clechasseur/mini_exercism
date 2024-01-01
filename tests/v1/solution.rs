mod response {
    mod deserialize {
        use mini_exercism::api::v1::solution;
        use mini_exercism::api::v1::solution::{Exercise, Solution, Submission, User};
        use mini_exercism::api::v1::track::Track;

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

            let expected = solution::Response {
                solution: Solution {
                    uuid: "00c717b68e1b4213b316df82636f5e0f".into(),
                    url: "https://exercism.org/tracks/rust/exercises/poker".into(),
                    user: User {
                        handle: "clechasseur".into(),
                        is_requester: true,
                    },
                    exercise: Exercise {
                        name: "poker".into(),
                        instructions_url: "https://exercism.org/tracks/rust/exercises/poker".into(),
                        track: Track {
                            name: "rust".into(),
                            title: "Rust".into(),
                        },
                    },
                    file_download_base_url: "https://exercism.org/api/v1/solutions/00c717b68e1b4213b316df82636f5e0f/files/".into(),
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
            let actual = serde_json::from_str(json).unwrap();
            assert_eq!(expected, actual);
        }
    }
}

mod solution_test {
    mod deserialize {
        use mini_exercism::api::v1::solution::{Exercise, Solution, Submission, User};
        use mini_exercism::api::v1::track::Track;

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
                uuid: "00c717b68e1b4213b316df82636f5e0f".into(),
                url: "https://exercism.org/tracks/rust/exercises/poker".into(),
                user: User { handle: "clechasseur".into(), is_requester: true },
                exercise: Exercise {
                    name: "poker".into(),
                    instructions_url: "https://exercism.org/tracks/rust/exercises/poker".into(),
                    track: Track { name: "rust".into(), title: "Rust".into() },
                },
                file_download_base_url:
                    "https://exercism.org/api/v1/solutions/00c717b68e1b4213b316df82636f5e0f/files/"
                        .into(),
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
                submission: Some(Submission { submitted_at: "2023-05-07T05:35:43.366Z".into() }),
            };
            let actual = serde_json::from_str(json).unwrap();
            assert_eq!(expected, actual);
        }
    }
}

mod user {
    mod deserialize {
        use mini_exercism::api::v1::solution::User;

        #[test]
        fn test_all() {
            let json = r#"{
                "handle": "clechasseur",
                "is_requester": true
            }"#;

            let expected = User { handle: "clechasseur".into(), is_requester: true };
            let actual = serde_json::from_str(json).unwrap();
            assert_eq!(expected, actual);
        }
    }
}

mod exercise {
    mod deserialize {
        use mini_exercism::api::v1::solution::Exercise;
        use mini_exercism::api::v1::track::Track;

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

            let expected = Exercise {
                name: "poker".into(),
                instructions_url: "https://exercism.org/tracks/rust/exercises/poker".into(),
                track: Track { name: "rust".into(), title: "Rust".into() },
            };
            let actual = serde_json::from_str(json).unwrap();
            assert_eq!(expected, actual);
        }
    }
}

mod submission {
    mod deserialize {
        use mini_exercism::api::v1::solution::Submission;

        #[test]
        fn test_all() {
            let json = r#"{
                "submitted_at": "2023-05-07T05:35:43.366Z"
            }"#;

            let expected = Submission { submitted_at: "2023-05-07T05:35:43.366Z".into() };
            let actual = serde_json::from_str(json).unwrap();
            assert_eq!(expected, actual);
        }
    }
}
