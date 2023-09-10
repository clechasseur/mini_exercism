mod ping {
    use assert_matches::assert_matches;
    use mini_exercism::api;

    #[tokio::test]
    async fn test_ping() {
        let client = api::v1::Client::builder().build();
        let ping_response = client.ping().await;
        assert_matches!(ping_response, Ok(_));

        let status = ping_response.unwrap().status;
        assert!(status.website);
        assert!(status.database);
    }
}
