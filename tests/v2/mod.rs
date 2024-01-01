mod exercise;
mod exercises;
mod solution;
mod track;
mod tracks;

mod client {
    use mini_exercism::api;
    use mini_exercism::core::Credentials;
    use reqwest::StatusCode;
    use wiremock::http::Method::Get;
    use wiremock::matchers::{bearer_token, method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    const API_TOKEN: &str = "some_api_token";

    mod debug {
        use super::*;

        #[test]
        fn test_derive() {
            // Note: this test is necessary because of a bug in cargo-tarpaulin, see
            // https://github.com/xd009642/tarpaulin/issues/351#issuecomment-1722148936
            let client = api::v2::Client::new();
            assert!(!format!("{:?}", client).is_empty());
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
        use mini_exercism::api::v2::solution::TestsStatus::Passed;
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
}
