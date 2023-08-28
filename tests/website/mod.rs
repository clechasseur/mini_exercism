mod tracks {
    mod deserialize {
        use mini_exercism::api::website::{Track, TrackLinks, Tracks};

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

            let expected = Tracks {
                tracks: vec![
                    Track {
                        name: "8th".to_string(),
                        title: "8th".to_string(),
                        num_concepts: 0,
                        num_exercises: 22,
                        web_url: "https://exercism.org/tracks/8th".to_string(),
                        icon_url: "https://dg8krxphbh767.cloudfront.net/tracks/8th.svg".to_string(),
                        tags: vec![
                            "Functional".to_string(),
                            "Imperative".to_string(),
                            "Procedural".to_string(),
                            "Static".to_string(),
                            "Strong".to_string(),
                            "Dynamic".to_string(),
                            "Compiled".to_string(),
                            "Interpreted".to_string(),
                            "Windows".to_string(),
                            "Mac OSX".to_string(),
                            "Linux".to_string(),
                            "iOS".to_string(),
                            "Android".to_string(),
                            "Backends".to_string(),
                            "Cross-platform development".to_string(),
                            "Frontends".to_string(),
                            "Games".to_string(),
                            "GUIs".to_string(),
                            "Mobile".to_string(),
                            "Web development".to_string(),
                        ],
                        links: TrackLinks {
                            exercises: "https://exercism.org/tracks/8th/exercises".to_string(),
                            concepts: "https://exercism.org/tracks/8th/concepts".to_string(),
                        },
                        is_joined: false,
                        num_learnt_concepts: 0,
                        num_completed_exercises: 0,
                    },
                    Track {
                        name: "abap".to_string(),
                        title: "ABAP".to_string(),
                        num_concepts: 0,
                        num_exercises: 40,
                        web_url: "https://exercism.org/tracks/abap".to_string(),
                        icon_url: "https://dg8krxphbh767.cloudfront.net/tracks/abap.svg"
                            .to_string(),
                        tags: vec![
                            "Object-oriented".to_string(),
                            "Procedural".to_string(),
                            "Static".to_string(),
                            "Strong".to_string(),
                            "Compiled".to_string(),
                            "Language-specific runtime".to_string(),
                            "Backends".to_string(),
                            "Financial systems".to_string(),
                        ],
                        links: TrackLinks {
                            exercises: "https://exercism.org/tracks/abap/exercises".to_string(),
                            concepts: "https://exercism.org/tracks/abap/concepts".to_string(),
                        },
                        is_joined: false,
                        num_learnt_concepts: 0,
                        num_completed_exercises: 0,
                    },
                ],
            };
            let actual: Tracks = serde_json::from_str(json).unwrap();
            assert_eq!(expected, actual);
        }
    }
}

mod track {
    mod deserialize {
        use mini_exercism::api::website::{Track, TrackLinks};

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
                name: "clojure".to_string(),
                title: "Clojure".to_string(),
                num_concepts: 10,
                num_exercises: 87,
                web_url: "https://exercism.org/tracks/clojure".to_string(),
                icon_url: "https://dg8krxphbh767.cloudfront.net/tracks/clojure.svg".to_string(),
                tags: vec![
                    "Declarative".to_string(),
                    "Functional".to_string(),
                    "Dynamic".to_string(),
                    "Compiled".to_string(),
                    "Windows".to_string(),
                    "Mac OSX".to_string(),
                    "Linux".to_string(),
                    "JVM (Java)".to_string(),
                    "Artificial Intelligence".to_string(),
                    "Backends".to_string(),
                    "Cross-platform development".to_string(),
                    "Financial systems".to_string(),
                    "Frontends".to_string(),
                    "Games".to_string(),
                    "GUIs".to_string(),
                    "Robotics".to_string(),
                    "Scientific calculations".to_string(),
                    "Web development".to_string(),
                ],
                links: TrackLinks {
                    exercises: "https://exercism.org/tracks/clojure/exercises".to_string(),
                    concepts: "https://exercism.org/tracks/clojure/concepts".to_string(),
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
                name: "cpp".to_string(),
                title: "C++".to_string(),
                num_concepts: 14,
                num_exercises: 73,
                web_url: "https://exercism.org/tracks/cpp".to_string(),
                icon_url: "https://dg8krxphbh767.cloudfront.net/tracks/cpp.svg".to_string(),
                tags: vec![
                    "Object-oriented".to_string(),
                    "Static".to_string(),
                    "Strong".to_string(),
                    "Compiled".to_string(),
                    "Android".to_string(),
                    "iOS".to_string(),
                    "Linux".to_string(),
                    "Mac OSX".to_string(),
                    "Windows".to_string(),
                    "Standalone executable".to_string(),
                    "Backends".to_string(),
                    "Cross-platform development".to_string(),
                    "Embedded systems".to_string(),
                    "Financial systems".to_string(),
                    "Games".to_string(),
                    "GUIs".to_string(),
                    "Mobile".to_string(),
                    "Robotics".to_string(),
                    "Scientific calculations".to_string(),
                ],
                links: TrackLinks {
                    exercises: "https://exercism.org/tracks/cpp/exercises".to_string(),
                    concepts: "https://exercism.org/tracks/cpp/concepts".to_string(),
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
        use mini_exercism::api::website::TrackLinks;

        #[test]
        fn test_all() {
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
