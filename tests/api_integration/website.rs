mod get_tracks {
    use assert_matches::assert_matches;
    use mini_exercism::api;
    use mini_exercism::api::website::TrackFilters;

    #[tokio::test]
    async fn test_all_tracks() {
        let client = api::website::Client::builder().build();
        let tracks_response = client.get_tracks(None).await;
        assert_matches!(tracks_response, Ok(_));

        let tracks = tracks_response.unwrap().tracks;
        assert!(!tracks.is_empty());

        let common_lisp_track = tracks.iter().find(|&track| track.name == "common-lisp");
        assert_matches!(common_lisp_track, Some(_));
        assert_eq!("Common Lisp", common_lisp_track.unwrap().title);
    }

    #[tokio::test]
    async fn test_julia_track() {
        let client = api::website::Client::builder().build();
        let filters = TrackFilters::builder().criteria("julia").build();
        let track_response = client.get_tracks(Some(filters)).await;
        assert_matches!(track_response, Ok(_));

        let tracks = track_response.unwrap().tracks;
        assert_eq!(1, tracks.len());

        let track = tracks.first().unwrap();
        assert_eq!("julia", track.name);
        assert_eq!("Julia", track.title);
    }

    #[tokio::test]
    async fn test_tags() {
        let client = api::website::Client::builder().build();
        let filters = TrackFilters::builder().tag("Functional").build();
        let track_response = client.get_tracks(Some(filters)).await;
        assert_matches!(track_response, Ok(_));

        // Tags do not currently work.
        assert!(track_response.unwrap().tracks.is_empty());
    }
}

mod get_exercises {
    use assert_matches::assert_matches;
    use mini_exercism::api;
    use mini_exercism::api::website::ExerciseFilters;

    #[tokio::test]
    async fn test_all_exercises() {
        let client = api::website::Client::builder().build();
        let exercises_response = client.get_exercises("rust", None).await;
        assert_matches!(exercises_response, Ok(_));

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
        let client = api::website::Client::builder().build();
        let filters = ExerciseFilters::builder()
            .criteria("difference-of-squares")
            .build();
        let exercises_response = client.get_exercises("rust", Some(filters)).await;
        assert_matches!(exercises_response, Ok(_));

        let exercises = exercises_response.unwrap().exercises;
        assert_eq!(1, exercises.len());
        assert_eq!("difference-of-squares", exercises.first().unwrap().name);
    }

    #[tokio::test]
    async fn test_solutions_sideloading() {
        let client = api::website::Client::builder().build();
        let filters = ExerciseFilters::builder().include_solutions(true).build();
        let exercises_response = client.get_exercises("rust", Some(filters)).await;
        assert_matches!(exercises_response, Ok(_));

        // Sideloading solutions does not work when querying anonymously.
        assert!(exercises_response.unwrap().solutions.is_empty());
    }
}
