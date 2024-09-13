mod response {
    mod deserialize {
        use mini_exercism::api::v2::iteration::Status::NonActionableAutomatedFeedback;
        use mini_exercism::api::v2::iteration::{Iteration, Links};
        use mini_exercism::api::v2::solution;
        use mini_exercism::api::v2::solution::Status::Published;
        use mini_exercism::api::v2::solution::{Exercise, MentoringStatus, Solution, Track};
        use mini_exercism::api::v2::tests::Status::Passed;

        #[test]
        fn test_all() {
            let json = r#"{
                "solution": {
                    "uuid": "a0c9664059d345ac8d677b0154794ff2",
                    "private_url": "https://exercism.org/tracks/rust/exercises/clock",
                    "public_url": "https://exercism.org/tracks/rust/exercises/clock/solutions/clechasseur",
                    "status": "published",
                    "mentoring_status": "none",
                    "published_iteration_head_tests_status": "passed",
                    "has_notifications": false,
                    "num_views": 0,
                    "num_stars": 0,
                    "num_comments": 0,
                    "num_iterations": 2,
                    "num_loc": 28,
                    "is_out_of_date": false,
                    "published_at": "2023-03-26T05:22:57Z",
                    "completed_at": "2023-03-26T05:22:57Z",
                    "updated_at": "2023-12-06T12:48:07Z",
                    "last_iterated_at": "2023-03-26T05:22:23Z",
                    "exercise": {
                        "slug": "clock",
                        "title": "Clock",
                        "icon_url": "https://assets.exercism.org/exercises/clock.svg"
                    },
                    "track": {
                        "slug": "rust",
                        "title": "Rust",
                        "icon_url": "https://assets.exercism.org/tracks/rust.svg"
                    }
                },
                "iterations": [
                    {
                        "uuid": "98f8b04515a8484ca211edc7c56d2aa2",
                        "submission_uuid": "ab542af6906349ebb37e7cbee4828554",
                        "idx": 1,
                        "status": "non_actionable_automated_feedback",
                        "num_essential_automated_comments": 0,
                        "num_actionable_automated_comments": 0,
                        "num_non_actionable_automated_comments": 3,
                        "num_celebratory_automated_comments": 0,
                        "submission_method": "cli",
                        "created_at": "2023-03-26T05:22:23Z",
                        "tests_status": "passed",
                        "is_published": true,
                        "is_latest": true,
                        "links": {
                            "self": "https://exercism.org/tracks/rust/exercises/clock/iterations?idx=2",
                            "automated_feedback": "https://exercism.org/api/v2/solutions/a0c9664059d345ac8d677b0154794ff2/iterations/98f8b04515a8484ca211edc7c56d2aa2/automated_feedback",
                            "delete": "https://exercism.org/api/v2/solutions/a0c9664059d345ac8d677b0154794ff2/iterations/98f8b04515a8484ca211edc7c56d2aa2",
                            "solution": "https://exercism.org/tracks/rust/exercises/clock",
                            "test_run": "https://exercism.org/api/v2/solutions/a0c9664059d345ac8d677b0154794ff2/submissions/ab542af6906349ebb37e7cbee4828554/test_run",
                            "files": "https://exercism.org/api/v2/solutions/a0c9664059d345ac8d677b0154794ff2/submissions/ab542af6906349ebb37e7cbee4828554/files"
                        }
                    }
                ]
            }"#;

            let expected = solution::Response {
                solution: Solution {
                    uuid: "a0c9664059d345ac8d677b0154794ff2".into(),
                    private_url: "https://exercism.org/tracks/rust/exercises/clock".into(),
                    public_url: "https://exercism.org/tracks/rust/exercises/clock/solutions/clechasseur".into(),
                    status: Published,
                    mentoring_status: MentoringStatus::None,
                    published_iteration_head_tests_status: Passed,
                    has_notifications: false,
                    num_views: 0,
                    num_stars: 0,
                    num_comments: 0,
                    num_iterations: 2,
                    num_loc: Some(28),
                    is_out_of_date: false,
                    published_at: Some("2023-03-26T05:22:57Z".into()),
                    completed_at: Some("2023-03-26T05:22:57Z".into()),
                    updated_at: "2023-12-06T12:48:07Z".into(),
                    last_iterated_at: Some("2023-03-26T05:22:23Z".into()),
                    exercise: Exercise {
                        name: "clock".into(),
                        title: "Clock".into(),
                        icon_url: "https://assets.exercism.org/exercises/clock.svg".into(),
                    },
                    track: Track {
                        name: "rust".into(),
                        title: "Rust".into(),
                        icon_url: "https://assets.exercism.org/tracks/rust.svg".into(),
                    }
                },
                iterations: vec![
                    Iteration {
                        uuid: "98f8b04515a8484ca211edc7c56d2aa2".into(),
                        submission_uuid: Some("ab542af6906349ebb37e7cbee4828554".into()),
                        index: 1,
                        status: NonActionableAutomatedFeedback,
                        num_essential_automated_comments: 0,
                        num_actionable_automated_comments: 0,
                        num_non_actionable_automated_comments: 3,
                        num_celebratory_automated_comments: 0,
                        submission_method: "cli".into(),
                        created_at: "2023-03-26T05:22:23Z".into(),
                        tests_status: Passed,
                        representer_feedback: None,
                        analyzer_feedback: None,
                        is_published: true,
                        is_latest: true,
                        files: vec![],
                        links: Links {
                            self_path: "https://exercism.org/tracks/rust/exercises/clock/iterations?idx=2".into(),
                            automated_feedback: Some("https://exercism.org/api/v2/solutions/a0c9664059d345ac8d677b0154794ff2/iterations/98f8b04515a8484ca211edc7c56d2aa2/automated_feedback".into()),
                            delete: Some("https://exercism.org/api/v2/solutions/a0c9664059d345ac8d677b0154794ff2/iterations/98f8b04515a8484ca211edc7c56d2aa2".into()),
                            solution: "https://exercism.org/tracks/rust/exercises/clock".into(),
                            test_run: Some("https://exercism.org/api/v2/solutions/a0c9664059d345ac8d677b0154794ff2/submissions/ab542af6906349ebb37e7cbee4828554/test_run".into()),
                            files: Some("https://exercism.org/api/v2/solutions/a0c9664059d345ac8d677b0154794ff2/submissions/ab542af6906349ebb37e7cbee4828554/files".into()),
                        },
                    },
                ],
            };
            let actual: solution::Response = serde_json::from_str(json).unwrap();
            assert_eq!(expected, actual);
        }
    }
}

#[allow(clippy::module_inception)]
mod solution {
    mod deserialize {
        use mini_exercism::api::v2::solution::MentoringStatus::Finished;
        use mini_exercism::api::v2::solution::Status::Published;
        use mini_exercism::api::v2::solution::{
            Exercise, MentoringStatus, Solution, Status, Track,
        };
        use mini_exercism::api::v2::tests;
        use mini_exercism::api::v2::tests::Status::Passed;

        #[test]
        fn test_all() {
            let json = r#"{
                "uuid": "00c717b68e1b4213b316df82636f5e0f",
                "private_url": "https://exercism.org/tracks/rust/exercises/poker",
                "public_url": "https://exercism.org/tracks/rust/exercises/poker/solutions/clechasseur",
                "status": "published",
                "mentoring_status": "finished",
                "published_iteration_head_tests_status": "passed",
                "has_notifications": false,
                "num_views": 0,
                "num_stars": 0,
                "num_comments": 0,
                "num_iterations": 13,
                "num_loc": 252,
                "is_out_of_date": false,
                "published_at": "2023-05-08T00:02:21Z",
                "completed_at": "2023-05-08T00:02:21Z",
                "updated_at": "2023-08-27T07:06:01Z",
                "last_iterated_at": "2023-05-07T05:35:43Z",
                "exercise": {
                    "slug": "poker",
                    "title": "Poker",
                    "icon_url": "https://assets.exercism.org/exercises/poker.svg"
                },
                "track": {
                    "slug": "rust",
                    "title": "Rust",
                    "icon_url": "https://assets.exercism.org/tracks/rust.svg"
                }
            }"#;

            let expected = Solution {
                uuid: "00c717b68e1b4213b316df82636f5e0f".into(),
                private_url: "https://exercism.org/tracks/rust/exercises/poker".into(),
                public_url:
                    "https://exercism.org/tracks/rust/exercises/poker/solutions/clechasseur".into(),
                status: Published,
                mentoring_status: Finished,
                published_iteration_head_tests_status: Passed,
                has_notifications: false,
                num_views: 0,
                num_stars: 0,
                num_comments: 0,
                num_iterations: 13,
                num_loc: Some(252),
                is_out_of_date: false,
                published_at: Some("2023-05-08T00:02:21Z".into()),
                completed_at: Some("2023-05-08T00:02:21Z".into()),
                updated_at: "2023-08-27T07:06:01Z".into(),
                last_iterated_at: Some("2023-05-07T05:35:43Z".into()),
                exercise: Exercise {
                    name: "poker".into(),
                    title: "Poker".into(),
                    icon_url: "https://assets.exercism.org/exercises/poker.svg".into(),
                },
                track: Track {
                    name: "rust".into(),
                    title: "Rust".into(),
                    icon_url: "https://assets.exercism.org/tracks/rust.svg".into(),
                },
            };
            let actual: Solution = serde_json::from_str(json).unwrap();
            assert_eq!(expected, actual);
        }

        #[test]
        fn test_unknown() {
            let json = r#"{
                "uuid": "7b1fe9e73ccf44d5aa4da9b3d28da405",
                "private_url": "https://exercism.org/tracks/rust/exercises/rlyehian",
                "public_url": "https://exercism.org/tracks/rust/exercises/rlyehian/solutions/clechasseur",
                "status": "syha'hnahh",
                "mentoring_status": "or'azath",
                "published_iteration_head_tests_status": "ah'mglw'nafh",
                "has_notifications": false,
                "num_views": 0,
                "num_stars": 0,
                "num_comments": 0,
                "num_iterations": 13,
                "num_loc": 252,
                "is_out_of_date": false,
                "published_at": "2023-05-08T00:02:21Z",
                "completed_at": "2023-05-08T00:02:21Z",
                "updated_at": "2023-08-27T07:06:01Z",
                "last_iterated_at": "2023-05-07T05:35:43Z",
                "exercise": {
                    "slug": "rlyehian",
                    "title": "R'lyehian",
                    "icon_url": "https://assets.exercism.org/exercises/rlyehian.svg"
                },
                "track": {
                    "slug": "rust",
                    "title": "Rust",
                    "icon_url": "https://assets.exercism.org/tracks/rust.svg"
                }
            }"#;

            let expected = Solution {
                uuid: "7b1fe9e73ccf44d5aa4da9b3d28da405".into(),
                private_url: "https://exercism.org/tracks/rust/exercises/rlyehian".into(),
                public_url:
                    "https://exercism.org/tracks/rust/exercises/rlyehian/solutions/clechasseur"
                        .into(),
                status: Status::Unknown,
                mentoring_status: MentoringStatus::Unknown,
                published_iteration_head_tests_status: tests::Status::Unknown,
                has_notifications: false,
                num_views: 0,
                num_stars: 0,
                num_comments: 0,
                num_iterations: 13,
                num_loc: Some(252),
                is_out_of_date: false,
                published_at: Some("2023-05-08T00:02:21Z".into()),
                completed_at: Some("2023-05-08T00:02:21Z".into()),
                updated_at: "2023-08-27T07:06:01Z".into(),
                last_iterated_at: Some("2023-05-07T05:35:43Z".into()),
                exercise: Exercise {
                    name: "rlyehian".into(),
                    title: "R'lyehian".into(),
                    icon_url: "https://assets.exercism.org/exercises/rlyehian.svg".into(),
                },
                track: Track {
                    name: "rust".into(),
                    title: "Rust".into(),
                    icon_url: "https://assets.exercism.org/tracks/rust.svg".into(),
                },
            };
            let actual: Solution = serde_json::from_str(json).unwrap();
            assert_eq!(expected, actual);
        }
    }
}

mod exercise {
    mod deserialize {
        use mini_exercism::api::v2::solution::Exercise;

        #[test]
        fn test_all() {
            let json = r#"{
                "slug": "poker",
                "title": "Poker",
                "icon_url": "https://assets.exercism.org/exercises/poker.svg"
            }"#;

            let expected = Exercise {
                name: "poker".into(),
                title: "Poker".into(),
                icon_url: "https://assets.exercism.org/exercises/poker.svg".into(),
            };
            let actual: Exercise = serde_json::from_str(json).unwrap();
            assert_eq!(expected, actual);
        }
    }
}

mod track {
    mod deserialize {
        use mini_exercism::api::v2::solution::Track;

        #[test]
        fn test_all() {
            let json = r#"{
                "slug": "rust",
                "title": "Rust",
                "icon_url": "https://assets.exercism.org/tracks/rust.svg"
            }"#;

            let expected = Track {
                name: "rust".into(),
                title: "Rust".into(),
                icon_url: "https://assets.exercism.org/tracks/rust.svg".into(),
            };
            let actual: Track = serde_json::from_str(json).unwrap();
            assert_eq!(expected, actual);
        }
    }
}
