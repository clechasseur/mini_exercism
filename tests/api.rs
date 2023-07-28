mod website {
    mod track_links {
        use mini_exercism::api::website::TrackLinks;

        #[test]
        fn test_deserialize() {
            let json = r#"{
                "self": "https://exercism.org/tracks/cpp",
                "exercises": "https://exercism.org/tracks/cpp/exercises",
                "concepts": "https://exercism.org/tracks/cpp/concepts"
            }"#;

            let expected = TrackLinks {
                exercises: "https://exercism.org/tracks/cpp/exercises".to_string(),
                concepts: "https://exercism.org/tracks/cpp/concepts".to_string(),
            };
            let actual: TrackLinks = serde_json::from_str(json).unwrap();
            assert_eq!(expected, actual);
        }
    }
}
