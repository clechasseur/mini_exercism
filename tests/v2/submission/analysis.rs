mod analyzer_feedback_tests {
    mod deserialize {
        use mini_exercism::api::v2::submission::analysis::AnalyzerCommentType::Informative;
        use mini_exercism::api::v2::submission::analysis::{AnalyzerComment, AnalyzerFeedback};

        #[test]
        fn test_all() {
            let json = r#"{
                "summary": null,
                "comments": [
                    {
                        "type": "informative",
                        "html": "<p>Nice work using <code>impl Display</code> to implement to_string.</p>\n"
                    },
                    {
                        "type": "informative",
                        "html": "<p>Nice work using rem_euclid!</p>\n"
                    },
                    {
                        "type": "informative",
                        "html": "<p>(Some people don't bother storing the hours in the struct which simplifies things a bit.)</p>\n"
                    }
                ]
            }"#;

            let expected = AnalyzerFeedback {
                summary: None,
                comments: vec![
                    AnalyzerComment {
                        comment_type: Informative,
                        html: "<p>Nice work using <code>impl Display</code> to implement to_string.</p>\n".into(),
                    },
                    AnalyzerComment {
                        comment_type: Informative,
                        html: "<p>Nice work using rem_euclid!</p>\n".into(),
                    },
                    AnalyzerComment {
                        comment_type: Informative,
                        html: "<p>(Some people don't bother storing the hours in the struct which simplifies things a bit.)</p>\n".into(),
                    },
                ],
            };
            let actual: AnalyzerFeedback = serde_json::from_str(json).unwrap();
            assert_eq!(expected, actual);
        }
    }
}

mod analyzer_comment_tests {
    mod deserialize {
        use mini_exercism::api::v2::submission::analysis::AnalyzerCommentType::Celebratory;
        use mini_exercism::api::v2::submission::analysis::{AnalyzerComment, AnalyzerCommentType};

        #[test]
        fn test_all() {
            let json = r#"{
                "type": "celebratory",
                "html": "<p>Nice work using rem_euclid!</p>\n"
            }"#;

            let expected = AnalyzerComment {
                comment_type: Celebratory,
                html: "<p>Nice work using rem_euclid!</p>\n".into(),
            };
            let actual: AnalyzerComment = serde_json::from_str(json).unwrap();
            assert_eq!(expected, actual);
        }

        #[test]
        fn test_unknown() {
            let json = r#"{
                "type": "mglw'nafh",
                "html": "<p>Cahf ah nafl mglw'nafh hh' ahor syha'h ah'legeth, ng llll or'azath syha'hnahh n'ghftephai n'gha ahornah ah'mglw'nafh</p>"
            }"#;

            let expected = AnalyzerComment {
                comment_type: AnalyzerCommentType::Unknown,
                html: "<p>Cahf ah nafl mglw'nafh hh' ahor syha'h ah'legeth, ng llll or'azath syha'hnahh n'ghftephai n'gha ahornah ah'mglw'nafh</p>".into(),
            };
            let actual: AnalyzerComment = serde_json::from_str(json).unwrap();
            assert_eq!(expected, actual);
        }
    }
}

mod representer_feedback_tests {
    mod deserialize {
        use mini_exercism::api::v2::submission::analysis::{FeedbackAuthor, RepresenterFeedback};
        use mini_exercism::api::v2::user::Flair::LifetimeInsider;

        #[test]
        fn test_all() {
            let json = r#"{
                "html": "<p>This looks great. Thank you for the submission!</p>\n",
                "author": {
                    "name": "John Smith",
                    "reputation": 12345,
                    "flair": "lifetime_insider",
                    "avatar_url": "https://assets.exercism.org/avatars/31337/0",
                    "profile_url": "https://exercism.org/profiles/jsmith_mini_exercism"
                },
                "editor": null
            }"#;

            let expected = RepresenterFeedback {
                html: "<p>This looks great. Thank you for the submission!</p>\n".into(),
                author: FeedbackAuthor {
                    name: "John Smith".into(),
                    reputation: 12345,
                    flair: Some(LifetimeInsider),
                    avatar_url: "https://assets.exercism.org/avatars/31337/0".into(),
                    profile_url: Some("https://exercism.org/profiles/jsmith_mini_exercism".into()),
                },
                editor: None,
            };
            let actual: RepresenterFeedback = serde_json::from_str(json).unwrap();
            assert_eq!(expected, actual);
        }
    }
}

mod feedback_author_tests {
    mod deserialize {
        use mini_exercism::api::v2::submission::analysis::FeedbackAuthor;
        use mini_exercism::api::v2::user::Flair::LifetimeInsider;

        #[test]
        fn test_all() {
            let json = r#"{
                "name": "Charles Lechasseur",
                "reputation": 2213,
                "flair": "lifetime_insider",
                "avatar_url": "https://assets.exercism.org/avatars/295160/0",
                "profile_url": "https://exercism.org/profiles/clechasseur"
            }"#;

            let expected = FeedbackAuthor {
                name: "Charles Lechasseur".into(),
                reputation: 2213,
                flair: Some(LifetimeInsider),
                avatar_url: "https://assets.exercism.org/avatars/295160/0".into(),
                profile_url: Some("https://exercism.org/profiles/clechasseur".into()),
            };
            let actual: FeedbackAuthor = serde_json::from_str(json).unwrap();
            assert_eq!(expected, actual);
        }
    }
}
