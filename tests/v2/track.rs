#[allow(clippy::module_inception)]
mod track {
    mod deserialize {
        use mini_exercism::api::v2::track::{Links, Track};

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
                name: "clojure".into(),
                title: "Clojure".into(),
                num_concepts: 10,
                num_exercises: 87,
                web_url: "https://exercism.org/tracks/clojure".into(),
                icon_url: "https://dg8krxphbh767.cloudfront.net/tracks/clojure.svg".into(),
                tags: vec![
                    "Declarative".into(),
                    "Functional".into(),
                    "Dynamic".into(),
                    "Compiled".into(),
                    "Windows".into(),
                    "Mac OSX".into(),
                    "Linux".into(),
                    "JVM (Java)".into(),
                    "Artificial Intelligence".into(),
                    "Backends".into(),
                    "Cross-platform development".into(),
                    "Financial systems".into(),
                    "Frontends".into(),
                    "Games".into(),
                    "GUIs".into(),
                    "Robotics".into(),
                    "Scientific calculations".into(),
                    "Web development".into(),
                ],
                links: Links {
                    self_url: "https://exercism.org/tracks/clojure".into(),
                    exercises: "https://exercism.org/tracks/clojure/exercises".into(),
                    concepts: "https://exercism.org/tracks/clojure/concepts".into(),
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
            };
            let actual: Track = serde_json::from_str(json).unwrap();
            assert_eq!(expected, actual);
        }
    }
}

mod links {
    mod deserialize {
        use mini_exercism::api::v2::track::Links;

        #[test]
        fn test_all() {
            let json = r#"{
                "self": "https://exercism.org/tracks/cpp",
                "exercises": "https://exercism.org/tracks/cpp/exercises",
                "concepts": "https://exercism.org/tracks/cpp/concepts"
            }"#;

            let expected = Links {
                self_url: "https://exercism.org/tracks/cpp".into(),
                exercises: "https://exercism.org/tracks/cpp/exercises".into(),
                concepts: "https://exercism.org/tracks/cpp/concepts".into(),
            };
            let actual: Links = serde_json::from_str(json).unwrap();
            assert_eq!(expected, actual);
        }
    }
}
