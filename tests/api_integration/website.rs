mod get_tracks {
    use assert_matches::assert_matches;
    use mini_exercism::api;
    use mini_exercism::api::website::TrackFilters;

    #[tokio::test]
    async fn test_all_tracks() {
        let client = api::website::Client::builder().build();
        let tracks_result = client.get_tracks(None).await;
        assert_matches!(tracks_result, Ok(_));

        let tracks = tracks_result.unwrap().tracks;
        assert!(!tracks.is_empty());

        let common_lisp_track = tracks.iter().find(|&track| track.name == "common-lisp");
        assert_matches!(common_lisp_track, Some(_));
        assert_eq!("Common Lisp", common_lisp_track.unwrap().title);
    }

    #[tokio::test]
    async fn test_julia_track() {
        let client = api::website::Client::builder().build();
        let filters = TrackFilters::builder().criteria("julia").build();
        let track_results = client.get_tracks(Some(filters)).await;
        assert_matches!(track_results, Ok(_));

        let tracks = track_results.unwrap().tracks;
        assert_eq!(1, tracks.len());

        let track = tracks.first().unwrap();
        assert_eq!("julia", track.name);
        assert_eq!("Julia", track.title);
    }

    #[tokio::test]
    async fn test_tags() {
        let client = api::website::Client::builder().build();
        let filters = TrackFilters::builder().tag("Functional").build();
        let track_results = client.get_tracks(Some(filters)).await;
        assert_matches!(track_results, Ok(_));

        // Tags do not currently work.
        assert!(track_results.unwrap().tracks.is_empty());
    }
}
