mod filters_tests {
    mod builder {
        use assert_matches::assert_matches;
        use mini_exercism::api::v2::exercises::Filters;

        #[test]
        fn test_build() {
            let filters = Filters::builder()
                .criteria("csharp")
                .include_solutions(true)
                .build();

            assert_matches!(filters.criteria, Some(criteria) if criteria == "csharp");
            assert!(filters.include_solutions);
        }
    }
}

mod response_tests {
    mod deserialize {
        use mini_exercism::api::v2::exercise::Difficulty::{Easy, Hard};
        use mini_exercism::api::v2::exercise::Type::{Practice, Tutorial};
        use mini_exercism::api::v2::exercise::{Exercise, Links};
        use mini_exercism::api::v2::solution::MentoringStatus::Finished;
        use mini_exercism::api::v2::solution::Solution;
        use mini_exercism::api::v2::solution::Status::Published;
        use mini_exercism::api::v2::solution::TestsStatus::Passed;
        use mini_exercism::api::v2::{exercises, solution};

        #[test]
        fn test_anonymous() {
            let json = r#"{
                "exercises": [
                    {
                        "slug": "hello-world",
                        "type": "tutorial",
                        "title": "Hello World",
                        "icon_url": "https://assets.exercism.org/exercises/hello-world.svg",
                        "difficulty": "easy",
                        "blurb": "The classical introductory exercise. Just say \"Hello, World!\".",
                        "is_external": true,
                        "is_unlocked": true,
                        "is_recommended": false,
                        "links": {
                            "self": "/tracks/rust/exercises/hello-world"
                        }
                    },
                    {
                        "slug": "forth",
                        "type": "practice",
                        "title": "Forth",
                        "icon_url": "https://assets.exercism.org/exercises/forth.svg",
                        "difficulty": "hard",
                        "blurb": "Implement an evaluator for a very simple subset of Forth.",
                        "is_external": true,
                        "is_unlocked": true,
                        "is_recommended": false,
                        "links": {
                            "self": "/tracks/rust/exercises/forth"
                        }
                    }
                ]
            }"#;

            let expected = exercises::Response {
                exercises: vec![
                    Exercise {
                        name: "hello-world".into(),
                        exercise_type: Tutorial,
                        title: "Hello World".into(),
                        icon_url: "https://assets.exercism.org/exercises/hello-world.svg".into(),
                        difficulty: Easy,
                        blurb: "The classical introductory exercise. Just say \"Hello, World!\"."
                            .into(),
                        is_external: true,
                        is_unlocked: true,
                        is_recommended: false,
                        links: Links { self_path: "/tracks/rust/exercises/hello-world".into() },
                    },
                    Exercise {
                        name: "forth".into(),
                        exercise_type: Practice,
                        title: "Forth".into(),
                        icon_url: "https://assets.exercism.org/exercises/forth.svg".into(),
                        difficulty: Hard,
                        blurb: "Implement an evaluator for a very simple subset of Forth.".into(),
                        is_external: true,
                        is_unlocked: true,
                        is_recommended: false,
                        links: Links { self_path: "/tracks/rust/exercises/forth".into() },
                    },
                ],
                solutions: vec![],
            };
            let actual: exercises::Response = serde_json::from_str(json).unwrap();
            assert_eq!(expected, actual);
        }

        #[test]
        fn test_authenticated() {
            let json = r#"{
                "exercises": [
                    {
                        "slug": "poker",
                        "type": "practice",
                        "title": "Poker",
                        "icon_url": "https://assets.exercism.org/exercises/poker.svg",
                        "difficulty": "hard",
                        "blurb": "Pick the best hand(s) from a list of poker hands.",
                        "is_external": false,
                        "is_unlocked": true,
                        "is_recommended": false,
                        "links": {
                            "self": "/tracks/rust/exercises/poker"
                        }
                    }
                ],
                "solutions": [
                    {
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
                    }
                ]
            }"#;

            let expected = exercises::Response {
                exercises: vec![Exercise {
                    name: "poker".into(),
                    exercise_type: Practice,
                    title: "Poker".into(),
                    icon_url: "https://assets.exercism.org/exercises/poker.svg".into(),
                    difficulty: Hard,
                    blurb: "Pick the best hand(s) from a list of poker hands.".into(),
                    is_external: false,
                    is_unlocked: true,
                    is_recommended: false,
                    links: Links { self_path: "/tracks/rust/exercises/poker".into() },
                }],
                solutions: vec![Solution {
                    uuid: "00c717b68e1b4213b316df82636f5e0f".into(),
                    private_url: "https://exercism.org/tracks/rust/exercises/poker".into(),
                    public_url:
                        "https://exercism.org/tracks/rust/exercises/poker/solutions/clechasseur"
                            .into(),
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
                    exercise: solution::Exercise {
                        name: "poker".into(),
                        title: "Poker".into(),
                        icon_url: "https://assets.exercism.org/exercises/poker.svg".into(),
                    },
                    track: solution::Track {
                        name: "rust".into(),
                        title: "Rust".into(),
                        icon_url: "https://assets.exercism.org/tracks/rust.svg".into(),
                    },
                }],
            };
            let actual: exercises::Response = serde_json::from_str(json).unwrap();
            assert_eq!(expected, actual);
        }
    }
}
