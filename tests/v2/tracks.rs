mod filters {
    mod builder {
        use assert_matches::assert_matches;
        use mini_exercism::api::v2::tracks::Filters;
        use mini_exercism::api::v2::tracks::StatusFilter::Joined;

        #[test]
        #[test_log::test]
        fn test_build() {
            let filters = Filters::builder()
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
}

mod response {
    mod deserialize {
        use mini_exercism::api::v2::track::{Links, Track};
        use mini_exercism::api::v2::tracks;

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

            let expected = tracks::Response {
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
                        links: Links {
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
                        links: Links {
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
            let actual: tracks::Response = serde_json::from_str(json).unwrap();
            assert_eq!(expected, actual);
        }
    }
}
