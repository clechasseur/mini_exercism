mod response_tests {
    mod deserialize {
        use mini_exercism::api::v1::track;
        use mini_exercism::api::v1::track::Track;

        #[test]
        fn test_all() {
            let json = r#"{
                "track": {
                    "id": "awk",
                    "language": "AWK"
                }
            }"#;

            let expected =
                track::Response { track: Track { name: "awk".into(), title: "AWK".into() } };
            let actual = serde_json::from_str(json).unwrap();
            assert_eq!(expected, actual);
        }
    }
}

mod track_tests {
    mod deserialize {
        use mini_exercism::api::v1::track::Track;

        #[test]
        fn test_all() {
            let json = r#"{
                "id": "rust",
                "language": "Rust"
            }"#;

            let expected = Track { name: "rust".into(), title: "Rust".into() };
            let actual = serde_json::from_str(json).unwrap();
            assert_eq!(expected, actual);
        }
    }
}
