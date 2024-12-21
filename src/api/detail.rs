use std::fmt::Display;

use derive_builder::Builder;
use serde::de::DeserializeOwned;

use crate::core::{BuildError, Credentials};
use crate::http;
use crate::http::IntoUrl;
use crate::Result;

#[derive(Debug, Builder)]
#[builder(derive(Debug), build_fn(error = "crate::Error"))]
pub struct ApiClient {
    #[builder(default = "self.default_http_client()?")]
    http_client: http::Client,

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

    pub fn request<U>(&self, method: http::Method, url: U) -> ApiRequestBuilder
    where
        U: Display,
    {
        ApiRequestBuilder::new(&self.http_client, method, self.api_url(url), &self.credentials)
    }

    pub fn get<U>(&self, url: U) -> ApiRequestBuilder
    where
        U: Display,
    {
        self.request(http::Method::GET, url)
    }

    fn api_url<U>(&self, url: U) -> String
    where
        U: Display,
    {
        format!("{}{}", self.api_base_url, url)
    }
}

impl ApiClientBuilder {
    pub fn api_base_url(&mut self, url: &str) -> &mut Self {
        self.api_base_url = Some(url.trim_end_matches('/').into());
        self
    }

    fn default_http_client(&self) -> Result<http::Client> {
        Ok(http::Client::builder().build().map_err(BuildError::from)?)
    }
}

pub trait IntoQuery {
    fn into_query(self, request: http::RequestBuilder) -> http::RequestBuilder;
}

impl<V> IntoQuery for (&str, Option<V>)
where
    V: AsRef<str>,
{
    fn into_query(self, request: http::RequestBuilder) -> http::RequestBuilder {
        match self.1 {
            Some(param) => request.query(&[(self.0, param.as_ref())]),
            None => request,
        }
    }
}

impl<V> IntoQuery for (&str, Vec<V>)
where
    V: AsRef<str>,
{
    fn into_query(self, request: http::RequestBuilder) -> http::RequestBuilder {
        self.1
            .into_iter()
            .fold(request, |request, v| request.query(&[(self.0, v.as_ref())]))
    }
}

impl<Q> IntoQuery for Option<Q>
where
    Q: IntoQuery,
{
    fn into_query(self, request: http::RequestBuilder) -> http::RequestBuilder {
        match self {
            Some(query) => query.into_query(request),
            None => request,
        }
    }
}

pub trait QueryBuilder: Sized {
    fn build_query<Q>(self, query: Q) -> Self
    where
        Q: IntoQuery;

    fn build_query_if<Q>(self, cond: bool, query: Q) -> Self
    where
        Q: IntoQuery,
    {
        if cond {
            self.build_query(query)
        } else {
            self
        }
    }

    fn build_joined_query<V>(self, key: &str, values: Vec<V>) -> Self
    where
        V: AsRef<str>,
    {
        if !values.is_empty() {
            let values = values
                .iter()
                .map(|v| v.as_ref())
                .collect::<Vec<_>>()
                .join(" ");

            self.build_query((key, Some(values.as_str())))
        } else {
            self
        }
    }
}

impl QueryBuilder for http::RequestBuilder {
    fn build_query<Q>(self, query: Q) -> Self
    where
        Q: IntoQuery,
    {
        query.into_query(self)
    }
}

pub struct ApiRequestBuilder {
    request: http::RequestBuilder,
}

impl ApiRequestBuilder {
    pub fn new<U>(
        http_client: &http::Client,
        method: http::Method,
        url: U,
        credentials: &Option<Credentials>,
    ) -> Self
    where
        U: IntoUrl,
    {
        let mut request = http_client.request(method, url);
        if let Some(credentials) = credentials {
            request = request.bearer_auth(credentials.api_token());
        }
        Self { request }
    }

    pub fn query<Q>(self, query: Q) -> Self
    where
        Q: IntoQuery,
    {
        Self { request: query.into_query(self.request) }
    }

    pub async fn send(self) -> Result<http::Response> {
        Ok(self.request.send().await?.error_for_status()?)
    }

    pub async fn execute<R>(self) -> Result<R>
    where
        R: DeserializeOwned,
    {
        Ok(self.send().await?.json().await?)
    }
}

macro_rules! define_api_client {
    (
        $(#[$attr:meta])*
        $vis:vis struct $api_name:ident($base_url:expr);
    ) => {
        paste::paste! {
            $(#[$attr])*
            #[derive(Debug, Clone)]
            $vis struct $api_name {
                api_client: ::std::sync::Arc<$crate::api::detail::ApiClient>,
            }

            impl $api_name {
                #[doc = r"
                    Creates a new [`" $api_name r"`] with default values.

                    This is the same as calling `" $api_name r"::builder().build()`.
                "]
                pub fn new() -> $crate::Result<Self> {
                    Self::builder().build()
                }

                #[doc = r"
                    Returns a [`" $api_name r"Builder`] that can be used to
                    create an API client instance.
                "]
                pub fn builder() -> [<$api_name Builder>] {
                    [<$api_name Builder>]::default()
                }
            }

            #[doc = r"
                Builder for the [`" $api_name r"`] type.

                To create a builder instance, call [`" $api_name r"::builder`].

                Because all fields have default values, it is legal to create
                an instance of this builder and simply call [`build`](" $api_name r"Builder::build).
            "]
            #[derive(Debug)]
            $vis struct [<$api_name Builder>] {
                api_client_builder: $crate::api::detail::ApiClientBuilder,
                error: Option<$crate::Error>,
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
                    Sets the [HTTP client](crate::http::Client) to use to perform requests
                    to the API. If not specified, a default client will be created.
                "]
                pub fn http_client(&mut self, value: $crate::http::Client) -> &mut Self {
                    if self.error.is_none() {
                        self.api_client_builder.http_client(value);
                    }
                    self
                }

                #[doc = r#"
                    Builds the [HTTP client](crate::http::Client) to use to perform requests
                    to the API using a [builder](crate::http::ClientBuilder).

                    # Examples

                    ```no_run
                    use http::{HeaderMap, HeaderValue};
                    use mini_exercism::api;

                    async fn get_client() -> anyhow::Result<api::v2::Client> {
                        Ok(api::v2::Client::builder()
                            .build_http_client(|builder| {
                                let mut default_headers = HeaderMap::new();
                                default_headers.insert(
                                    "x-some-header",
                                    HeaderValue::from_static("some-header-value"),
                                );

                                builder.default_headers(default_headers)
                            })
                            .build()?)
                    }
                    ```
                "#]
                pub fn build_http_client<F>(&mut self, value_f: F) -> &mut Self
                where
                    F: FnOnce($crate::http::ClientBuilder) -> $crate::http::ClientBuilder
                {
                    if self.error.is_none() {
                        match value_f($crate::http::Client::builder()).build() {
                            Ok(client) => {
                                self.api_client_builder.http_client(client);
                            },
                            Err(err) => {
                                self.error = Some($crate::core::BuildError::from(err).into());
                            },
                        }
                    }
                    self
                }

                #[doc = r"
                    Sets the base URL to use to connect to the API.

                    Normally, this is set to the default value ([`" $base_url r"`])
                    when the builder is created and should not be changed.
                "]
                pub fn api_base_url(&mut self, value: &str) -> &mut Self {
                    if self.error.is_none() {
                        self.api_client_builder.api_base_url(value);
                    }
                    self
                }

                #[doc = r"
                    Sets the [`Credentials`](crate::core::Credentials) to use to
                    connect to the API.

                    If not specified, requests will be performed anonymously.
                "]
                pub fn credentials(&mut self, value: $crate::core::Credentials) -> &mut Self {
                    if self.error.is_none() {
                        self.api_client_builder.credentials(value);
                    }
                    self
                }

                #[doc = "Builds a new [`" $api_name "`] instance using the parameters of this builder."]
                pub fn build(&mut self) -> $crate::Result<$api_name> {
                    match self.error.take() {
                        None => Ok($api_name {
                            api_client: ::std::sync::Arc::new(self.api_client_builder.build()?),
                        }),
                        Some(err) => Err(err),
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
                    Self { api_client_builder, error: None }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod api_client {
        use assert_matches::assert_matches;
        use itertools::iproduct;
        use serde::{Deserialize, Serialize};
        use strum::{AsRefStr, Display};
        use wiremock::matchers::{
            bearer_token, header_exists, method, path, query_param, query_param_is_missing,
        };
        use wiremock::{Mock, MockBuilder, MockServer, ResponseTemplate};
        use wiremock_logical_matchers::not;

        use super::*;
        use crate::http::header::{HeaderMap, HeaderValue};
        use crate::http::StatusCode;

        const ROUTE: &str = "/";
        const API_TOKEN: &str = "some_api_token";
        const TEST_HEADER: &str = "x-mini_exercism-test";

        #[derive(Debug, Copy, Clone, PartialEq, Eq, Display, AsRefStr)]
        #[strum(serialize_all = "snake_case")]
        enum TestEnum {
            ValueA,
            ValueB,
            ValueC,
        }

        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        struct TestData {
            pub name: Option<String>,
            pub test: bool,
            pub values: Vec<TestEnum>,
            pub joined: Vec<TestEnum>,
        }

        impl TestData {
            fn get(on: bool) -> Self {
                if on {
                    Self::on()
                } else {
                    Self::off()
                }
            }

            fn on() -> Self {
                Self {
                    name: Some("clechasseur".into()),
                    test: true,
                    values: vec![TestEnum::ValueA, TestEnum::ValueB, TestEnum::ValueC],
                    joined: vec![TestEnum::ValueA, TestEnum::ValueB, TestEnum::ValueC],
                }
            }

            fn off() -> Self {
                Self::default()
            }

            #[must_use]
            fn add_to_mock(&self, mut mock: MockBuilder) -> MockBuilder {
                mock = match &self.name {
                    Some(name) => mock.and(query_param("name", name)),
                    None => mock.and(query_param_is_missing("name")),
                };

                mock = if self.test {
                    mock.and(not(query_param_is_missing("test")))
                } else {
                    mock.and(query_param_is_missing("test"))
                };

                if !self.values.is_empty() {
                    for value in &self.values {
                        mock = mock.and(query_param("values[]", value.as_ref()));
                    }
                } else {
                    mock = mock.and(query_param_is_missing("values[]"));
                }

                mock = if !self.joined.is_empty() {
                    let values = self
                        .joined
                        .iter()
                        .map(|v| v.as_ref())
                        .collect::<Vec<_>>()
                        .join(" ");

                    mock.and(query_param("joined", values))
                } else {
                    mock.and(query_param_is_missing("joined"))
                };

                mock
            }
        }

        impl IntoQuery for TestData {
            fn into_query(self, request: http::RequestBuilder) -> http::RequestBuilder {
                request
                    .build_query(("name", self.name))
                    .build_query_if(self.test, ("test", Some("1")))
                    .build_query(("values[]", self.values))
                    .build_joined_query("joined", self.joined)
            }
        }

        #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
        struct TestOutput {
            pub message: String,
        }

        impl Default for TestOutput {
            fn default() -> Self {
                Self { message: "test message".into() }
            }
        }

        fn test_permutations() -> impl Iterator<Item = (bool, bool, bool)> {
            iproduct!(
                [false, true], // anonymous
                [false, true], // test_header
                [false, true]  // test_data on?
            )
        }

        async fn setup_mock_server(
            anonymous: bool,
            test_header: bool,
            test_data_on: bool,
        ) -> MockServer {
            let mock_server = MockServer::start().await;

            let mut mock = Mock::given(method("GET")).and(path(ROUTE));

            mock = if anonymous {
                mock.and(not(header_exists("authorization")))
            } else {
                mock.and(bearer_token(API_TOKEN))
            };

            mock = if test_header {
                mock.and(header_exists(TEST_HEADER))
            } else {
                mock.and(not(header_exists(TEST_HEADER)))
            };

            mock = TestData::get(test_data_on).add_to_mock(mock);

            mock.respond_with(
                ResponseTemplate::new(StatusCode::OK).set_body_json(TestOutput::default()),
            )
            .mount(&mock_server)
            .await;

            mock_server
        }

        fn create_test_http_client() -> http::Client {
            let mut default_headers = HeaderMap::new();
            default_headers.insert(TEST_HEADER, HeaderValue::from_static("any_value_will_do"));

            http::Client::builder()
                .default_headers(default_headers)
                .build()
                .unwrap()
        }

        fn create_authenticated_credentials() -> Credentials {
            Credentials::from_api_token(API_TOKEN)
        }

        fn create_api_client<U>(api_base_url: &U, anonymous: bool, test_header: bool) -> ApiClient
        where
            U: AsRef<str>,
        {
            let mut builder = ApiClient::builder();
            builder.api_base_url(api_base_url.as_ref());

            if !anonymous {
                builder.credentials(create_authenticated_credentials());
            }
            if test_header {
                builder.http_client(create_test_http_client());
            }

            builder.build().unwrap()
        }

        async fn test_routes<U>(
            api_base_url: &U,
            expected_anonymous: bool,
            expected_test_header: bool,
            expected_test_data_on: bool,
        ) where
            U: AsRef<str>,
        {
            let permutations = test_permutations();

            for (actual_anonymous, actual_test_header, actual_test_data_on) in permutations {
                let correct = (actual_anonymous, actual_test_header, actual_test_data_on)
                    == (expected_anonymous, expected_test_header, expected_test_data_on);

                let client = create_api_client(api_base_url, actual_anonymous, actual_test_header);

                let actual_test_data = TestData::get(actual_test_data_on);
                let opt_actual_test_data =
                    if actual_test_data_on { Some(actual_test_data.clone()) } else { None };

                let from_request = client
                    .request(http::Method::GET, ROUTE)
                    .query(actual_test_data.clone())
                    .send()
                    .await;
                let from_get = client
                    .get(ROUTE)
                    .query(opt_actual_test_data.clone())
                    .send()
                    .await;

                if correct {
                    assert_matches!(
                        from_request,
                        Ok(response) if response.status() == StatusCode::OK,
                        "Test for ({}, {}, {}), permutation ({}, {}, {})",
                        expected_anonymous, expected_test_header, expected_test_data_on,
                        actual_anonymous, actual_test_header, actual_test_data_on
                    );
                    assert_matches!(
                        from_get,
                        Ok(response) if response.status() == StatusCode::OK,
                        "Test for ({}, {}, {}), permutation ({}, {}, {})",
                        expected_anonymous, expected_test_header, expected_test_data_on,
                        actual_anonymous, actual_test_header, actual_test_data_on
                    );

                    let from_request: TestOutput = client
                        .request(http::Method::GET, ROUTE)
                        .query(actual_test_data.clone())
                        .execute()
                        .await
                        .unwrap();
                    let from_get: TestOutput = client
                        .get(ROUTE)
                        .query(actual_test_data.clone())
                        .execute()
                        .await
                        .unwrap();

                    let expected = TestOutput::default();
                    assert_eq!(
                        expected,
                        from_request,
                        "Test for ({}, {}, {}), permutation ({}, {}, {})",
                        expected_anonymous,
                        expected_test_header,
                        expected_test_data_on,
                        actual_anonymous,
                        actual_test_header,
                        actual_test_data_on
                    );
                    assert_eq!(
                        expected,
                        from_get,
                        "Test for ({}, {}, {}), permutation ({}, {}, {})",
                        expected_anonymous,
                        expected_test_header,
                        expected_test_data_on,
                        actual_anonymous,
                        actual_test_header,
                        actual_test_data_on
                    );
                } else {
                    assert_matches!(
                        from_request,
                        Err(crate::Error::ApiError(err)) if err.is_status() => {
                            assert_matches!(err.status(), Some(StatusCode::NOT_FOUND));
                        },
                        "Test for ({}, {}, {}), permutation ({}, {}, {})",
                        expected_anonymous, expected_test_header, expected_test_data_on,
                        actual_anonymous, actual_test_header, actual_test_data_on
                    );
                    assert_matches!(
                        from_get,
                        Err(crate::Error::ApiError(err)) if err.is_status() => {
                            assert_matches!(err.status(), Some(StatusCode::NOT_FOUND));
                        },
                        "Test for ({}, {}, {}), permutation ({}, {}, {})",
                        expected_anonymous, expected_test_header, expected_test_data_on,
                        actual_anonymous, actual_test_header, actual_test_data_on
                    );
                }
            }
        }

        #[tokio::test]
        async fn test_all_permutations() {
            let permutations = test_permutations();

            for (anonymous, test_header, test_data_on) in permutations {
                let mock_server = setup_mock_server(anonymous, test_header, test_data_on).await;

                test_routes(&mock_server.uri(), anonymous, test_header, test_data_on).await;
            }
        }
    }

    mod define_api_client {
        use assert_matches::assert_matches;

        use super::*;
        use crate::http::header::{HeaderMap, HeaderValue, InvalidHeaderValue};

        const TEST_API_TOKEN: &str = "some_token";
        const TEST_API_CLIENT_BASE_URL: &str = "https://test.api.client/api";

        define_api_client! {
            struct TestApiClient(TEST_API_CLIENT_BASE_URL);
        }

        impl TestApiClient {
            pub fn api_base_url(&self) -> &str {
                self.api_client.api_base_url()
            }
        }

        struct CannotBeAHeaderValue;

        impl TryInto<HeaderValue> for CannotBeAHeaderValue {
            type Error = InvalidHeaderValue;

            fn try_into(self) -> std::result::Result<HeaderValue, Self::Error> {
                HeaderValue::from_bytes(&[20])
            }
        }

        mod builder {
            use super::*;

            #[test]
            fn test_default_base_url() {
                let test_api_client = TestApiClient::builder()
                    .http_client(http::Client::default())
                    .credentials(Credentials::from_api_token(TEST_API_TOKEN))
                    .build()
                    .unwrap();

                assert_eq!(test_api_client.api_base_url(), TEST_API_CLIENT_BASE_URL);
            }

            #[test]
            fn test_build_http_client() {
                let result = TestApiClient::builder()
                    .build_http_client(|builder| {
                        let mut default_headers = HeaderMap::new();
                        default_headers.insert("x-life", HeaderValue::from_static("42"));

                        builder.default_headers(default_headers)
                    })
                    .build();

                assert!(result.is_ok());
            }

            #[test]
            fn test_custom_base_url() {
                let custom_api_base_url = "https://custom.api.client/api";
                let test_api_client = TestApiClient::builder()
                    .api_base_url(custom_api_base_url)
                    .build()
                    .unwrap();

                assert_eq!(test_api_client.api_base_url(), custom_api_base_url);
            }

            #[test]
            fn test_build_error() {
                // This test might be a little brittle because it relies on the fact that setting
                // a user agent with invalid characters will cause a builder error. On one hand
                // it's documented as such, but on the other hand maybe they might break it some day?
                // In any case, if this test fails one day we'll need to revisit.
                let result = TestApiClient::builder()
                    .build_http_client(|builder| builder.user_agent(CannotBeAHeaderValue))
                    .build();

                assert_matches!(
                    result,
                    Err(crate::Error::BuildFailed(BuildError::HttpClientCreationFailed(_)))
                );
            }

            #[test]
            fn test_new() {
                let test_api_client = TestApiClientBuilder::new().build().unwrap();

                assert_eq!(test_api_client.api_base_url(), TEST_API_CLIENT_BASE_URL);
            }

            #[test]
            fn test_default() {
                let test_api_client = TestApiClientBuilder::default().build().unwrap();

                assert_eq!(test_api_client.api_base_url(), TEST_API_CLIENT_BASE_URL);
            }

            mod debug {
                use super::*;

                #[test]
                fn test_derive() {
                    // Note: this test is necessary because of a bug in cargo-tarpaulin, see
                    // https://github.com/xd009642/tarpaulin/issues/351#issuecomment-1722148936
                    let builder = TestApiClient::builder();
                    assert!(!format!("{:?}", builder).is_empty());
                }
            }
        }

        mod client {
            use super::*;

            #[test]
            fn test_new() {
                let test_api_client = TestApiClient::new().unwrap();

                assert_eq!(test_api_client.api_base_url(), TEST_API_CLIENT_BASE_URL);
            }

            mod debug {
                use super::*;

                #[test]
                fn test_derive() {
                    // Note: this test is necessary because of a bug in cargo-tarpaulin, see
                    // https://github.com/xd009642/tarpaulin/issues/351#issuecomment-1722148936
                    let client = TestApiClient::new();
                    assert!(!format!("{:?}", client).is_empty());
                }
            }
        }
    }
}
