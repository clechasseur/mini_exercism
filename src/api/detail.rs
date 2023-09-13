use derive_builder::Builder;
use reqwest::{Client, Method, RequestBuilder};

use crate::core::Credentials;

#[derive(Builder)]
pub struct ApiClient {
    #[builder(default)]
    http_client: Client,

    #[builder(setter(custom))]
    api_base_url: String,

    #[builder(default, setter(strip_option))]
    credentials: Option<Credentials>,
}

impl ApiClient {
    pub fn builder() -> ApiClientBuilder {
        ApiClientBuilder::default()
    }

    // Note: this method is indeed used in the tests below; not sure why rustc thinks otherwise...
    #[allow(dead_code)]
    pub fn api_base_url(&self) -> &str {
        self.api_base_url.as_str()
    }

    pub fn request(&self, method: Method, url: &str) -> RequestBuilder {
        let builder = self.http_client.request(method, self.api_url(url));
        match &self.credentials {
            Some(creds) => builder.bearer_auth(creds.api_token()),
            None => builder,
        }
    }

    pub fn get(&self, url: &str) -> RequestBuilder {
        self.request(Method::GET, url)
    }

    fn api_url(&self, url: &str) -> String {
        format!("{}{}", self.api_base_url, url)
    }
}

impl ApiClientBuilder {
    pub fn api_base_url(&mut self, url: &str) -> &mut Self {
        self.api_base_url = Some(url.trim_end_matches('/').to_string());
        self
    }
}

macro_rules! define_api_client {
    (
        $(#[$attr:meta])*
        pub struct $api_name:ident($base_url:expr);
    ) => {
        paste::paste! {
            $(#[$attr])*
            pub struct $api_name {
                api_client: $crate::api::detail::ApiClient,
            }

            impl $api_name {
                #[doc = r"
                    Creates a new [`" $api_name r"`] with default values.

                    This is the same as calling `" $api_name r"::builder().build()`.

                    # Panics

                    This method can panic when building a default HTTP client. To handle this
                    type of error gracefully, use a [`builder`](Self::builder). (See
                    [crate documentation](crate#custom-http-client) for details.)
                "]
                pub fn new() -> Self {
                    Self::default()
                }

                #[doc = r"
                    Returns a [`" $api_name r"Builder`] that can be used to
                    create an API client instance.
                "]
                pub fn builder() -> [<$api_name Builder>] {
                    [<$api_name Builder>]::default()
                }
            }

            impl Default for $api_name {
                #[doc = r"
                    Creates a new [`" $api_name r"`] with default values.

                    This is the same as calling `" $api_name r"::builder().build()`.

                    # Panics

                    This method can panic when building a default HTTP client. To handle this
                    type of error gracefully, use a [`builder`](Self::builder). (See
                    [crate documentation](crate#custom-http-client) for details.)
                "]
                fn default() -> Self {
                    Self::builder().build()
                }
            }

            #[doc = r"
                Builder for the [`" $api_name r"`] type.

                To create a builder instance, call [`" $api_name r"::builder`].

                Because all fields have default values, it is legal to create
                an instance of this builder and simply call [`build`](" $api_name r"Builder::build).
            "]
            pub struct [<$api_name Builder>] {
                api_client_builder: $crate::api::detail::ApiClientBuilder,
            }

            impl [<$api_name Builder>] {
                #[doc = r"
                    Creates a new [`" $api_name r"Builder`] that can be used to
                    create an API client instance.

                    This is the same as calling [`" $api_name "::builder`].
                "]
                pub fn new() -> Self {
                    Self::default()
                }

                #[doc = r"
                    Sets the [HTTP client](reqwest::Client) to use to perform requests
                    to the API. If not specified, a default client will be created.
                "]
                pub fn http_client(&mut self, value: reqwest::Client) -> &mut Self {
                    self.api_client_builder.http_client(value);
                    self
                }

                #[doc = r"
                    Sets the base URL to use to connect to the API.

                    Normally, this is set to the default value ([`" $base_url r"`])
                    when the builder is created and should not be changed.
                "]
                pub fn api_base_url(&mut self, value: &str) -> &mut Self {
                    self.api_client_builder.api_base_url(value);
                    self
                }

                #[doc = r"
                    Sets the [`Credentials`](crate::core::Credentials) to use to
                    connect to the API. If not specified, requests will be performed
                    anonymously.
                "]
                pub fn credentials(&mut self, value: $crate::core::Credentials) -> &mut Self {
                    self.api_client_builder.credentials(value);
                    self
                }

                #[doc = "Builds a new [`" $api_name "`] instance using the parameters of this builder."]
                pub fn build(&self) -> $api_name {
                    $api_name {
                        api_client: self.api_client_builder.build().unwrap(),
                    }
                }
            }

            impl Default for [<$api_name Builder>] {
                #[doc = r"
                    Returns a default [`" $api_name r"Builder`] instance.

                    This is the same as calling [`" $api_name "::builder`].
                "]
                fn default() -> Self {
                    let mut api_client_builder = $crate::api::detail::ApiClient::builder();
                    api_client_builder.api_base_url($base_url);
                    Self { api_client_builder }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod api_client {
        use reqwest::header::{HeaderMap, HeaderValue};
        use reqwest::StatusCode;
        use wiremock::matchers::{bearer_token, header_exists, method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};
        use wiremock_logical_matchers::not;

        use super::*;

        const API_TOKEN: &str = "some_api_token";
        const TEST_HEADER: &str = "x-mini_exercism-test";

        const ANONYMOUS_API_PATH: &str = "/anonymous";
        const AUTHENTICATED_API_PATH: &str = "/authenticated";
        const TEST_API_PATH: &str = "/test";

        async fn setup_route<T: Into<String>>(
            mock_server: &MockServer,
            route: T,
            anonymous: bool,
            test: bool,
        ) {
            let mut mock = Mock::given(method("GET")).and(path(route));

            if anonymous {
                mock = mock.and(not(header_exists("authorization")));
            } else {
                mock = mock.and(bearer_token(API_TOKEN));
            }

            if test {
                mock = mock.and(header_exists(TEST_HEADER));
            } else {
                mock = mock.and(not(header_exists(TEST_HEADER)));
            }

            mock.respond_with(ResponseTemplate::new(StatusCode::OK))
                .mount(mock_server)
                .await;
        }

        async fn setup_mock_server() -> MockServer {
            let mock_server = MockServer::start().await;

            setup_route(&mock_server, ANONYMOUS_API_PATH, true, false).await;
            setup_route(
                &mock_server,
                format!("{}{}", ANONYMOUS_API_PATH, TEST_API_PATH),
                true,
                true,
            )
            .await;
            setup_route(&mock_server, AUTHENTICATED_API_PATH, false, false).await;
            setup_route(
                &mock_server,
                format!("{}{}", AUTHENTICATED_API_PATH, TEST_API_PATH),
                false,
                true,
            )
            .await;

            mock_server
        }

        fn expected_status_code(working_uri: &str, actual_uri: &str) -> StatusCode {
            if working_uri == actual_uri {
                StatusCode::OK
            } else {
                StatusCode::NOT_FOUND
            }
        }

        fn get_uri_to_test(anonymous: bool, test: bool) -> String {
            match (anonymous, test) {
                (true, false) => ANONYMOUS_API_PATH.to_string(),
                (true, true) => format!("{}{}", ANONYMOUS_API_PATH, TEST_API_PATH),
                (false, false) => AUTHENTICATED_API_PATH.to_string(),
                (false, true) => format!("{}{}", AUTHENTICATED_API_PATH, TEST_API_PATH),
            }
        }

        async fn test_routes(client: &ApiClient, anonymous: bool, test: bool) {
            let working_uri = get_uri_to_test(anonymous, test);

            let uris = vec![
                get_uri_to_test(true, false),
                get_uri_to_test(true, true),
                get_uri_to_test(false, false),
                get_uri_to_test(false, true),
            ];

            for uri in &uris {
                let request_status = client
                    .request(Method::GET, uri)
                    .send()
                    .await
                    .unwrap()
                    .status();
                let get_status = client.get(uri).send().await.unwrap().status();

                let expected_status = expected_status_code(&working_uri, uri);
                assert_eq!(
                    request_status, expected_status,
                    "Tried to request {}, expected {}, got {}",
                    uri, expected_status, request_status
                );
                assert_eq!(
                    get_status, expected_status,
                    "Tried to get {}, expected {}, got {}",
                    uri, expected_status, get_status
                );
            }
        }

        fn create_test_http_client() -> Client {
            let mut default_headers = HeaderMap::new();
            default_headers.insert(TEST_HEADER, HeaderValue::from_static("any_value_will_do"));

            Client::builder()
                .default_headers(default_headers)
                .build()
                .unwrap()
        }

        fn create_authenticated_credentials() -> Credentials {
            Credentials::from_api_token(API_TOKEN)
        }

        fn create_anonymous_api_client(mock_server: &MockServer) -> ApiClient {
            ApiClient::builder()
                .api_base_url(&mock_server.uri())
                .build()
                .unwrap()
        }

        fn create_anonymous_test_api_client(mock_server: &MockServer) -> ApiClient {
            ApiClient::builder()
                .http_client(create_test_http_client())
                .api_base_url(&mock_server.uri())
                .build()
                .unwrap()
        }

        fn create_authenticated_api_client(mock_server: &MockServer) -> ApiClient {
            ApiClient::builder()
                .api_base_url(&mock_server.uri())
                .credentials(create_authenticated_credentials())
                .build()
                .unwrap()
        }

        fn create_authenticated_test_api_client(mock_server: &MockServer) -> ApiClient {
            ApiClient::builder()
                .http_client(create_test_http_client())
                .api_base_url(&mock_server.uri())
                .credentials(create_authenticated_credentials())
                .build()
                .unwrap()
        }

        #[tokio::test]
        async fn with_default_client_and_no_credentials() {
            let mock_server = setup_mock_server().await;
            let client = create_anonymous_api_client(&mock_server);

            test_routes(&client, true, false).await;
        }

        #[tokio::test]
        async fn with_default_client_and_credentials() {
            let mock_server = setup_mock_server().await;
            let client = create_authenticated_api_client(&mock_server);

            test_routes(&client, false, false).await;
        }

        #[tokio::test]
        async fn with_test_client_and_no_credentials() {
            let mock_server = setup_mock_server().await;
            let client = create_anonymous_test_api_client(&mock_server);

            test_routes(&client, true, true).await;
        }

        #[tokio::test]
        async fn with_test_client_and_credentials() {
            let mock_server = setup_mock_server().await;
            let client = create_authenticated_test_api_client(&mock_server);

            test_routes(&client, false, true).await;
        }
    }

    mod define_api_client {
        use super::*;

        const TEST_API_TOKEN: &str = "some_token";
        const TEST_API_CLIENT_BASE_URL: &str = "https://test.api.client/api";

        define_api_client! {
            pub struct TestApiClient(TEST_API_CLIENT_BASE_URL);
        }

        impl TestApiClient {
            pub fn api_base_url(&self) -> &str {
                self.api_client.api_base_url()
            }
        }

        #[test]
        fn test_builder_with_default_base_url() {
            let test_api_client = TestApiClient::builder()
                .http_client(Client::default())
                .credentials(Credentials::from_api_token(TEST_API_TOKEN))
                .build();

            assert_eq!(test_api_client.api_base_url(), TEST_API_CLIENT_BASE_URL);
        }

        #[test]
        fn test_builder_with_custom_base_url() {
            let custom_api_base_url = "https://custom.api.client/api";
            let test_api_client = TestApiClient::builder()
                .api_base_url(custom_api_base_url)
                .build();

            assert_eq!(test_api_client.api_base_url(), custom_api_base_url);
        }

        #[test]
        fn test_builder_with_new() {
            let test_api_client = TestApiClientBuilder::new().build();

            assert_eq!(test_api_client.api_base_url(), TEST_API_CLIENT_BASE_URL);
        }

        #[test]
        fn test_default_client() {
            let test_api_client = TestApiClient::default();

            assert_eq!(test_api_client.api_base_url(), TEST_API_CLIENT_BASE_URL);
        }

        #[test]
        fn test_client_with_new() {
            let test_api_client = TestApiClient::new();

            assert_eq!(test_api_client.api_base_url(), TEST_API_CLIENT_BASE_URL);
        }
    }
}
