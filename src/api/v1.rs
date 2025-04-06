//! Types and functions to interact with the [Exercism website](https://exercism.org) v1 API.

pub mod ping;
pub mod solution;
pub mod track;

use futures::future::Either;
use futures::stream;

use crate::http::StatusCode;
use crate::stream::{Bytes, Stream, StreamExt, TryStreamExt};
use crate::{Error, Result};

/// Default base URL for the [Exercism website](https://exercism.org) v1 API.
pub const DEFAULT_V1_API_BASE_URL: &str = "https://api.exercism.io/v1";

define_api_client! {
    /// Client for the [Exercism website](https://exercism.org) v1 API.
    ///
    /// This API is undocumented and is mostly used by the [Exercism CLI](https://exercism.org/docs/using/solving-exercises/working-locally)
    /// to download solution files.
    pub struct Client(DEFAULT_V1_API_BASE_URL);
}

impl Client {
    /// Returns information about a specific solution submitted by the user.
    ///
    /// The `solution_uuid` can be obtained from the mentoring interface, or
    /// returned by another API, like [`api::v2::Client::get_exercises`]
    /// (see [`Solution::uuid`]).
    ///
    /// # Notes
    ///
    /// Performing this request requires [`credentials`], otherwise a
    /// `401 Unauthorized` error will be returned.
    ///
    /// # Errors
    ///
    /// - [`ApiError`]: Error while fetching solution information from API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mini_exercism::api;
    /// use mini_exercism::core::Credentials;
    ///
    /// async fn get_solution_url(api_token: &str, solution_uuid: &str) -> anyhow::Result<String> {
    ///     let credentials = Credentials::from_api_token(api_token);
    ///     let client = api::v1::Client::builder()
    ///         .credentials(credentials)
    ///         .build()?;
    ///
    ///     Ok(client.get_solution(solution_uuid).await?.solution.url)
    /// }
    /// ```
    ///
    /// [`api::v2::Client::get_exercises`]: crate::api::v2::Client::get_exercises
    /// [`Solution::uuid`]: crate::api::v2::solution::Solution::uuid
    /// [`credentials`]: ClientBuilder::credentials
    /// [`ApiError`]: Error::ApiError
    pub async fn get_solution(&self, uuid: &str) -> Result<solution::Response> {
        self.api_client
            .get(format!("/solutions/{}", uuid))
            .execute()
            .await
    }

    /// Returns information about the latest solution submitted by the user for
    /// a given exercise.
    ///
    /// # Notes
    ///
    /// Performing this request requires [`credentials`](ClientBuilder::credentials),
    /// otherwise a `401 Unauthorized` error will be returned.
    ///
    /// # Errors
    ///
    /// - [`ApiError`]: Error while fetching solution information from API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mini_exercism::api;
    /// use mini_exercism::core::Credentials;
    ///
    /// async fn get_latest_solution_url(
    ///     api_token: &str,
    ///     track: &str,
    ///     exercise: &str,
    /// ) -> anyhow::Result<String> {
    ///     let credentials = Credentials::from_api_token(api_token);
    ///     let client = api::v1::Client::builder()
    ///         .credentials(credentials)
    ///         .build()?;
    ///
    ///     Ok(client
    ///         .get_latest_solution(track, exercise)
    ///         .await?
    ///         .solution
    ///         .url)
    /// }
    /// ```
    ///
    /// [`ApiError`]: Error::ApiError
    pub async fn get_latest_solution(
        &self,
        track: &str,
        exercise: &str,
    ) -> Result<solution::Response> {
        self.api_client
            .get("/solutions/latest")
            .query(("track_id", Some(track)))
            .query(("exercise_id", Some(exercise)))
            .execute()
            .await
    }

    /// Returns the contents of a specific file that is part of a solution.
    ///
    /// # Arguments
    ///
    /// - `solution_uuid` - [UUID](solution::Solution::uuid) of the solution containing the file.
    /// - `file_path` - Path to the file, as returned in [`solution::Solution::files`].
    ///
    /// # Notes
    ///
    /// - Performing this request requires [`credentials`](ClientBuilder::credentials),
    ///   otherwise a `401 Unauthorized` error will be returned.
    /// - If the API call to fetch file content fails, this method will return a [`Stream`]
    ///   containing a single [`ApiError`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::io::Write;
    ///
    /// use mini_exercism::api;
    /// use mini_exercism::core::Credentials;
    /// use mini_exercism::stream::StreamExt;
    ///
    /// async fn get_file_content(
    ///     api_token: &str,
    ///     track: &str,
    ///     exercise: &str,
    ///     file: &str,
    /// ) -> anyhow::Result<String> {
    ///     let credentials = Credentials::from_api_token(api_token);
    ///     let client = api::v1::Client::builder()
    ///         .credentials(credentials)
    ///         .build()?;
    ///
    ///     let solution = client.get_latest_solution(track, exercise).await?.solution;
    ///     let mut file_response = client.get_file(&solution.uuid, file).await;
    ///     let mut file_content: Vec<u8> = Vec::new();
    ///     while let Some(bytes) = file_response.next().await {
    ///         file_content.write_all(&bytes?)?;
    ///     }
    ///
    ///     Ok(String::from_utf8(file_content).expect("File should be valid UTF-8"))
    /// }
    /// ```
    ///
    /// [`ApiError`]: Error::ApiError
    pub async fn get_file(
        &self,
        solution_uuid: &str,
        file_path: &str,
    ) -> impl Stream<Item = Result<Bytes>> {
        let result = self
            .api_client
            .get(format!("/solutions/{}/files/{}", solution_uuid, file_path))
            .send()
            .await;

        // The result of `stream::once` is not `Unpin`, so calling `boxed()` will make sure it's
        // possible for callers to use `next()` on the returned `Stream` without pinning it first.
        match result {
            Ok(response) => Either::Left(response.bytes_stream().map_err(|err| err.into())),
            Err(error) => Either::Right(stream::once(async { Err(error) }).boxed()),
        }
    }

    /// Returns information about a language track.
    ///
    /// # Notes
    ///
    /// Perhaps strangely, performing this request requires [`credentials`](ClientBuilder::credentials),
    /// otherwise a `401 Unauthorized` error will be returned.
    ///
    /// # Errors
    ///
    /// - [`ApiError`]: Error while fetching track information from API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mini_exercism::api;
    /// use mini_exercism::api::v1::track::Track;
    /// use mini_exercism::core::Credentials;
    ///
    /// async fn get_language_track_details(api_token: &str, track: &str) -> anyhow::Result<Track> {
    ///     let credentials = Credentials::from_api_token(api_token);
    ///     let client = api::v1::Client::builder()
    ///         .credentials(credentials)
    ///         .build()?;
    ///
    ///     Ok(client.get_track(track).await?.track)
    /// }
    /// ```
    ///
    /// [`ApiError`]: Error::ApiError
    pub async fn get_track(&self, track: &str) -> Result<track::Response> {
        self.api_client
            .get(format!("/tracks/{}", track))
            .execute()
            .await
    }

    /// Validates the token used to perform API requests.
    ///
    /// If the API token is invalid or if the query is performed without [`credentials`],
    /// the API will return `401 Unauthorized` and this method will return `false`.
    /// If another HTTP, error is returned by the API, this method will return an [`ApiError`].
    ///
    /// # Errors
    ///
    /// - [`ApiError`]: Error while validating API token (other than `401 Unauthorized`)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mini_exercism::api;
    /// use mini_exercism::core::Credentials;
    ///
    /// async fn is_api_token_valid(api_token: &str) -> bool {
    ///     let credentials = Credentials::from_api_token(api_token);
    ///     match api::v1::Client::builder().credentials(credentials).build() {
    ///         Ok(client) => client.validate_token().await.unwrap_or(false),
    ///         Err(_) => false,
    ///     }
    /// }
    /// ```
    ///
    /// [`credentials`]: ClientBuilder::credentials
    /// [`ApiError`]: Error::ApiError
    pub async fn validate_token(&self) -> Result<bool> {
        // This API call returns a payload, but it doesn't really contain useful information:
        // if the token is invalid, 401 will be returned.
        let response = self.api_client.get("/validate_token").send().await;

        match response {
            Ok(_) => Ok(true),
            Err(Error::ApiError(error)) if error.status() == Some(StatusCode::UNAUTHORIZED) => {
                Ok(false)
            },
            Err(error) => Err(error),
        }
    }

    /// Sends a "ping" to the server to determine if service is up and available.
    ///
    /// The call returns information about the website and database.
    ///
    /// # Notes
    ///
    /// - This call does not require [`credentials`], but works anyway if they are provided.
    /// - As of this writing, the [current implementation] of this endpoint always return `true`
    ///   as status for all components. It makes sense if you think about it: if the database
    ///   or the Rails server misbehave, then the API would be inaccessible anyway 😅 It also
    ///   means that if the service is actually down, this method will simply return an [`ApiError`].
    ///
    /// # Errors
    ///
    /// - [`ApiError`]: Error while pinging API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mini_exercism::api;
    /// use mini_exercism::core::Credentials;
    ///
    /// async fn report_service_status() -> anyhow::Result<()> {
    ///     let client = api::v1::Client::new()?;
    ///
    ///     let service_status = client.ping().await?.status;
    ///     println!(
    ///         "Status: website: {}, database: {}",
    ///         service_status.website, service_status.database,
    ///     );
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// [`credentials`]: ClientBuilder::credentials
    /// [current implementation]: https://github.com/exercism/website/blob/2580b8fa2b13cad7aa7e8a877551bbd8552bee8b/app/controllers/api/v1/ping_controller.rb
    /// [`ApiError`]: Error::ApiError
    pub async fn ping(&self) -> Result<ping::Response> {
        self.api_client.get("/ping").execute().await
    }
}
