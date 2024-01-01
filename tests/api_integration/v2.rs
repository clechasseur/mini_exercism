mod get_tracks {
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
        let track_response = client.get_tracks(Some(filters)).await;
        let tracks = track_response.unwrap().tracks;
        assert_eq!(1, tracks.len());

        let track = tracks.first().unwrap();
        assert_eq!("julia", track.name);
        assert_eq!("Julia", track.title);
    }

    #[tokio::test]
    async fn test_tags() {
        let client = api::v2::Client::new();
        let filters = Filters::builder().tag("Functional").build();
        let track_response = client.get_tracks(Some(filters)).await;

        // Tags do not currently work.
        assert!(track_response.unwrap().tracks.is_empty());
    }

    #[tokio::test]
    async fn test_status() {
        let client = api::v2::Client::new();
        let filters = Filters::builder().status(Joined).build();
        let track_response = client.get_tracks(Some(filters)).await;

        // Asking for a specific status fails when querying anonymously.
        // Furthermore, it actually results in a `500 Internal Server Error`.
        assert_matches!(track_response,
            Err(Error::ApiError(error)) if error.status() == Some(StatusCode::INTERNAL_SERVER_ERROR));
    }
}

mod get_exercises {
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
}
