mod iteration_tests {
    mod deserialize {
        use mini_exercism::api::v2::iteration::Status::NonActionableAutomatedFeedback;
        use mini_exercism::api::v2::iteration::{Iteration, Links};
        use mini_exercism::api::v2::tests::Status::Passed;
        use mini_exercism::api::v2::{iteration, tests};

        #[test]
        fn test_all() {
            let json = r#"{
                "uuid": "98f8b04515a8484ca211edc7c56d2aa2",
                "submission_uuid": "ab542af6906349ebb37e7cbee4828554",
                "idx": 2,
                "status": "non_actionable_automated_feedback",
                "num_essential_automated_comments": 0,
                "num_actionable_automated_comments": 0,
                "num_non_actionable_automated_comments": 3,
                "num_celebratory_automated_comments": 0,
                "submission_method": "cli",
                "created_at": "2023-03-26T05:22:23Z",
                "tests_status": "passed",
                "is_published": true,
                "is_latest": true,
                "links": {
                    "self": "https://exercism.org/tracks/rust/exercises/clock/iterations?idx=2",
                    "automated_feedback": "https://exercism.org/api/v2/solutions/a0c9664059d345ac8d677b0154794ff2/iterations/98f8b04515a8484ca211edc7c56d2aa2/automated_feedback",
                    "delete": "https://exercism.org/api/v2/solutions/a0c9664059d345ac8d677b0154794ff2/iterations/98f8b04515a8484ca211edc7c56d2aa2",
                    "solution": "https://exercism.org/tracks/rust/exercises/clock",
                    "test_run": "https://exercism.org/api/v2/solutions/a0c9664059d345ac8d677b0154794ff2/submissions/ab542af6906349ebb37e7cbee4828554/test_run",
                    "files": "https://exercism.org/api/v2/solutions/a0c9664059d345ac8d677b0154794ff2/submissions/ab542af6906349ebb37e7cbee4828554/files"
                }
            }"#;

            let expected = Iteration {
                uuid: "98f8b04515a8484ca211edc7c56d2aa2".into(),
                submission_uuid: "ab542af6906349ebb37e7cbee4828554".into(),
                index: 2,
                status: NonActionableAutomatedFeedback,
                num_essential_automated_comments: 0,
                num_actionable_automated_comments: 0,
                num_non_actionable_automated_comments: 3,
                num_celebratory_automated_comments: 0,
                submission_method: "cli".into(),
                created_at: "2023-03-26T05:22:23Z".into(),
                tests_status: Passed,
                representer_feedback: None,
                analyzer_feedback: None,
                is_published: true,
                is_latest: true,
                files: vec![],
                links: Links {
                    self_path: "https://exercism.org/tracks/rust/exercises/clock/iterations?idx=2".into(),
                    automated_feedback: "https://exercism.org/api/v2/solutions/a0c9664059d345ac8d677b0154794ff2/iterations/98f8b04515a8484ca211edc7c56d2aa2/automated_feedback".into(),
                    delete: "https://exercism.org/api/v2/solutions/a0c9664059d345ac8d677b0154794ff2/iterations/98f8b04515a8484ca211edc7c56d2aa2".into(),
                    solution: "https://exercism.org/tracks/rust/exercises/clock".into(),
                    test_run: "https://exercism.org/api/v2/solutions/a0c9664059d345ac8d677b0154794ff2/submissions/ab542af6906349ebb37e7cbee4828554/test_run".into(),
                    files: "https://exercism.org/api/v2/solutions/a0c9664059d345ac8d677b0154794ff2/submissions/ab542af6906349ebb37e7cbee4828554/files".into()
                }
            };
            let actual: Iteration = serde_json::from_str(json).unwrap();
            assert_eq!(expected, actual);
        }

        #[test]
        fn test_unknown() {
            let json = r#"{
                "uuid": "667beaee5e6d4a67a2679545879e6c3f",
                "submission_uuid": "4a41c68afbf343268fe78dd3ce81f44e",
                "idx": 2,
                "status": "or'azath",
                "num_essential_automated_comments": 0,
                "num_actionable_automated_comments": 0,
                "num_non_actionable_automated_comments": 3,
                "num_celebratory_automated_comments": 0,
                "submission_method": "mggoka'drn",
                "created_at": "2023-03-26T05:22:23Z",
                "tests_status": "ah'mglw'nafh",
                "is_published": true,
                "is_latest": true,
                "links": {
                    "self": "https://exercism.org/tracks/rust/exercises/rlyehian/iterations?idx=2",
                    "automated_feedback": "https://exercism.org/api/v2/solutions/826fff5ec5d246aa904c4270126efde9/iterations/667beaee5e6d4a67a2679545879e6c3f/automated_feedback",
                    "delete": "https://exercism.org/api/v2/solutions/826fff5ec5d246aa904c4270126efde9/iterations/667beaee5e6d4a67a2679545879e6c3f",
                    "solution": "https://exercism.org/tracks/rust/exercises/rlyehian",
                    "test_run": "https://exercism.org/api/v2/solutions/826fff5ec5d246aa904c4270126efde9/submissions/4a41c68afbf343268fe78dd3ce81f44e/test_run",
                    "files": "https://exercism.org/api/v2/solutions/826fff5ec5d246aa904c4270126efde9/submissions/4a41c68afbf343268fe78dd3ce81f44e/files"
                }
            }"#;

            let expected = Iteration {
                uuid: "667beaee5e6d4a67a2679545879e6c3f".into(),
                submission_uuid: "4a41c68afbf343268fe78dd3ce81f44e".into(),
                index: 2,
                status: iteration::Status::Unknown,
                num_essential_automated_comments: 0,
                num_actionable_automated_comments: 0,
                num_non_actionable_automated_comments: 3,
                num_celebratory_automated_comments: 0,
                submission_method: "mggoka'drn".into(),
                created_at: "2023-03-26T05:22:23Z".into(),
                tests_status: tests::Status::Unknown,
                representer_feedback: None,
                analyzer_feedback: None,
                is_published: true,
                is_latest: true,
                files: vec![],
                links: Links {
                    self_path: "https://exercism.org/tracks/rust/exercises/rlyehian/iterations?idx=2".into(),
                    automated_feedback: "https://exercism.org/api/v2/solutions/826fff5ec5d246aa904c4270126efde9/iterations/667beaee5e6d4a67a2679545879e6c3f/automated_feedback".into(),
                    delete: "https://exercism.org/api/v2/solutions/826fff5ec5d246aa904c4270126efde9/iterations/667beaee5e6d4a67a2679545879e6c3f".into(),
                    solution: "https://exercism.org/tracks/rust/exercises/rlyehian".into(),
                    test_run: "https://exercism.org/api/v2/solutions/826fff5ec5d246aa904c4270126efde9/submissions/4a41c68afbf343268fe78dd3ce81f44e/test_run".into(),
                    files: "https://exercism.org/api/v2/solutions/826fff5ec5d246aa904c4270126efde9/submissions/4a41c68afbf343268fe78dd3ce81f44e/files".into()
                }
            };
            let actual: Iteration = serde_json::from_str(json).unwrap();
            assert_eq!(expected, actual);
        }
    }
}

mod links_tests {
    mod deserialize {
        use mini_exercism::api::v2::iteration::Links;

        #[test]
        fn test_all() {
            let json = r#" {
                "self": "https://exercism.org/tracks/rust/exercises/clock/iterations?idx=2",
                "automated_feedback": "https://exercism.org/api/v2/solutions/a0c9664059d345ac8d677b0154794ff2/iterations/98f8b04515a8484ca211edc7c56d2aa2/automated_feedback",
                "delete": "https://exercism.org/api/v2/solutions/a0c9664059d345ac8d677b0154794ff2/iterations/98f8b04515a8484ca211edc7c56d2aa2",
                "solution": "https://exercism.org/tracks/rust/exercises/clock",
                "test_run": "https://exercism.org/api/v2/solutions/a0c9664059d345ac8d677b0154794ff2/submissions/ab542af6906349ebb37e7cbee4828554/test_run",
                "files": "https://exercism.org/api/v2/solutions/a0c9664059d345ac8d677b0154794ff2/submissions/ab542af6906349ebb37e7cbee4828554/files"
            }"#;

            let expected = Links {
                self_path: "https://exercism.org/tracks/rust/exercises/clock/iterations?idx=2".into(),
                automated_feedback: "https://exercism.org/api/v2/solutions/a0c9664059d345ac8d677b0154794ff2/iterations/98f8b04515a8484ca211edc7c56d2aa2/automated_feedback".into(),
                delete: "https://exercism.org/api/v2/solutions/a0c9664059d345ac8d677b0154794ff2/iterations/98f8b04515a8484ca211edc7c56d2aa2".into(),
                solution: "https://exercism.org/tracks/rust/exercises/clock".into(),
                test_run: "https://exercism.org/api/v2/solutions/a0c9664059d345ac8d677b0154794ff2/submissions/ab542af6906349ebb37e7cbee4828554/test_run".into(),
                files: "https://exercism.org/api/v2/solutions/a0c9664059d345ac8d677b0154794ff2/submissions/ab542af6906349ebb37e7cbee4828554/files".into()
            };
            let actual: Links = serde_json::from_str(json).unwrap();
            assert_eq!(expected, actual);
        }
    }
}
