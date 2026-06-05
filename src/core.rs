//! Core types used across the [mini_exercism](crate) library.

use derive_builder::UninitializedFieldError;
use thiserror::Error;

use crate::http;

/// Credentials used to access the [Exercism](https://exercism.org) APIs.
///
/// # Examples
///
/// ```
/// use mini_exercism::core::Credentials;
///
/// let api_token = "some_token";
/// let credentials = Credentials::from_api_token(api_token);
///
/// assert_eq!(credentials.api_token(), api_token);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Credentials {
    api_token: String,
}

impl Credentials {
    /// Creates a new [Exercism](https://exercism.org) credentials wrapper from the given API token.
    pub fn from_api_token<T: Into<String>>(api_token: T) -> Self {
        Self { api_token: api_token.into() }
    }

    /// Accesses the [Exercism](https://exercism.org) API token.
    pub fn api_token(&self) -> &str {
        self.api_token.as_str()
    }
}

/// Result type used by the [mini_exercism](crate) library when an error can occur.
pub type Result<T, E = Error> = core::result::Result<T, E>;

/// Error type used by the [mini_exercism](crate) library.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    /// CLI config file could not be found (see [`get_cli_credentials`](crate::cli::get_cli_credentials))
    #[cfg(feature = "cli")]
    #[error(
        "Exercism CLI config file not found - perhaps CLI application is not installed or configured?"
    )]
    ConfigNotFound,

    /// I/O error reading CLI config file (see [`get_cli_credentials`](crate::cli::get_cli_credentials))
    #[cfg(feature = "cli")]
    #[error("could not read Exercism CLI config file: {0:?}")]
    ConfigReadError(#[from] std::io::Error),

    /// JSON error parsing CLI config file (see [`get_cli_credentials`](crate::cli::get_cli_credentials))
    #[cfg(feature = "cli")]
    #[error("failed to parse Exercism CLI config file: {0:?}")]
    ConfigParseError(#[from] serde_json::Error),

    /// CLI config file did not contain an API token (see [`get_cli_credentials`](crate::cli::get_cli_credentials))
    #[cfg(feature = "cli")]
    #[error("Exercism CLI config file did not contain an API token")]
    ApiTokenNotFoundInConfig,

    /// A call to a builder's `build` method failed
    #[error(transparent)]
    BuildFailed(#[from] BuildError),

    /// Error encountered while performing a request to an [Exercism](https://exercism.org) API
    #[error("error while performing API request: {0:?}")]
    ApiError(#[from] http::Error),

    /// Error encountered while performing a request to an [Exercism](https://exercism.org) API
    /// which persisted even after retried have been exhausted
    #[error("error while performing API request with retries: {0:?}")]
    ApiRetryError(anyhow::Error),
}

impl From<UninitializedFieldError> for Error {
    #[cfg_attr(coverage_nightly, coverage(off))]
    fn from(_value: UninitializedFieldError) -> Self {
        // This cannot occur in the crate's current code.
        unreachable!("all fields should have had default values")
    }
}

/// Type used when a builder error occurs.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum BuildError {
    /// Creation of an [HTTP client](http::Client) failed.
    #[error("http client creation failed: {0:?}")]
    HttpClientCreationFailed(#[from] http::Error),
}

impl From<http::middleware::Error> for Error {
    fn from(value: http::middleware::Error) -> Self {
        match value {
            http::middleware::Error::Middleware(err) => Self::ApiRetryError(err),
            http::middleware::Error::Reqwest(err) => err.into(),
        }
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use std::collections::HashMap;

    use assert_matches::assert_matches;
    use rstest::{fixture, rstest};

    use super::*;

    mod error {
        use super::*;

        #[derive(Debug, Error)]
        enum MiddlewareError {
            #[error("not found")]
            NotFound,
        }

        #[fixture]
        fn reqwest_builder_error() -> http::Error {
            // There's no way to create a `reqwest::Error` outside the reqwest crate,
            // so we'll have to trigger an actual error.
            let map_with_non_string_keys: HashMap<_, _> = [(true, 42), (false, 23)].into();
            http::Client::new()
                .get("/test")
                .json(&map_with_non_string_keys)
                .build()
                .unwrap_err()
        }

        mod from_reqwest_middleware_error_for_error {
            use super::*;

            #[test]
            fn middleware() {
                let middleware_error = MiddlewareError::NotFound;
                let middleware_error = http::middleware::Error::middleware(middleware_error);
                let error: Error = middleware_error.into();

                assert_matches!(error, Error::ApiRetryError(err) => {
                    assert_eq!(err.to_string(), "not found");
                });
            }

            #[rstest]
            fn reqwest(reqwest_builder_error: http::Error) {
                let error: Error = reqwest_builder_error.into();

                assert_matches!(error, Error::ApiError(err) => {
                    assert!(err.is_builder());
                });
            }
        }
    }
}
