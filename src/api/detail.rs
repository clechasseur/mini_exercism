use std::fmt::Display;

use derive_builder::UninitializedFieldError;
use serde::de::DeserializeOwned;

use crate::Result;
use crate::core::{BuildError, Credentials};
use crate::http;
use crate::http::middleware::{ClientBuilder, ClientWithMiddleware, RequestBuilder};
use crate::http::retry::after::{RetryAfterMiddleware, RetryAfterPolicy};
use crate::http::retry::policies::ExponentialBackoff;
use crate::http::{IntoUrl, Method};

pub const DEFAULT_MAX_RETRIES: u32 = 5;

#[derive(Debug)]
pub struct ApiClient {
    http_client: ClientWithMiddleware,
    api_base_url: String,
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

    pub fn request<U>(&self, method: Method, url: U) -> ApiRequestBuilder
    where
        U: Display,
    {
        ApiRequestBuilder::new(&self.http_client, method, self.api_url(url), &self.credentials)
    }

    pub fn get<U>(&self, url: U) -> ApiRequestBuilder
    where
        U: Display,
    {
        self.request(Method::GET, url)
    }

    fn api_url<U>(&self, url: U) -> String
    where
        U: Display,
    {
        format!("{}{url}", self.api_base_url)
    }
}

#[derive(Debug, Default)]
pub struct ApiClientBuilder {
    http_client: Option<http::Client>,
    retry_policy: Option<ExponentialBackoff>,
    api_base_url: Option<String>,
    credentials: Option<Credentials>,
}

impl ApiClientBuilder {
    pub fn http_client(&mut self, client: http::Client) -> &mut Self {
        self.http_client = Some(client);
        self
    }

    pub fn retry_policy(&mut self, policy: ExponentialBackoff) -> &mut Self {
        self.retry_policy = Some(policy);
        self
    }

    pub fn api_base_url(&mut self, url: &str) -> &mut Self {
        self.api_base_url = Some(url.trim_end_matches('/').into());
        self
    }

    pub fn credentials(&mut self, credentials: Credentials) -> &mut Self {
        self.credentials = Some(credentials);
        self
    }

    pub fn build(&mut self) -> Result<ApiClient> {
        let api_base_url = match self.api_base_url.clone() {
            Some(url) => url,
            None => return Err(UninitializedFieldError::new("api_base_url").into()),
        };
        let http_client = match self.http_client.clone() {
            Some(client) => client,
            None => Self::default_http_client()?,
        };
        let retry_policy = self.retry_policy.unwrap_or_else(Self::default_retry_policy);
        let http_client = Self::build_http_client(http_client, retry_policy);

        Ok(ApiClient { http_client, api_base_url, credentials: self.credentials.clone() })
    }

    fn default_http_client() -> Result<http::Client> {
        Ok(http::Client::builder().build().map_err(BuildError::from)?)
    }

    fn default_retry_policy() -> ExponentialBackoff {
        ExponentialBackoff::builder().build_with_max_retries(DEFAULT_MAX_RETRIES)
    }

    fn build_http_client(
        http_client: http::Client,
        retry_policy: ExponentialBackoff,
    ) -> ClientWithMiddleware {
        let retry_policy = RetryAfterPolicy::with_policy(retry_policy);
        ClientBuilder::new(http_client)
            .with(RetryAfterMiddleware::new_with_policy(retry_policy))
            .build()
    }
}

pub trait IntoQuery {
    fn into_query(self, request: RequestBuilder) -> RequestBuilder;
}

impl<V> IntoQuery for (&str, Option<V>)
where
    V: AsRef<str>,
{
    fn into_query(self, request: RequestBuilder) -> RequestBuilder {
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
    fn into_query(self, request: RequestBuilder) -> RequestBuilder {
        self.1
            .into_iter()
            .fold(request, |request, v| request.query(&[(self.0, v.as_ref())]))
    }
}

impl<Q> IntoQuery for Option<Q>
where
    Q: IntoQuery,
{
    fn into_query(self, request: RequestBuilder) -> RequestBuilder {
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
        if cond { self.build_query(query) } else { self }
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

impl QueryBuilder for RequestBuilder {
    fn build_query<Q>(self, query: Q) -> Self
    where
        Q: IntoQuery,
    {
        query.into_query(self)
    }
}

pub struct ApiRequestBuilder {
    request: RequestBuilder,
}

impl ApiRequestBuilder {
    pub fn new<U>(
        http_client: &ClientWithMiddleware,
        method: Method,
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
                error: ::std::option::Option<$crate::Error>,
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
                    to the API.

                    If not specified, a default client will be created.
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
                    use mini_exercism::api;
                    use mini_exercism::http::header::{HeaderMap, HeaderValue};

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
                    F: ::std::ops::FnOnce($crate::http::ClientBuilder) -> $crate::http::ClientBuilder
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
                    Sets the number of retries to attempt when performing requests to the API.

                    If not specified, the default policy is to retry requests up to five (5) times
                    if they fail with specific status codes (see
                    [`default_on_request_success`](crate::http::retry::default_on_request_success)
                    for details).
                "]
                pub fn num_retries(&mut self, value: u32) -> &mut Self {
                    if self.error.is_none() {
                        self.api_client_builder.retry_policy(
                            $crate::http::retry::policies::ExponentialBackoff::builder().build_with_max_retries(value),
                        );
                    }
                    self
                }

                #[doc = r"
                    Sets the [retry policy](crate::http::retry::RetryPolicy] to use to perform
                    requests to the API.

                    If not specified, the default policy is to retry requests up to five (5) times
                    if they fail with specific status codes (see
                    [`default_on_request_success`](crate::http::retry::default_on_request_success)
                    for details).
                "]
                pub fn retry_policy(&mut self, value: $crate::http::retry::policies::ExponentialBackoff) -> &mut Self {
                    if self.error.is_none() {
                        self.api_client_builder.retry_policy(value);
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
#[allow(clippy::too_many_arguments)]
mod tests {
    use super::*;

    #[cfg_attr(coverage_nightly, coverage(off))]
    mod api_client {
        use assert_matches::assert_matches;
        use rstest::{fixture, rstest};
        use serde::{Deserialize, Serialize};
        use strum::{AsRefStr, Display};
        use wiremock::matchers::{
            bearer_token, header_exists, method, path, query_param, query_param_is_missing,
        };
        use wiremock::{Mock, MockBuilder, MockServer, Request, Respond, ResponseTemplate};
        use wiremock_logical_matchers::not;

        use super::*;
        use crate::http::StatusCode;
        use crate::http::header::{HeaderMap, HeaderValue};

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
                if on { Self::on() } else { Self::off() }
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
            fn into_query(self, request: RequestBuilder) -> RequestBuilder {
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

        #[fixture]
        async fn mock_server(
            #[default(false)] anonymous: bool,
            #[default(false)] test_header: bool,
            #[default(false)] test_data_on: bool,
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

        #[fixture]
        fn http_client_with_test_header() -> http::Client {
            let mut default_headers = HeaderMap::new();
            default_headers.insert(TEST_HEADER, HeaderValue::from_static("any_value_will_do"));

            http::Client::builder()
                .default_headers(default_headers)
                .build()
                .unwrap()
        }

        #[fixture]
        fn authenticated_credentials() -> Credentials {
            Credentials::from_api_token(API_TOKEN)
        }

        #[fixture]
        fn api_client(
            #[default("")] api_base_url: &str,
            #[default(false)] anonymous: bool,
            #[default(false)] test_header: bool,
        ) -> ApiClient {
            let mut builder = ApiClient::builder();
            builder.api_base_url(api_base_url.as_ref());

            if !anonymous {
                builder.credentials(authenticated_credentials());
            }
            if test_header {
                builder.http_client(http_client_with_test_header());
            }

            builder.build().unwrap()
        }

        #[rstest]
        #[tokio::test]
        #[awt]
        async fn test_all(
            #[values(false, true)] expected_anonymous: bool,
            #[values(false, true)] expected_test_header: bool,
            #[values(false, true)] expected_test_data_on: bool,
            #[values(false, true)] actual_anonymous: bool,
            #[values(false, true)] actual_test_header: bool,
            #[values(false, true)] actual_test_data_on: bool,
            #[future]
            #[with(expected_anonymous, expected_test_header, expected_test_data_on)]
            mock_server: MockServer,
        ) {
            let correct = (actual_anonymous, actual_test_header, actual_test_data_on)
                == (expected_anonymous, expected_test_header, expected_test_data_on);

            let client = api_client(&mock_server.uri(), actual_anonymous, actual_test_header);

            let actual_test_data = TestData::get(actual_test_data_on);
            let opt_actual_test_data =
                if actual_test_data_on { Some(actual_test_data.clone()) } else { None };

            let from_request = client
                .request(Method::GET, ROUTE)
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
                    "Test for ({expected_anonymous}, {expected_test_header}, {expected_test_data_on}), permutation ({actual_anonymous}, {actual_test_header}, {actual_test_data_on})"
                );
                assert_matches!(
                    from_get,
                    Ok(response) if response.status() == StatusCode::OK,
                    "Test for ({expected_anonymous}, {expected_test_header}, {expected_test_data_on}), permutation ({actual_anonymous}, {actual_test_header}, {actual_test_data_on})"
                );

                let from_request: TestOutput = client
                    .request(Method::GET, ROUTE)
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
                    expected, from_request,
                    "Test for ({expected_anonymous}, {expected_test_header}, {expected_test_data_on}), permutation ({actual_anonymous}, {actual_test_header}, {actual_test_data_on})"
                );
                assert_eq!(
                    expected, from_get,
                    "Test for ({expected_anonymous}, {expected_test_header}, {expected_test_data_on}), permutation ({actual_anonymous}, {actual_test_header}, {actual_test_data_on})"
                );
            } else {
                assert_matches!(
                    from_request,
                    Err(crate::Error::ApiError(err)) if err.is_status() => {
                        assert_matches!(err.status(), Some(StatusCode::NOT_FOUND));
                    },
                    "Test for ({expected_anonymous}, {expected_test_header}, {expected_test_data_on}), permutation ({actual_anonymous}, {actual_test_header}, {actual_test_data_on})"
                );
                assert_matches!(
                    from_get,
                    Err(crate::Error::ApiError(err)) if err.is_status() => {
                        assert_matches!(err.status(), Some(StatusCode::NOT_FOUND));
                    },
                    "Test for ({expected_anonymous}, {expected_test_header}, {expected_test_data_on}), permutation ({actual_anonymous}, {actual_test_header}, {actual_test_data_on})"
                );
            }
        }

        #[test]
        #[should_panic]
        fn test_without_api_base_url() {
            let _ = ApiClient::builder().build();
        }

        mod retries {
            use std::sync::Mutex;
            use std::time::Duration;

            use super::*;
            use crate::http::header::RETRY_AFTER;

            #[derive(Debug)]
            struct ThrottledResponse {
                throttled_count: Mutex<usize>,
                throttling_status_code: StatusCode,
                throttling_retry_after: Option<Duration>,
                response: ResponseTemplate,
            }

            impl ThrottledResponse {
                fn new(
                    throttled_count: usize,
                    throttling_status_code: StatusCode,
                    throttling_retry_after: Option<Duration>,
                    response: ResponseTemplate,
                ) -> Self {
                    Self {
                        throttled_count: Mutex::new(throttled_count),
                        throttling_status_code,
                        throttling_retry_after,
                        response,
                    }
                }
            }

            impl Respond for ThrottledResponse {
                fn respond(&self, _request: &Request) -> ResponseTemplate {
                    let mut lock = self.throttled_count.lock().unwrap();
                    if *lock > 0 {
                        *lock -= 1;
                        let mut response = ResponseTemplate::new(self.throttling_status_code);
                        if let Some(retry_after) = self.throttling_retry_after {
                            response = response
                                .append_header(RETRY_AFTER, retry_after.as_secs().to_string());
                        }
                        response
                    } else {
                        self.response.clone()
                    }
                }
            }

            #[rstest]
            #[case::request_timeout(StatusCode::REQUEST_TIMEOUT, None)]
            #[case::too_many_requests(StatusCode::TOO_MANY_REQUESTS, None)]
            #[case::internal_server_error(StatusCode::INTERNAL_SERVER_ERROR, None)]
            #[case::with_retry_after_header(
                StatusCode::TOO_MANY_REQUESTS,
                Some(Duration::from_secs(1))
            )]
            #[awt]
            #[tokio::test]
            async fn for_status(
                #[case] throttling_status_code: StatusCode,
                #[case] throttling_retry_after: Option<Duration>,
            ) {
                let mock_server = MockServer::start().await;

                Mock::given(method("GET"))
                    .and(path(ROUTE))
                    .and(not(header_exists("authorization")))
                    .respond_with(ThrottledResponse::new(
                        2,
                        throttling_status_code,
                        throttling_retry_after,
                        ResponseTemplate::new(StatusCode::OK).set_body_json(TestOutput::default()),
                    ))
                    .mount(&mock_server)
                    .await;

                let client = ApiClient::builder()
                    .api_base_url(&mock_server.uri())
                    .build()
                    .unwrap();

                let result = client.get(ROUTE).send().await;
                assert_matches!(result, Ok(response) if response.status() == StatusCode::OK);
            }

            #[tokio::test]
            async fn throttled_too_many_times() {
                let mock_server = MockServer::start().await;

                Mock::given(method("GET"))
                    .and(path(ROUTE))
                    .and(not(header_exists("authorization")))
                    .respond_with(ThrottledResponse::new(
                        2,
                        StatusCode::TOO_MANY_REQUESTS,
                        None,
                        ResponseTemplate::new(StatusCode::OK).set_body_json(TestOutput::default()),
                    ))
                    .mount(&mock_server)
                    .await;

                let client = ApiClient::builder()
                    .api_base_url(&mock_server.uri())
                    .retry_policy(ExponentialBackoff::builder().build_with_max_retries(1))
                    .build()
                    .unwrap();

                let result = client.get(ROUTE).send().await;
                assert_matches!(result, Err(crate::Error::ApiError(err)) => {
                    assert_matches!(err.status(), Some(StatusCode::TOO_MANY_REQUESTS));
                });
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
            #[cfg_attr(coverage_nightly, coverage(off))]
            pub fn api_base_url(&self) -> &str {
                self.api_client.api_base_url()
            }
        }

        struct CannotBeAHeaderValue;

        impl TryInto<HeaderValue> for CannotBeAHeaderValue {
            type Error = InvalidHeaderValue;

            #[cfg_attr(coverage_nightly, coverage(off))]
            fn try_into(self) -> std::result::Result<HeaderValue, Self::Error> {
                HeaderValue::from_bytes(&[20])
            }
        }

        #[cfg_attr(coverage_nightly, coverage(off))]
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
            fn test_num_retries() {
                let result = TestApiClient::builder().num_retries(2).build();

                assert!(result.is_ok());
            }

            #[test]
            fn test_retry_policy() {
                let result = TestApiClient::builder()
                    .retry_policy(ExponentialBackoff::builder().build_with_max_retries(2))
                    .build();

                assert!(result.is_ok());
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
                    assert!(!format!("{builder:?}").is_empty());
                }
            }
        }

        #[cfg_attr(coverage_nightly, coverage(off))]
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
                    assert!(!format!("{client:?}").is_empty());
                }
            }
        }
    }
}
