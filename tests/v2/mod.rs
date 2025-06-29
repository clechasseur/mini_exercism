mod exercise;
mod exercises;
mod iteration;
mod solution;
mod solutions;
mod submission;
mod track;
mod tracks;

mod client {
    use mini_exercism::api;
    use mini_exercism::core::Credentials;
    use mini_exercism::http;
    use wiremock::matchers::{bearer_token, method, path, query_param, query_param_is_missing};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    const API_TOKEN: &str = "some_api_token";

    mod debug {
        use super::*;

        #[test]
        fn test_derive() {
            // Note: this test is necessary because of a bug in cargo-tarpaulin, see
            // https://github.com/xd009642/tarpaulin/issues/351#issuecomment-1722148936
            let client = api::v2::Client::new();
            assert!(!format!("{client:?}").is_empty());
        }
    }

    mod get_tracks {
        use assert_matches::assert_matches;
        use mini_exercism::api::v2::track::{Links, Track};
        use mini_exercism::api::v2::tracks;
        use mini_exercism::api::v2::tracks::Filters;
        use mini_exercism::api::v2::tracks::StatusFilter::Joined;

        use super::*;

        #[tokio::test]
        async fn test_get_tracks() {
            let mock_server = MockServer::start().await;

            let tracks_response = tracks::Response {
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
                    links: Links {
                        self_url: "https://exercism.org/tracks/cpp".into(),
                        exercises: "https://exercism.org/tracks/cpp/exercises".into(),
                        concepts: "https://exercism.org/tracks/cpp/concepts".into(),
                    },
                    is_joined: true,
                    num_learnt_concepts: 0,
                    num_completed_exercises: 1,
                }],
            };
            Mock::given(method(http::Method::GET))
                .and(path("/tracks"))
                .and(query_param("criteria", "cpp"))
                .and(query_param("tags[]", "Object-oriented"))
                .and(query_param("status", "joined"))
                .and(bearer_token(API_TOKEN))
                .respond_with(
                    ResponseTemplate::new(http::StatusCode::OK).set_body_json(tracks_response),
                )
                .mount(&mock_server)
                .await;

            let client = api::v2::Client::builder()
                .api_base_url(mock_server.uri().as_str())
                .credentials(Credentials::from_api_token(API_TOKEN))
                .build()
                .unwrap();
            let filters = Filters::builder()
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
        use mini_exercism::api::v2::exercise::Difficulty::Hard;
        use mini_exercism::api::v2::exercise::Type::Practice;
        use mini_exercism::api::v2::exercise::{Exercise, Links};
        use mini_exercism::api::v2::exercises::Filters;
        use mini_exercism::api::v2::solution::MentoringStatus::Finished;
        use mini_exercism::api::v2::solution::Solution;
        use mini_exercism::api::v2::solution::Status::Published;
        use mini_exercism::api::v2::tests::Status::Passed;
        use mini_exercism::api::v2::{exercises, solution};

        use super::*;

        #[tokio::test]
        async fn test_get_exercises() {
            let mock_server = MockServer::start().await;

            let exercises_response = exercises::Response {
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
            Mock::given(method(http::Method::GET))
                .and(path("/tracks/rust/exercises"))
                .and(query_param("criteria", "poker"))
                .and(query_param("sideload", "solutions"))
                .and(bearer_token(API_TOKEN))
                .respond_with(
                    ResponseTemplate::new(http::StatusCode::OK).set_body_json(exercises_response),
                )
                .mount(&mock_server)
                .await;

            let client = api::v2::Client::builder()
                .api_base_url(mock_server.uri().as_str())
                .credentials(Credentials::from_api_token(API_TOKEN))
                .build()
                .unwrap();
            let filters = Filters::builder()
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

    mod get_solutions {
        use mini_exercism::api::v2::solution::Status::Published;
        use mini_exercism::api::v2::solution::{Exercise, MentoringStatus, Solution, Track};
        use mini_exercism::api::v2::solutions;
        use mini_exercism::api::v2::solutions::SortOrder::NewestFirst;
        use mini_exercism::api::v2::solutions::{Filters, Paging, ResponseMeta};
        use mini_exercism::api::v2::tests::Status::{Failed, Passed};

        use super::*;

        #[tokio::test]
        async fn test_get_solutions() {
            let mock_server = MockServer::start().await;

            let solutions_response = solutions::Response {
                results: vec![Solution {
                    uuid: "82f1ce4b47514db29f4831a0d2680ebd".into(),
                    private_url: "https://exercism.org/tracks/javascript/exercises/resistor-color-duo".into(),
                    public_url: "https://exercism.org/tracks/javascript/exercises/resistor-color-duo/solutions/clechasseur".into(),
                    status: Published,
                    mentoring_status: MentoringStatus::None,
                    published_iteration_head_tests_status: Failed,
                    has_notifications: false,
                    num_views: 0,
                    num_stars: 0,
                    num_comments: 0,
                    num_iterations: 1,
                    num_loc: Some(5),
                    is_out_of_date: true,
                    published_at: Some("2019-05-25T22:17:35Z".into()),
                    completed_at: Some("2019-05-25T22:17:32Z".into()),
                    updated_at: "2023-11-18T06:22:04Z".into(),
                    last_iterated_at: None,
                    exercise: Exercise {
                        name: "resistor-color-duo".into(),
                        title: "Resistor Color Duo".into(),
                        icon_url: "https://assets.exercism.org/exercises/resistor-color-duo.svg".into(),
                    },
                    track: Track {
                        name: "javascript".into(),
                        title: "JavaScript".into(),
                        icon_url: "https://assets.exercism.org/tracks/javascript.svg".into(),
                    },
                }, Solution {
                    uuid: "85bcc0c08a134bde8afcb16d062ad6b0".into(),
                    private_url: "https://exercism.org/tracks/javascript/exercises/resistor-color".into(),
                    public_url: "https://exercism.org/tracks/javascript/exercises/resistor-color/solutions/clechasseur".into(),
                    status: Published,
                    mentoring_status: MentoringStatus::None,
                    published_iteration_head_tests_status: Passed,
                    has_notifications: false,
                    num_views: 0,
                    num_stars: 0,
                    num_comments: 0,
                    num_iterations: 1,
                    num_loc: Some(2),
                    is_out_of_date: false,
                    published_at: Some("2019-05-25T21:33:35Z".into()),
                    completed_at: Some("2019-05-25T21:33:32Z".into()),
                    updated_at: "2023-11-23T07:09:31Z".into(),
                    last_iterated_at: None,
                    exercise: Exercise {
                        name: "resistor-color".into(),
                        title: "Resistor Color".into(),
                        icon_url: "https://assets.exercism.org/exercises/resistor-color.svg".into(),
                    },
                    track: Track {
                        name: "javascript".into(),
                        title: "JavaScript".into(),
                        icon_url: "https://assets.exercism.org/tracks/javascript.svg".into(),
                    },
                }],
                meta: ResponseMeta {
                    current_page: 1,
                    total_count: 2,
                    total_pages: 1,
                },
            };
            Mock::given(method(http::Method::GET))
                .and(path("/solutions"))
                .and(query_param("criteria", "resistor"))
                .and(query_param("track_slug", "javascript"))
                .and(query_param("status", "published"))
                .and(query_param("mentoring_status", "none"))
                .and(query_param("page", "1"))
                .and(query_param("per_page", "10"))
                .and(query_param("order", "newest_first"))
                .and(bearer_token(API_TOKEN))
                .respond_with(
                    ResponseTemplate::new(http::StatusCode::OK).set_body_json(solutions_response),
                )
                .mount(&mock_server)
                .await;

            let client = api::v2::Client::builder()
                .api_base_url(mock_server.uri().as_str())
                .credentials(Credentials::from_api_token(API_TOKEN))
                .build()
                .unwrap();
            let filters = Filters::builder()
                .criteria("resistor")
                .track("javascript")
                .status(Published)
                .mentoring_status(MentoringStatus::None)
                .build();
            let paging = Paging::for_page(1).and_per_page(10);
            let sort_order = NewestFirst;
            let solutions_response = client
                .get_solutions(Some(filters), Some(paging), Some(sort_order))
                .await
                .unwrap();
            let solutions = solutions_response.results;
            let meta = solutions_response.meta;
            assert_eq!(2, solutions.len());
            assert_eq!("82f1ce4b47514db29f4831a0d2680ebd", solutions.first().unwrap().uuid);
            assert_eq!("resistor-color-duo", solutions.first().unwrap().exercise.name);
            assert_eq!("85bcc0c08a134bde8afcb16d062ad6b0", solutions.get(1).unwrap().uuid);
            assert_eq!("resistor-color", solutions.get(1).unwrap().exercise.name);
            assert_eq!(1, meta.current_page);
            assert_eq!(2, meta.total_count);
            assert_eq!(1, meta.total_pages);
        }

        #[tokio::test]
        async fn test_get_out_of_date_solutions() {
            let mock_server = MockServer::start().await;

            let solutions_response = solutions::Response {
                results: vec![Solution {
                    uuid: "82f1ce4b47514db29f4831a0d2680ebd".into(),
                    private_url: "https://exercism.org/tracks/javascript/exercises/resistor-color-duo".into(),
                    public_url: "https://exercism.org/tracks/javascript/exercises/resistor-color-duo/solutions/clechasseur".into(),
                    status: Published,
                    mentoring_status: MentoringStatus::None,
                    published_iteration_head_tests_status: Failed,
                    has_notifications: false,
                    num_views: 0,
                    num_stars: 0,
                    num_comments: 0,
                    num_iterations: 1,
                    num_loc: Some(5),
                    is_out_of_date: true,
                    published_at: Some("2019-05-25T22:17:35Z".into()),
                    completed_at: Some("2019-05-25T22:17:32Z".into()),
                    updated_at: "2023-11-18T06:22:04Z".into(),
                    last_iterated_at: None,
                    exercise: Exercise {
                        name: "resistor-color-duo".into(),
                        title: "Resistor Color Duo".into(),
                        icon_url: "https://assets.exercism.org/exercises/resistor-color-duo.svg".into(),
                    },
                    track: Track {
                        name: "javascript".into(),
                        title: "JavaScript".into(),
                        icon_url: "https://assets.exercism.org/tracks/javascript.svg".into(),
                    },
                }],
                meta: ResponseMeta {
                    current_page: 1,
                    total_count: 1,
                    total_pages: 1,
                },
            };
            Mock::given(method(http::Method::GET))
                .and(path("/solutions"))
                .and(query_param("criteria", "resistor"))
                .and(query_param("track_slug", "javascript"))
                .and(query_param("sync_status", "out_of_date"))
                .and(bearer_token(API_TOKEN))
                .respond_with(
                    ResponseTemplate::new(http::StatusCode::OK).set_body_json(solutions_response),
                )
                .mount(&mock_server)
                .await;

            let client = api::v2::Client::builder()
                .api_base_url(mock_server.uri().as_str())
                .credentials(Credentials::from_api_token(API_TOKEN))
                .build()
                .unwrap();
            let filters = Filters::builder()
                .criteria("resistor")
                .track("javascript")
                .out_of_date()
                .build();
            let solutions_response = client
                .get_solutions(Some(filters), None, None)
                .await
                .unwrap();
            let solutions = solutions_response.results;
            let meta = solutions_response.meta;
            assert_eq!(1, solutions.len());
            assert_eq!("82f1ce4b47514db29f4831a0d2680ebd", solutions.first().unwrap().uuid);
            assert_eq!("resistor-color-duo", solutions.first().unwrap().exercise.name);
            assert_eq!(1, meta.current_page);
            assert_eq!(1, meta.total_count);
            assert_eq!(1, meta.total_pages);
        }

        #[tokio::test]
        async fn test_get_up_to_date_solutions() {
            let mock_server = MockServer::start().await;

            let solutions_response = solutions::Response {
                results: vec![Solution {
                    uuid: "85bcc0c08a134bde8afcb16d062ad6b0".into(),
                    private_url: "https://exercism.org/tracks/javascript/exercises/resistor-color".into(),
                    public_url: "https://exercism.org/tracks/javascript/exercises/resistor-color/solutions/clechasseur".into(),
                    status: Published,
                    mentoring_status: MentoringStatus::None,
                    published_iteration_head_tests_status: Passed,
                    has_notifications: false,
                    num_views: 0,
                    num_stars: 0,
                    num_comments: 0,
                    num_iterations: 1,
                    num_loc: Some(2),
                    is_out_of_date: false,
                    published_at: Some("2019-05-25T21:33:35Z".into()),
                    completed_at: Some("2019-05-25T21:33:32Z".into()),
                    updated_at: "2023-11-23T07:09:31Z".into(),
                    last_iterated_at: None,
                    exercise: Exercise {
                        name: "resistor-color".into(),
                        title: "Resistor Color".into(),
                        icon_url: "https://assets.exercism.org/exercises/resistor-color.svg".into(),
                    },
                    track: Track {
                        name: "javascript".into(),
                        title: "JavaScript".into(),
                        icon_url: "https://assets.exercism.org/tracks/javascript.svg".into(),
                    },
                }],
                meta: ResponseMeta {
                    current_page: 1,
                    total_count: 1,
                    total_pages: 1,
                },
            };
            Mock::given(method(http::Method::GET))
                .and(path("/solutions"))
                .and(query_param("criteria", "resistor"))
                .and(query_param("track_slug", "javascript"))
                .and(query_param("sync_status", "up_to_date"))
                .and(bearer_token(API_TOKEN))
                .respond_with(
                    ResponseTemplate::new(http::StatusCode::OK).set_body_json(solutions_response),
                )
                .mount(&mock_server)
                .await;

            let client = api::v2::Client::builder()
                .api_base_url(mock_server.uri().as_str())
                .credentials(Credentials::from_api_token(API_TOKEN))
                .build()
                .unwrap();
            let filters = Filters::builder()
                .criteria("resistor")
                .track("javascript")
                .up_to_date()
                .build();
            let solutions_response = client
                .get_solutions(Some(filters), None, None)
                .await
                .unwrap();
            let solutions = solutions_response.results;
            let meta = solutions_response.meta;
            assert_eq!(1, solutions.len());
            assert_eq!("85bcc0c08a134bde8afcb16d062ad6b0", solutions.first().unwrap().uuid);
            assert_eq!("resistor-color", solutions.first().unwrap().exercise.name);
            assert_eq!(1, meta.current_page);
            assert_eq!(1, meta.total_count);
            assert_eq!(1, meta.total_pages);
        }
    }

    mod get_solution {
        use mini_exercism::api::v2::iteration::Status::NonActionableAutomatedFeedback;
        use mini_exercism::api::v2::iteration::{Iteration, Links};
        use mini_exercism::api::v2::solution;
        use mini_exercism::api::v2::solution::Status::Published;
        use mini_exercism::api::v2::solution::{Exercise, MentoringStatus, Solution, Track};
        use mini_exercism::api::v2::tests::Status::Passed;

        use super::*;

        #[tokio::test]
        async fn test_get_solution() {
            let mock_server = MockServer::start().await;

            let solution_response = solution::Response {
                solution: Solution {
                    uuid: "a0c9664059d345ac8d677b0154794ff2".into(),
                    private_url: "https://exercism.org/tracks/rust/exercises/clock".into(),
                    public_url:
                        "https://exercism.org/tracks/rust/exercises/clock/solutions/clechasseur"
                            .into(),
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
                    },
                },
                iterations: vec![],
            };
            Mock::given(method(http::Method::GET))
                .and(path("/solutions/a0c9664059d345ac8d677b0154794ff2"))
                .and(query_param_is_missing("sideload"))
                .and(bearer_token(API_TOKEN))
                .respond_with(
                    ResponseTemplate::new(http::StatusCode::OK).set_body_json(solution_response),
                )
                .mount(&mock_server)
                .await;

            let client = api::v2::Client::builder()
                .api_base_url(mock_server.uri().as_str())
                .credentials(Credentials::from_api_token(API_TOKEN))
                .build()
                .unwrap();
            let solution_response = client
                .get_solution("a0c9664059d345ac8d677b0154794ff2", false)
                .await
                .unwrap();
            let solution = solution_response.solution;
            let iterations = solution_response.iterations;
            assert_eq!("a0c9664059d345ac8d677b0154794ff2", solution.uuid);
            assert_eq!("clock", solution.exercise.name);
            assert_eq!("rust", solution.track.name);
            assert!(iterations.is_empty());
        }

        #[tokio::test]
        async fn test_get_iterations() {
            let mock_server = MockServer::start().await;

            let solution_response = solution::Response {
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
            Mock::given(method(http::Method::GET))
                .and(path("/solutions/a0c9664059d345ac8d677b0154794ff2"))
                .and(query_param("sideload", "iterations"))
                .and(bearer_token(API_TOKEN))
                .respond_with(
                    ResponseTemplate::new(http::StatusCode::OK).set_body_json(solution_response),
                )
                .mount(&mock_server)
                .await;

            let client = api::v2::Client::builder()
                .api_base_url(mock_server.uri().as_str())
                .credentials(Credentials::from_api_token(API_TOKEN))
                .build()
                .unwrap();
            let solution_response = client
                .get_solution("a0c9664059d345ac8d677b0154794ff2", true)
                .await
                .unwrap();
            let solution = solution_response.solution;
            let iterations = solution_response.iterations;
            assert_eq!("a0c9664059d345ac8d677b0154794ff2", solution.uuid);
            assert_eq!("clock", solution.exercise.name);
            assert_eq!("rust", solution.track.name);
            assert!(!iterations.is_empty());
            let iteration = iterations.first().unwrap();
            assert_eq!("98f8b04515a8484ca211edc7c56d2aa2", iteration.uuid);
            assert_eq!(Some("ab542af6906349ebb37e7cbee4828554".into()), iteration.submission_uuid);
            assert_eq!(1, iteration.index);
            assert!(iteration.is_latest);
        }
    }

    mod get_submission_files {
        use mini_exercism::api::v2::submission::files;
        use mini_exercism::api::v2::submission::files::File;

        use super::*;

        #[tokio::test]
        async fn test_get_files() {
            let mock_server = MockServer::start().await;

            let files_response = files::Response {
                files: vec![
                    File {
                        filename: "src/lib.rs".into(),
                        content: "mod detail;\n\nuse crate::detail::Hand;\n\n/// Given a list of poker hands, return a list of those hands which win.\n///\n/// Note the type signature: this function should return _the same_ reference to\n/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.\npub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {\n    let hands: Vec<_> = hands.iter().map(|&h| Hand::new(h).unwrap()).collect();\n    let best = hands.iter().max().unwrap();\n    hands.iter().filter(|&h| h == best).map(|h| h.hand_s()).collect()\n}\n".into(),
                        digest: "2edfab2886de7d3aadac30d6aee983e3eb965aed".into(),
                    },
                    File {
                        filename: "Cargo.toml".into(),
                        content: "[package]\nedition = \"2021\"\nname = \"poker\"\nversion = \"1.1.0\"\n\n[dependencies]\nderivative = \"2.2.0\"\nstrum = \"0.24.1\"\nstrum_macros = \"0.24.3\"\nthiserror = \"1.0.40\"\n".into(),
                        digest: "9ad1c8abd08fcc3111eaf728a9fb1f3717d10ad8".into(),
                    },
                ],
            };
            Mock::given(method(http::Method::GET))
                .and(path("/solutions/00c717b68e1b4213b316df82636f5e0f/submissions/4da3f19906214f678d5aadaea8635250/files"))
                .and(bearer_token(API_TOKEN))
                .respond_with(ResponseTemplate::new(http::StatusCode::OK).set_body_json(files_response))
                .mount(&mock_server)
                .await;

            let client = api::v2::Client::builder()
                .api_base_url(mock_server.uri().as_str())
                .credentials(Credentials::from_api_token(API_TOKEN))
                .build()
                .unwrap();
            let files_response = client
                .get_submission_files(
                    "00c717b68e1b4213b316df82636f5e0f",
                    "4da3f19906214f678d5aadaea8635250",
                )
                .await
                .unwrap();
            let files = files_response.files;
            assert!(!files.is_empty());
            let cargo_toml = files
                .iter()
                .find(|&file| file.filename == "Cargo.toml")
                .unwrap();
            assert!(cargo_toml.content.contains("edition = \"2021\""));
            assert!(cargo_toml.content.contains("thiserror"));
        }
    }
}
