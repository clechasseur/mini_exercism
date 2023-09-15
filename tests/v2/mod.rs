mod client {
    use mini_exercism::api;
    use mini_exercism::core::Credentials;
    use reqwest::StatusCode;
    use wiremock::http::Method::Get;
    use wiremock::matchers::{bearer_token, method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    const API_TOKEN: &str = "some_api_token";

    mod get_tracks {
        use assert_matches::assert_matches;
        use mini_exercism::api::v2::TrackStatusFilter::Joined;
        use mini_exercism::api::v2::{Track, TrackFilters, TrackLinks, TracksResponse};

        use super::*;

        #[tokio::test]
        async fn test_get_tracks() {
            let mock_server = MockServer::start().await;

            let tracks_response = TracksResponse {
                tracks: vec![Track {
                    name: "cpp".into(),
                    title: "C++".into(),
                    num_concepts: 14,
                    num_exercises: 73,
                    web_url: "https://exercism.org/tracks/cpp".into(),
                    icon_url: "https://dg8krxphbh767.cloudfront.net/tracks/cpp.svg".into(),
                    tags: vec![
                        "Object-oriented".into(),
                        "Static".into(),
                        "Strong".into(),
                        "Compiled".into(),
                        "Android".into(),
                        "iOS".into(),
                        "Linux".into(),
                        "Mac OSX".into(),
                        "Windows".into(),
                        "Standalone executable".into(),
                        "Backends".into(),
                        "Cross-platform development".into(),
                        "Embedded systems".into(),
                        "Financial systems".into(),
                        "Games".into(),
                        "GUIs".into(),
                        "Mobile".into(),
                        "Robotics".into(),
                        "Scientific calculations".into(),
                    ],
                    links: TrackLinks {
                        self_url: "https://exercism.org/tracks/cpp".into(),
                        exercises: "https://exercism.org/tracks/cpp/exercises".into(),
                        concepts: "https://exercism.org/tracks/cpp/concepts".into(),
                    },
                    is_joined: true,
                    num_learnt_concepts: 0,
                    num_completed_exercises: 1,
                }],
            };
            Mock::given(method(Get))
                .and(path("/tracks"))
                .and(query_param("criteria", "cpp"))
                .and(query_param("tags[]", "Object-oriented"))
                .and(query_param("status", "joined"))
                .and(bearer_token(API_TOKEN))
                .respond_with(ResponseTemplate::new(StatusCode::OK).set_body_json(tracks_response))
                .mount(&mock_server)
                .await;

            let client = api::v2::Client::builder()
                .api_base_url(mock_server.uri().as_str())
                .credentials(Credentials::from_api_token(API_TOKEN))
                .build();
            let filters = TrackFilters::builder()
                .criteria("cpp")
                .tag("Object-oriented")
                .status(Joined)
                .build();
            let track_response = client.get_tracks(Some(filters)).await;
            let tracks = track_response.unwrap().tracks;
            let track = tracks.first();
            assert_matches!(track, Some(track) if track.name == "cpp" && track.title == "C++");
        }
    }

    mod get_exercises {
        use mini_exercism::api::v2::ExerciseDifficulty::Hard;
        use mini_exercism::api::v2::ExerciseType::Practice;
        use mini_exercism::api::v2::SolutionMentoringStatus::Finished;
        use mini_exercism::api::v2::SolutionStatus::Published;
        use mini_exercism::api::v2::SolutionTestsStatus::Passed;
        use mini_exercism::api::v2::{
            Exercise, ExerciseFilters, ExerciseLinks, ExercisesResponse, Solution,
            SolutionExercise, SolutionTrack,
        };

        use super::*;

        #[tokio::test]
        async fn test_get_exercises() {
            let mock_server = MockServer::start().await;

            let exercises_response = ExercisesResponse {
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
                    links: ExerciseLinks { self_path: "/tracks/rust/exercises/poker".into() },
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
                    exercise: SolutionExercise {
                        name: "poker".into(),
                        title: "Poker".into(),
                        icon_url: "https://assets.exercism.org/exercises/poker.svg".into(),
                    },
                    track: SolutionTrack {
                        name: "rust".into(),
                        title: "Rust".into(),
                        icon_url: "https://assets.exercism.org/tracks/rust.svg".into(),
                    },
                }],
            };
            Mock::given(method(Get))
                .and(path("/tracks/rust/exercises"))
                .and(query_param("criteria", "poker"))
                .and(query_param("sideload", "solutions"))
                .and(bearer_token(API_TOKEN))
                .respond_with(
                    ResponseTemplate::new(StatusCode::OK).set_body_json(exercises_response),
                )
                .mount(&mock_server)
                .await;

            let client = api::v2::Client::builder()
                .api_base_url(mock_server.uri().as_str())
                .credentials(Credentials::from_api_token(API_TOKEN))
                .build();
            let filters = ExerciseFilters::builder()
                .criteria("poker")
                .include_solutions(true)
                .build();
            let exercises_response = client.get_exercises("rust", Some(filters)).await;
            let exercises_response = exercises_response.unwrap();
            let exercises = exercises_response.exercises;
            let solutions = exercises_response.solutions;
            assert_eq!(1, exercises.len());
            assert_eq!("poker", exercises.first().unwrap().name);
            assert_eq!(1, solutions.len());
            assert_eq!("00c717b68e1b4213b316df82636f5e0f", solutions.first().unwrap().uuid);
        }
    }
}

mod track_filters {
    mod builder {
        use assert_matches::assert_matches;
        use mini_exercism::api::v2::TrackFilters;
        use mini_exercism::api::v2::TrackStatusFilter::Joined;

        #[test]
        fn test_build() {
            let filters = TrackFilters::builder()
                .criteria("csharp")
                .tag("Functional")
                .tag("Compiled")
                .status(Joined)
                .build();

            let expected_tags = vec!["Functional", "Compiled"];
            assert_matches!(filters.criteria, Some(criteria) if criteria == "csharp");
            assert_eq!(expected_tags, filters.tags);
            assert_matches!(filters.status, Some(status) if status == Joined);
        }
    }

    mod from {
        use mini_exercism::api::v2::TrackFilters;
        use mini_exercism::api::v2::TrackStatusFilter::Unjoined;

        #[test]
        fn test_into_query_params() {
            let filters = TrackFilters::builder()
                .criteria("clojure")
                .tags(vec!["Functional", "Compiled"])
                .status(Unjoined)
                .build();

            let expected = vec![
                ("criteria", "clojure"),
                ("tags[]", "Functional"),
                ("tags[]", "Compiled"),
                ("status", "unjoined"),
            ];
            let actual: Vec<(_, _)> = filters.into();
            assert_eq!(expected, actual);
        }
    }
}

mod tracks_response {
    mod deserialize {
        use mini_exercism::api::v2::{Track, TrackLinks, TracksResponse};

        #[test]
        fn test_all() {
            let json = r#"{
                "tracks": [
                    {
                        "slug": "8th",
                        "title": "8th",
                        "course": false,
                        "num_concepts": 0,
                        "num_exercises": 22,
                        "web_url": "https://exercism.org/tracks/8th",
                        "icon_url": "https://dg8krxphbh767.cloudfront.net/tracks/8th.svg",
                        "tags": [
                            "Functional",
                            "Imperative",
                            "Procedural",
                            "Static",
                            "Strong",
                            "Dynamic",
                            "Compiled",
                            "Interpreted",
                            "Windows",
                            "Mac OSX",
                            "Linux",
                            "iOS",
                            "Android",
                            "Backends",
                            "Cross-platform development",
                            "Frontends",
                            "Games",
                            "GUIs",
                            "Mobile",
                            "Web development"
                        ],
                        "last_touched_at": null,
                        "is_new": false,
                        "links": {
                            "self": "https://exercism.org/tracks/8th",
                            "exercises": "https://exercism.org/tracks/8th/exercises",
                            "concepts": "https://exercism.org/tracks/8th/concepts"
                        }
                    },
                    {
                        "slug": "abap",
                        "title": "ABAP",
                        "course": false,
                        "num_concepts": 0,
                        "num_exercises": 40,
                        "web_url": "https://exercism.org/tracks/abap",
                        "icon_url": "https://dg8krxphbh767.cloudfront.net/tracks/abap.svg",
                        "tags": [
                            "Object-oriented",
                            "Procedural",
                            "Static",
                            "Strong",
                            "Compiled",
                            "Language-specific runtime",
                            "Backends",
                            "Financial systems"
                        ],
                        "last_touched_at": null,
                        "is_new": false,
                        "links": {
                            "self": "https://exercism.org/tracks/abap",
                            "exercises": "https://exercism.org/tracks/abap/exercises",
                            "concepts": "https://exercism.org/tracks/abap/concepts"
                        }
                    }
                ]
            }"#;

            let expected = TracksResponse {
                tracks: vec![
                    Track {
                        name: "8th".into(),
                        title: "8th".into(),
                        num_concepts: 0,
                        num_exercises: 22,
                        web_url: "https://exercism.org/tracks/8th".into(),
                        icon_url: "https://dg8krxphbh767.cloudfront.net/tracks/8th.svg".into(),
                        tags: vec![
                            "Functional".into(),
                            "Imperative".into(),
                            "Procedural".into(),
                            "Static".into(),
                            "Strong".into(),
                            "Dynamic".into(),
                            "Compiled".into(),
                            "Interpreted".into(),
                            "Windows".into(),
                            "Mac OSX".into(),
                            "Linux".into(),
                            "iOS".into(),
                            "Android".into(),
                            "Backends".into(),
                            "Cross-platform development".into(),
                            "Frontends".into(),
                            "Games".into(),
                            "GUIs".into(),
                            "Mobile".into(),
                            "Web development".into(),
                        ],
                        links: TrackLinks {
                            self_url: "https://exercism.org/tracks/8th".into(),
                            exercises: "https://exercism.org/tracks/8th/exercises".into(),
                            concepts: "https://exercism.org/tracks/8th/concepts".into(),
                        },
                        is_joined: false,
                        num_learnt_concepts: 0,
                        num_completed_exercises: 0,
                    },
                    Track {
                        name: "abap".into(),
                        title: "ABAP".into(),
                        num_concepts: 0,
                        num_exercises: 40,
                        web_url: "https://exercism.org/tracks/abap".into(),
                        icon_url: "https://dg8krxphbh767.cloudfront.net/tracks/abap.svg".into(),
                        tags: vec![
                            "Object-oriented".into(),
                            "Procedural".into(),
                            "Static".into(),
                            "Strong".into(),
                            "Compiled".into(),
                            "Language-specific runtime".into(),
                            "Backends".into(),
                            "Financial systems".into(),
                        ],
                        links: TrackLinks {
                            self_url: "https://exercism.org/tracks/abap".into(),
                            exercises: "https://exercism.org/tracks/abap/exercises".into(),
                            concepts: "https://exercism.org/tracks/abap/concepts".into(),
                        },
                        is_joined: false,
                        num_learnt_concepts: 0,
                        num_completed_exercises: 0,
                    },
                ],
            };
            let actual: TracksResponse = serde_json::from_str(json).unwrap();
            assert_eq!(expected, actual);
        }
    }
}

mod track {
    mod deserialize {
        use mini_exercism::api::v2::{Track, TrackLinks};

        #[test]
        fn test_anonymous() {
            let json = r#"{
                "slug": "clojure",
                "title": "Clojure",
                "course": true,
                "num_concepts": 10,
                "num_exercises": 87,
                "web_url": "https://exercism.org/tracks/clojure",
                "icon_url": "https://dg8krxphbh767.cloudfront.net/tracks/clojure.svg",
                "tags": [
                    "Declarative",
                    "Functional",
                    "Dynamic",
                    "Compiled",
                    "Windows",
                    "Mac OSX",
                    "Linux",
                    "JVM (Java)",
                    "Artificial Intelligence",
                    "Backends",
                    "Cross-platform development",
                    "Financial systems",
                    "Frontends",
                    "Games",
                    "GUIs",
                    "Robotics",
                    "Scientific calculations",
                    "Web development"
                ],
                "last_touched_at": null,
                "is_new": false,
                "links": {
                    "self": "https://exercism.org/tracks/clojure",
                    "exercises": "https://exercism.org/tracks/clojure/exercises",
                    "concepts": "https://exercism.org/tracks/clojure/concepts"
                }
            }"#;

            let expected = Track {
                name: "clojure".into(),
                title: "Clojure".into(),
                num_concepts: 10,
                num_exercises: 87,
                web_url: "https://exercism.org/tracks/clojure".into(),
                icon_url: "https://dg8krxphbh767.cloudfront.net/tracks/clojure.svg".into(),
                tags: vec![
                    "Declarative".into(),
                    "Functional".into(),
                    "Dynamic".into(),
                    "Compiled".into(),
                    "Windows".into(),
                    "Mac OSX".into(),
                    "Linux".into(),
                    "JVM (Java)".into(),
                    "Artificial Intelligence".into(),
                    "Backends".into(),
                    "Cross-platform development".into(),
                    "Financial systems".into(),
                    "Frontends".into(),
                    "Games".into(),
                    "GUIs".into(),
                    "Robotics".into(),
                    "Scientific calculations".into(),
                    "Web development".into(),
                ],
                links: TrackLinks {
                    self_url: "https://exercism.org/tracks/clojure".into(),
                    exercises: "https://exercism.org/tracks/clojure/exercises".into(),
                    concepts: "https://exercism.org/tracks/clojure/concepts".into(),
                },
                is_joined: false,
                num_learnt_concepts: 0,
                num_completed_exercises: 0,
            };
            let actual: Track = serde_json::from_str(json).unwrap();
            assert_eq!(expected, actual);
        }

        #[test]
        fn test_authenticated() {
            let json = r#"{
                "slug": "cpp",
                "title": "C++",
                "course": true,
                "num_concepts": 14,
                "num_exercises": 73,
                "web_url": "https://exercism.org/tracks/cpp",
                "icon_url": "https://dg8krxphbh767.cloudfront.net/tracks/cpp.svg",
                "tags": [
                    "Object-oriented",
                    "Static",
                    "Strong",
                    "Compiled",
                    "Android",
                    "iOS",
                    "Linux",
                    "Mac OSX",
                    "Windows",
                    "Standalone executable",
                    "Backends",
                    "Cross-platform development",
                    "Embedded systems",
                    "Financial systems",
                    "Games",
                    "GUIs",
                    "Mobile",
                    "Robotics",
                    "Scientific calculations"
                ],
                "last_touched_at": "2023-07-15T06:12:39Z",
                "is_new": false,
                "links": {
                    "self": "https://exercism.org/tracks/cpp",
                    "exercises": "https://exercism.org/tracks/cpp/exercises",
                    "concepts": "https://exercism.org/tracks/cpp/concepts"
                },
                "is_joined": true,
                "num_learnt_concepts": 0,
                "num_completed_exercises": 1,
                "num_solutions": 2,
                "has_notifications": false
            }"#;

            let expected = Track {
                name: "cpp".into(),
                title: "C++".into(),
                num_concepts: 14,
                num_exercises: 73,
                web_url: "https://exercism.org/tracks/cpp".into(),
                icon_url: "https://dg8krxphbh767.cloudfront.net/tracks/cpp.svg".into(),
                tags: vec![
                    "Object-oriented".into(),
                    "Static".into(),
                    "Strong".into(),
                    "Compiled".into(),
                    "Android".into(),
                    "iOS".into(),
                    "Linux".into(),
                    "Mac OSX".into(),
                    "Windows".into(),
                    "Standalone executable".into(),
                    "Backends".into(),
                    "Cross-platform development".into(),
                    "Embedded systems".into(),
                    "Financial systems".into(),
                    "Games".into(),
                    "GUIs".into(),
                    "Mobile".into(),
                    "Robotics".into(),
                    "Scientific calculations".into(),
                ],
                links: TrackLinks {
                    self_url: "https://exercism.org/tracks/cpp".into(),
                    exercises: "https://exercism.org/tracks/cpp/exercises".into(),
                    concepts: "https://exercism.org/tracks/cpp/concepts".into(),
                },
                is_joined: true,
                num_learnt_concepts: 0,
                num_completed_exercises: 1,
            };
            let actual: Track = serde_json::from_str(json).unwrap();
            assert_eq!(expected, actual);
        }
    }
}

mod track_links {
    mod deserialize {
        use mini_exercism::api::v2::TrackLinks;

        #[test]
        fn test_all() {
            let json = r#"{
                "self": "https://exercism.org/tracks/cpp",
                "exercises": "https://exercism.org/tracks/cpp/exercises",
                "concepts": "https://exercism.org/tracks/cpp/concepts"
            }"#;

            let expected = TrackLinks {
                self_url: "https://exercism.org/tracks/cpp".into(),
                exercises: "https://exercism.org/tracks/cpp/exercises".into(),
                concepts: "https://exercism.org/tracks/cpp/concepts".into(),
            };
            let actual: TrackLinks = serde_json::from_str(json).unwrap();
            assert_eq!(expected, actual);
        }
    }
}

mod exercise_filters {
    mod builder {
        use assert_matches::assert_matches;
        use mini_exercism::api::v2::ExerciseFilters;

        #[test]
        fn test_build() {
            let filters = ExerciseFilters::builder()
                .criteria("csharp")
                .include_solutions(true)
                .build();

            assert_matches!(filters.criteria, Some(criteria) if criteria == "csharp");
            assert!(filters.include_solutions);
        }
    }

    mod from {
        use mini_exercism::api::v2::ExerciseFilters;

        #[test]
        fn test_into_query_params() {
            let filters = ExerciseFilters::builder()
                .criteria("clojure")
                .include_solutions(true)
                .build();

            let expected = vec![("criteria", "clojure"), ("sideload", "solutions")];
            let actual: Vec<(_, _)> = filters.into();
            assert_eq!(expected, actual);
        }
    }
}

mod exercises_response {
    mod deserialize {
        use mini_exercism::api::v2::ExerciseDifficulty::{Easy, Hard};
        use mini_exercism::api::v2::ExerciseType::{Practice, Tutorial};
        use mini_exercism::api::v2::SolutionMentoringStatus::Finished;
        use mini_exercism::api::v2::SolutionStatus::Published;
        use mini_exercism::api::v2::SolutionTestsStatus::Passed;
        use mini_exercism::api::v2::{
            Exercise, ExerciseLinks, ExercisesResponse, Solution, SolutionExercise, SolutionTrack,
        };

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

            let expected = ExercisesResponse {
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
                        links: ExerciseLinks {
                            self_path: "/tracks/rust/exercises/hello-world".into(),
                        },
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
                        links: ExerciseLinks { self_path: "/tracks/rust/exercises/forth".into() },
                    },
                ],
                solutions: vec![],
            };
            let actual: ExercisesResponse = serde_json::from_str(json).unwrap();
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

            let expected = ExercisesResponse {
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
                    links: ExerciseLinks { self_path: "/tracks/rust/exercises/poker".into() },
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
                    exercise: SolutionExercise {
                        name: "poker".into(),
                        title: "Poker".into(),
                        icon_url: "https://assets.exercism.org/exercises/poker.svg".into(),
                    },
                    track: SolutionTrack {
                        name: "rust".into(),
                        title: "Rust".into(),
                        icon_url: "https://assets.exercism.org/tracks/rust.svg".into(),
                    },
                }],
            };
            let actual: ExercisesResponse = serde_json::from_str(json).unwrap();
            assert_eq!(expected, actual);
        }
    }
}

mod exercise {
    mod deserialize {
        use mini_exercism::api::v2::ExerciseDifficulty::Easy;
        use mini_exercism::api::v2::ExerciseType::Tutorial;
        use mini_exercism::api::v2::{Exercise, ExerciseDifficulty, ExerciseLinks, ExerciseType};

        #[test]
        fn test_all() {
            let json = r#"{
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
            }"#;

            let expected = Exercise {
                name: "hello-world".into(),
                exercise_type: Tutorial,
                title: "Hello World".into(),
                icon_url: "https://assets.exercism.org/exercises/hello-world.svg".into(),
                difficulty: Easy,
                blurb: "The classical introductory exercise. Just say \"Hello, World!\".".into(),
                is_external: true,
                is_unlocked: true,
                is_recommended: false,
                links: ExerciseLinks { self_path: "/tracks/rust/exercises/hello-world".into() },
            };
            let actual: Exercise = serde_json::from_str(json).unwrap();
            assert_eq!(expected, actual);
        }

        #[test]
        fn test_unknown() {
            let json = r#"{
                "slug": "rlyehian",
                "type": "mglw'nafh",
                "title": "R'lyehian",
                "icon_url": "https://assets.exercism.org/exercises/rlyehian.svg",
                "difficulty": "syha'h",
                "blurb": "Cahf ah nafl mglw'nafh hh' ahor syha'h ah'legeth, ng llll or'azath syha'hnahh n'ghftephai n'gha ahornah ah'mglw'nafh.",
                "is_external": true,
                "is_unlocked": true,
                "is_recommended": false,
                "links": {
                    "self": "/tracks/rust/exercises/rlyehian"
                }
            }"#;

            let expected = Exercise {
                name: "rlyehian".into(),
                exercise_type: ExerciseType::Unknown,
                title: "R'lyehian".into(),
                icon_url: "https://assets.exercism.org/exercises/rlyehian.svg"
                    .into(),
                difficulty: ExerciseDifficulty::Unknown,
                blurb: "Cahf ah nafl mglw'nafh hh' ahor syha'h ah'legeth, ng llll or'azath syha'hnahh n'ghftephai n'gha ahornah ah'mglw'nafh."
                    .into(),
                is_external: true,
                is_unlocked: true,
                is_recommended: false,
                links: ExerciseLinks {
                    self_path: "/tracks/rust/exercises/rlyehian".into(),
                },
            };
            let actual: Exercise = serde_json::from_str(json).unwrap();
            assert_eq!(expected, actual);
        }
    }
}

mod exercise_links {
    mod deserialize {
        use mini_exercism::api::v2::ExerciseLinks;

        #[test]
        fn test_all() {
            let json = r#"{
                "self": "/tracks/rust/exercises/hello-world"
            }"#;

            let expected = ExerciseLinks { self_path: "/tracks/rust/exercises/hello-world".into() };
            let actual: ExerciseLinks = serde_json::from_str(json).unwrap();
            assert_eq!(expected, actual);
        }
    }
}

mod solution {
    mod deserialize {
        use mini_exercism::api::v2::SolutionMentoringStatus::Finished;
        use mini_exercism::api::v2::SolutionStatus::Published;
        use mini_exercism::api::v2::SolutionTestsStatus::Passed;
        use mini_exercism::api::v2::{
            Solution, SolutionExercise, SolutionMentoringStatus, SolutionStatus,
            SolutionTestsStatus, SolutionTrack,
        };

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
                exercise: SolutionExercise {
                    name: "poker".into(),
                    title: "Poker".into(),
                    icon_url: "https://assets.exercism.org/exercises/poker.svg".into(),
                },
                track: SolutionTrack {
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
                status: SolutionStatus::Unknown,
                mentoring_status: SolutionMentoringStatus::Unknown,
                published_iteration_head_tests_status: SolutionTestsStatus::Unknown,
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
                exercise: SolutionExercise {
                    name: "rlyehian".into(),
                    title: "R'lyehian".into(),
                    icon_url: "https://assets.exercism.org/exercises/rlyehian.svg".into(),
                },
                track: SolutionTrack {
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

mod solution_exercise {
    mod deserialize {
        use mini_exercism::api::v2::SolutionExercise;

        #[test]
        fn test_all() {
            let json = r#"{
                "slug": "poker",
                "title": "Poker",
                "icon_url": "https://assets.exercism.org/exercises/poker.svg"
            }"#;

            let expected = SolutionExercise {
                name: "poker".into(),
                title: "Poker".into(),
                icon_url: "https://assets.exercism.org/exercises/poker.svg".into(),
            };
            let actual: SolutionExercise = serde_json::from_str(json).unwrap();
            assert_eq!(expected, actual);
        }
    }
}

mod solution_track {
    mod deserialize {
        use mini_exercism::api::v2::SolutionTrack;

        #[test]
        fn test_all() {
            let json = r#"{
                "slug": "rust",
                "title": "Rust",
                "icon_url": "https://assets.exercism.org/tracks/rust.svg"
            }"#;

            let expected = SolutionTrack {
                name: "rust".into(),
                title: "Rust".into(),
                icon_url: "https://assets.exercism.org/tracks/rust.svg".into(),
            };
            let actual: SolutionTrack = serde_json::from_str(json).unwrap();
            assert_eq!(expected, actual);
        }
    }
}
