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
    #[cfg_attr(any(nightly_rustc, docsrs), doc(cfg(feature = "cli")))]
    #[error("Exercism CLI config file not found - perhaps CLI application is not installed or configured?")]
    ConfigNotFound,

    /// I/O error reading CLI config file (see [`get_cli_credentials`](crate::cli::get_cli_credentials))
    #[cfg(feature = "cli")]
    #[cfg_attr(any(nightly_rustc, docsrs), doc(cfg(feature = "cli")))]
    #[error("could not read Exercism CLI config file: {0:?}")]
    ConfigReadError(#[from] std::io::Error),

    /// JSON error parsing CLI config file (see [`get_cli_credentials`](crate::cli::get_cli_credentials))
    #[cfg(feature = "cli")]
    #[cfg_attr(any(nightly_rustc, docsrs), doc(cfg(feature = "cli")))]
    #[error("failed to parse Exercism CLI config file: {0:?}")]
    ConfigParseError(#[from] serde_json::Error),

    /// CLI config file did not contain an API token (see [`get_cli_credentials`](crate::cli::get_cli_credentials))
    #[cfg(feature = "cli")]
    #[cfg_attr(any(nightly_rustc, docsrs), doc(cfg(feature = "cli")))]
    #[error("Exercism CLI config file did not contain an API token")]
    ApiTokenNotFoundInConfig,

    /// A call to a builder's `build` method failed.
    #[error(transparent)]
    BuildFailed(#[from] BuildError),

    /// Error encountered while performing a request to an [Exercism](https://exercism.org) API
    #[error("error while performing API request: {0:?}")]
    ApiError(#[from] http::Error),
}

impl From<UninitializedFieldError> for Error {
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
