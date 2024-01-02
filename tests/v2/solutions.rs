mod filters_tests {
    mod builder {
        use mini_exercism::api::v2::solution::{MentoringStatus, Status};
        use mini_exercism::api::v2::solutions::Filters;
        use mini_exercism::api::v2::tests::Status::Passed;

        #[test]
        fn test_build() {
            let filters = Filters::builder()
                .criteria("Minesweeper")
                .track("clojure")
                .status(Status::Published)
                .mentoring_status(MentoringStatus::None)
                .is_out_of_date(false)
                .published_iteration_tests_status(Passed)
                .published_iteration_head_tests_status(Passed)
                .build();

            let expected_tests_statuses = vec![Passed];
            assert_eq!(Some("Minesweeper"), filters.criteria);
            assert_eq!(Some("clojure"), filters.track);
            assert_eq!(Some(Status::Published), filters.status);
            assert_eq!(Some(MentoringStatus::None), filters.mentoring_status);
            assert_eq!(Some(false), filters.is_out_of_date);
            assert_eq!(expected_tests_statuses, filters.published_iteration_tests_statuses);
            assert_eq!(expected_tests_statuses, filters.published_iteration_head_tests_statuses);
        }
    }
}

mod paging_tests {
    use mini_exercism::api::v2::solutions::Paging;

    #[test]
    fn test_for_page() {
        let paging = Paging::for_page(42);

        assert_eq!(42, paging.page);
        assert_eq!(None, paging.per_page);
    }

    #[test]
    fn test_and_per_page() {
        let paging = Paging::for_page(42).and_per_page(23);

        assert_eq!(42, paging.page);
        assert_eq!(Some(23), paging.per_page);
    }
}

mod response_tests {
    mod deserialize {
        use mini_exercism::api::v2::solution::Status::Published;
        use mini_exercism::api::v2::solution::{Exercise, MentoringStatus, Solution, Track};
        use mini_exercism::api::v2::solutions;
        use mini_exercism::api::v2::solutions::ResponseMeta;
        use mini_exercism::api::v2::tests::Status::Passed;

        #[test]
        fn test_authenticated() {
            let json = r#"{
                "results": [
                    {
                        "uuid": "4084d99cff6b4c349b373ab62c8e1e3a",
                        "private_url": "https://exercism.org/tracks/javascript/exercises/minesweeper",
                        "public_url": "https://exercism.org/tracks/javascript/exercises/minesweeper/solutions/clechasseur",
                        "status": "published",
                        "mentoring_status": "none",
                        "published_iteration_head_tests_status": "passed",
                        "has_notifications": false,
                        "num_views": 0,
                        "num_stars": 0,
                        "num_comments": 0,
                        "num_iterations": 1,
                        "num_loc": 26,
                        "is_out_of_date": false,
                        "published_at": "2019-05-25T02:09:22Z",
                        "completed_at": "2019-05-25T02:09:20Z",
                        "updated_at": "2023-11-23T07:13:23Z",
                        "last_iterated_at": null,
                        "exercise": {
                            "slug": "minesweeper",
                            "title": "Minesweeper",
                            "icon_url": "https://assets.exercism.org/exercises/minesweeper.svg"
                        },
                        "track": {
                            "slug": "javascript",
                            "title": "JavaScript",
                            "icon_url": "https://assets.exercism.org/tracks/javascript.svg"
                        }
                    },
                    {
                        "uuid": "100d21fe25d6489a9083586d02e0e764",
                        "private_url": "https://exercism.org/tracks/rust/exercises/minesweeper",
                        "public_url": "https://exercism.org/tracks/rust/exercises/minesweeper/solutions/clechasseur",
                        "status": "published",
                        "mentoring_status": "none",
                        "published_iteration_head_tests_status": "passed",
                        "has_notifications": false,
                        "num_views": 0,
                        "num_stars": 0,
                        "num_comments": 0,
                        "num_iterations": 2,
                        "num_loc": 32,
                        "is_out_of_date": false,
                        "published_at": "2023-03-29T06:28:03Z",
                        "completed_at": "2023-03-29T06:28:03Z",
                        "updated_at": "2023-11-20T13:12:57Z",
                        "last_iterated_at": "2023-03-29T06:25:17Z",
                        "exercise": {
                            "slug": "minesweeper",
                            "title": "Minesweeper",
                            "icon_url": "https://assets.exercism.org/exercises/minesweeper.svg"
                        },
                        "track": {
                            "slug": "rust",
                            "title": "Rust",
                            "icon_url": "https://assets.exercism.org/tracks/rust.svg"
                        }
                    }
                ],
                "meta": {
                    "current_page": 1,
                    "total_count": 2,
                    "total_pages": 1
                }
            }"#;

            let expected = solutions::Response {
                results: vec![
                    Solution {
                        uuid: "4084d99cff6b4c349b373ab62c8e1e3a".into(),
                        private_url: "https://exercism.org/tracks/javascript/exercises/minesweeper".into(),
                        public_url: "https://exercism.org/tracks/javascript/exercises/minesweeper/solutions/clechasseur".into(),
                        status: Published,
                        mentoring_status: MentoringStatus::None,
                        published_iteration_head_tests_status: Passed,
                        has_notifications: false,
                        num_views: 0,
                        num_stars: 0,
                        num_comments: 0,
                        num_iterations: 1,
                        num_loc: Some(26),
                        is_out_of_date: false,
                        published_at: Some("2019-05-25T02:09:22Z".into()),
                        completed_at: Some("2019-05-25T02:09:20Z".into()),
                        updated_at: "2023-11-23T07:13:23Z".into(),
                        last_iterated_at: None,
                        exercise: Exercise {
                            name: "minesweeper".into(),
                            title: "Minesweeper".into(),
                            icon_url: "https://assets.exercism.org/exercises/minesweeper.svg".into(),
                        },
                        track: Track {
                            name: "javascript".into(),
                            title: "JavaScript".into(),
                            icon_url: "https://assets.exercism.org/tracks/javascript.svg".into(),
                        },
                    },
                    Solution {
                        uuid: "100d21fe25d6489a9083586d02e0e764".into(),
                        private_url: "https://exercism.org/tracks/rust/exercises/minesweeper".into(),
                        public_url: "https://exercism.org/tracks/rust/exercises/minesweeper/solutions/clechasseur".into(),
                        status: Published,
                        mentoring_status: MentoringStatus::None,
                        published_iteration_head_tests_status: Passed,
                        has_notifications: false,
                        num_views: 0,
                        num_stars: 0,
                        num_comments: 0,
                        num_iterations: 2,
                        num_loc: Some(32),
                        is_out_of_date: false,
                        published_at: Some("2023-03-29T06:28:03Z".into()),
                        completed_at: Some("2023-03-29T06:28:03Z".into()),
                        updated_at: "2023-11-20T13:12:57Z".into(),
                        last_iterated_at: Some("2023-03-29T06:25:17Z".into()),
                        exercise: Exercise {
                            name: "minesweeper".into(),
                            title: "Minesweeper".into(),
                            icon_url: "https://assets.exercism.org/exercises/minesweeper.svg".into(),
                        },
                        track: Track {
                            name: "rust".into(),
                            title: "Rust".into(),
                            icon_url: "https://assets.exercism.org/tracks/rust.svg".into(),
                        },
                    },
                ],
                meta: ResponseMeta {
                    current_page: 1,
                    total_count: 2,
                    total_pages: 1,
                },
            };
            let actual: solutions::Response = serde_json::from_str(json).unwrap();
            assert_eq!(expected, actual);
        }
    }
}

mod response_meta_tests {
    mod deserialize {
        use mini_exercism::api::v2::solutions::ResponseMeta;

        #[test]
        fn test_all() {
            let json = r#"{
                "current_page": 2,
                "total_count": 42,
                "total_pages": 5
            }"#;

            let expected = ResponseMeta { current_page: 2, total_count: 42, total_pages: 5 };
            let actual: ResponseMeta = serde_json::from_str(json).unwrap();
            assert_eq!(expected, actual);
        }
    }
}
