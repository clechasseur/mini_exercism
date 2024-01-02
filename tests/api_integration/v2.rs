mod get_tracks_tests {
    use assert_matches::assert_matches;
    use mini_exercism::api;
    use mini_exercism::api::v2::tracks::Filters;
    use mini_exercism::api::v2::tracks::StatusFilter::Joined;
    use mini_exercism::Error;
    use reqwest::StatusCode;

    #[tokio::test]
    async fn test_all_tracks() {
        let client = api::v2::Client::new();
        let tracks_response = client.get_tracks(None).await;
        let tracks = tracks_response.unwrap().tracks;
        assert!(!tracks.is_empty());

        let common_lisp_track = tracks.iter().find(|&track| track.name == "common-lisp");
        assert_eq!("Common Lisp", common_lisp_track.unwrap().title);
    }

    #[tokio::test]
    async fn test_julia_track() {
        let client = api::v2::Client::new();
        let filters = Filters::builder().criteria("julia").build();
        let tracks_response = client.get_tracks(Some(filters)).await;
        let tracks = tracks_response.unwrap().tracks;
        assert_eq!(1, tracks.len());

        let track = tracks.first().unwrap();
        assert_eq!("julia", track.name);
        assert_eq!("Julia", track.title);
    }

    #[tokio::test]
    async fn test_tags() {
        let client = api::v2::Client::new();
        let filters = Filters::builder().tag("Functional").build();
        let tracks_response = client.get_tracks(Some(filters)).await;

        // Tags do not currently work.
        assert!(tracks_response.unwrap().tracks.is_empty());
    }

    #[tokio::test]
    async fn test_status() {
        let client = api::v2::Client::new();
        let filters = Filters::builder().status(Joined).build();
        let tracks_response = client.get_tracks(Some(filters)).await;

        // Asking for a specific status fails when querying anonymously.
        // Furthermore, it actually results in a `500 Internal Server Error`.
        assert_matches!(tracks_response,
            Err(Error::ApiError(error)) if error.status() == Some(StatusCode::INTERNAL_SERVER_ERROR));
    }

    #[cfg(feature = "cli")]
    mod authenticated_with_cli {
        use mini_exercism::cli::get_cli_credentials;

        use super::*;

        #[tokio::test]
        async fn test_joined_tracks() {
            if let Ok(credentials) = get_cli_credentials() {
                let client = api::v2::Client::builder().credentials(credentials).build();
                let filters = Filters::builder().status(Joined).build();
                let tracks_response = client.get_tracks(Some(filters)).await;

                assert_matches!(tracks_response, Ok(response) => {
                    let tracks = response.tracks;
                    assert!(tracks.len() >= 17);
                    assert!(tracks.iter().any(|track| track.name == "rust"));
                });
            }
        }
    }
}

mod get_exercises_tests {
    use assert_matches::assert_matches;
    use mini_exercism::api;
    use mini_exercism::api::v2::exercises::Filters;

    #[tokio::test]
    async fn test_all_exercises() {
        let client = api::v2::Client::new();
        let exercises_response = client.get_exercises("rust", None).await;
        let exercises_response = exercises_response.unwrap();
        assert!(!exercises_response.exercises.is_empty());
        assert!(exercises_response.solutions.is_empty());

        let poker_exercise = exercises_response
            .exercises
            .iter()
            .find(|&exercise| exercise.name == "poker");
        assert_matches!(poker_exercise, Some(exercise) if exercise.title == "Poker");
    }

    #[tokio::test]
    async fn test_difference_of_squares_exercise() {
        let client = api::v2::Client::new();
        let filters = Filters::builder().criteria("difference-of-squares").build();
        let exercises_response = client.get_exercises("rust", Some(filters)).await;
        let exercises = exercises_response.unwrap().exercises;
        assert_eq!(1, exercises.len());
        assert_eq!("difference-of-squares", exercises.first().unwrap().name);
    }

    #[tokio::test]
    async fn test_solutions_sideloading() {
        let client = api::v2::Client::new();
        let filters = Filters::builder().include_solutions(true).build();
        let exercises_response = client.get_exercises("rust", Some(filters)).await;

        // Sideloading solutions does not work when querying anonymously.
        assert!(exercises_response.unwrap().solutions.is_empty());
    }

    #[cfg(feature = "cli")]
    mod authenticated_with_cli {
        use mini_exercism::api::v2::solution::MentoringStatus::Finished;
        use mini_exercism::api::v2::solution::Status::Published;
        use mini_exercism::cli::get_cli_credentials;

        use super::*;

        #[tokio::test]
        async fn test_solutions_sideloading() {
            if let Ok(credentials) = get_cli_credentials() {
                let client = api::v2::Client::builder().credentials(credentials).build();
                let filters = Filters::builder().include_solutions(true).build();
                let exercises_response = client.get_exercises("rust", Some(filters)).await;

                assert_matches!(exercises_response, Ok(response) => {
                    let solutions = response.solutions;
                    assert!(!solutions.is_empty());

                    let poker_solution = solutions.iter().find(|&solution| solution.exercise.name == "poker");
                    assert_matches!(poker_solution, Some(poker_solution) => {
                        assert_eq!(Published, poker_solution.status);
                        assert_eq!(Finished, poker_solution.mentoring_status);
                        assert!(poker_solution.num_iterations >= 10);
                    });
                });
            }
        }
    }
}

mod get_solutions_tests {
    use assert_matches::assert_matches;
    use mini_exercism::api;
    use mini_exercism::api::v2::solutions::{Filters, Paging, SortOrder};
    use mini_exercism::Error;
    use reqwest::StatusCode;

    #[tokio::test]
    async fn test_minesweeper() {
        let client = api::v2::Client::new();
        let filters = Filters::builder().criteria("minesweeper").build();
        let paging = Paging::for_page(1).and_per_page(10);
        let sort_order = SortOrder::NewestFirst;
        let solutions_response = client
            .get_solutions(Some(filters), Some(paging), Some(sort_order))
            .await;

        // Fetching solutions doesn't work anonymously.
        assert_matches!(solutions_response, Err(Error::ApiError(error)) if error.status() == Some(StatusCode::UNAUTHORIZED));
    }

    #[cfg(feature = "cli")]
    mod authenticated_via_cli {
        use mini_exercism::cli::get_cli_credentials;

        use super::*;

        #[tokio::test]
        async fn test_minesweeper() {
            if let Ok(credentials) = get_cli_credentials() {
                let client = api::v2::Client::builder().credentials(credentials).build();
                let filters = Filters::builder().criteria("minesweeper").build();
                let paging = Paging::for_page(1).and_per_page(50);
                let solutions_response = client
                    .get_solutions(Some(filters), Some(paging), None)
                    .await;

                assert_matches!(solutions_response, Ok(response) => {
                    assert_eq!(1, response.meta.total_pages);

                    let solutions = response.results;
                    assert!(!solutions.is_empty());
                    assert!(solutions.iter().any(|solution| solution.track.name == "rust"));
                });
            }
        }
    }
}

mod get_solution_tests {
    use assert_matches::assert_matches;
    use mini_exercism::{api, Error};
    use reqwest::StatusCode;

    const SOLUTION_UUID: &str = "a0c9664059d345ac8d677b0154794ff2";

    #[tokio::test]
    async fn test_solution() {
        let client = api::v2::Client::new();
        let solution_response = client.get_solution(SOLUTION_UUID, false).await;

        // Fetching a solution doesn't work anonymously.
        assert_matches!(solution_response, Err(Error::ApiError(error)) if error.status() == Some(StatusCode::UNAUTHORIZED));
    }

    #[tokio::test]
    async fn test_iterations() {
        let client = api::v2::Client::new();
        let solution_response = client.get_solution(SOLUTION_UUID, true).await;

        // Fetching iterations for a solution doesn't work anonymously.
        assert_matches!(solution_response, Err(Error::ApiError(error)) if error.status() == Some(StatusCode::UNAUTHORIZED));
    }

    #[cfg(feature = "cli")]
    mod authenticated_with_cli {
        use mini_exercism::api::v2::iteration::Status::NonActionableAutomatedFeedback;
        use mini_exercism::api::v2::tests::Status::Passed;
        use mini_exercism::cli::get_cli_credentials;

        use super::*;

        #[tokio::test]
        async fn test_iterations() {
            if let Ok(credentials) = get_cli_credentials() {
                let client = api::v2::Client::builder().credentials(credentials).build();
                let solution_response = client.get_solution(SOLUTION_UUID, true).await;

                assert_matches!(solution_response, Ok(response) => {
                    let iterations = response.iterations;
                    assert!(!iterations.is_empty());

                    let latest_iteration = iterations.iter().find(|&iteration| iteration.is_latest);
                    assert_matches!(latest_iteration, Some(iteration) => {
                        assert_eq!(NonActionableAutomatedFeedback, iteration.status);
                        assert_eq!(3, iteration.num_non_actionable_automated_comments);
                        assert_eq!(Passed, iteration.tests_status);
                    });
                });
            }
        }
    }
}

mod get_submission_files_tests {
    use assert_matches::assert_matches;
    use mini_exercism::api;

    const SOLUTION_UUID: &str = "00c717b68e1b4213b316df82636f5e0f";
    const SUBMISSION_UUID: &str = "4da3f19906214f678d5aadaea8635250";

    #[tokio::test]
    async fn test_files_content() {
        let client = api::v2::Client::new();
        let files_response = client
            .get_submission_files(SOLUTION_UUID, SUBMISSION_UUID)
            .await;

        assert_matches!(files_response, Ok(response) => {
            let files = response.files;
            assert!(!files.is_empty());

            let cargo_toml = files.iter().find(|&file| file.filename == "Cargo.toml");
            assert_matches!(cargo_toml, Some(cargo_toml) => {
                assert!(cargo_toml.content.contains("edition = \"2021\""));
                assert!(cargo_toml.content.contains("thiserror"));
            });
        });
    }
}
