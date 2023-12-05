mod response_tests {
    mod deserialize {
        use mini_exercism::api::v1::ping;
        use mini_exercism::api::v1::ping::ServiceStatus;

        #[test]
        fn test_all() {
            let json = r#"{
                "status": {
                    "website": true,
                    "database": true
                }
            }"#;

            let expected =
                ping::Response { status: ServiceStatus { website: true, database: true } };
            let actual = serde_json::from_str(json).unwrap();
            assert_eq!(expected, actual);
        }
    }
}

mod service_status_tests {
    mod deserialize {
        use mini_exercism::api::v1::ping::ServiceStatus;

        #[test]
        fn test_all() {
            let json = r#"{
                "website": true,
                "database": true
            }"#;

            let expected = ServiceStatus { website: true, database: true };
            let actual = serde_json::from_str(json).unwrap();
            assert_eq!(expected, actual);
        }
    }
}
