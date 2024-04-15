#[allow(clippy::module_inception)]
mod exercise {
    mod deserialize {
        use mini_exercism::api::v2::exercise::Difficulty::Easy;
        use mini_exercism::api::v2::exercise::Type::Tutorial;
        use mini_exercism::api::v2::exercise::{Difficulty, Exercise, Links, Type};

        #[test]
        fn test_all() {
            let json = r#"{
                "slug": "hello-world",
                "type": "tutorial",
                "title": "Hello World",
                "icon_url": "https://assets.exercism.org/exercises/hello-world.svg",
                "difficulty": "easy",
                "blurb": "The classical introductory exercise. Just say \"Hello, World!\".",
                "is_external": true,
                "is_unlocked": true,
                "is_recommended": false,
                "links": {
                    "self": "/tracks/rust/exercises/hello-world"
                }
            }"#;

            let expected = Exercise {
                name: "hello-world".into(),
                exercise_type: Tutorial,
                title: "Hello World".into(),
                icon_url: "https://assets.exercism.org/exercises/hello-world.svg".into(),
                difficulty: Easy,
                blurb: "The classical introductory exercise. Just say \"Hello, World!\".".into(),
                is_external: true,
                is_unlocked: true,
                is_recommended: false,
                links: Links { self_path: "/tracks/rust/exercises/hello-world".into() },
            };
            let actual: Exercise = serde_json::from_str(json).unwrap();
            assert_eq!(expected, actual);
        }

        #[test]
        fn test_unknown() {
            let json = r#"{
                "slug": "rlyehian",
                "type": "mglw'nafh",
                "title": "R'lyehian",
                "icon_url": "https://assets.exercism.org/exercises/rlyehian.svg",
                "difficulty": "syha'h",
                "blurb": "Cahf ah nafl mglw'nafh hh' ahor syha'h ah'legeth, ng llll or'azath syha'hnahh n'ghftephai n'gha ahornah ah'mglw'nafh.",
                "is_external": true,
                "is_unlocked": true,
                "is_recommended": false,
                "links": {
                    "self": "/tracks/rust/exercises/rlyehian"
                }
            }"#;

            let expected = Exercise {
                name: "rlyehian".into(),
                exercise_type: Type::Unknown,
                title: "R'lyehian".into(),
                icon_url: "https://assets.exercism.org/exercises/rlyehian.svg"
                    .into(),
                difficulty: Difficulty::Unknown,
                blurb: "Cahf ah nafl mglw'nafh hh' ahor syha'h ah'legeth, ng llll or'azath syha'hnahh n'ghftephai n'gha ahornah ah'mglw'nafh."
                    .into(),
                is_external: true,
                is_unlocked: true,
                is_recommended: false,
                links: Links {
                    self_path: "/tracks/rust/exercises/rlyehian".into(),
                },
            };
            let actual: Exercise = serde_json::from_str(json).unwrap();
            assert_eq!(expected, actual);
        }
    }
}

mod links {
    mod deserialize {
        use mini_exercism::api::v2::exercise::Links;

        #[test]
        fn test_all() {
            let json = r#"{
                "self": "/tracks/rust/exercises/hello-world"
            }"#;

            let expected = Links { self_path: "/tracks/rust/exercises/hello-world".into() };
            let actual: Links = serde_json::from_str(json).unwrap();
            assert_eq!(expected, actual);
        }
    }
}
