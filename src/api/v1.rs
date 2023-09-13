//! Types and functions to interact with the [Exercism website](https://exercism.org) v1 API.

use bytes::Bytes;
use futures::future::Either;
use futures::{stream, Stream, StreamExt, TryStreamExt};
use reqwest::StatusCode;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::core::Result;

/// Default base URL for the [Exercism website](https://exercism.org) v1 API.
pub const DEFAULT_V1_API_BASE_URL: &str = "https://api.exercism.io/v1";

define_api_client! {
    /// Client for the [Exercism website](https://exercism.org) v1 API. This API is undocumented
    /// and is mostly used by the [Exercism CLI](https://exercism.org/docs/using/solving-exercises/working-locally)
    /// to download solution files.
    pub struct Client(DEFAULT_V1_API_BASE_URL);
}

impl Client {
    /// Returns information about a specific solution submitted by the user.
    ///
    /// # Arguments
    ///
    /// - `uuid` - UUID of the solution to fetch. This can be provided by the mentoring
    ///            interface, or returned by another API, like
    ///            [`api::v2::Client::get_exercises`](crate::api::v2::Client::get_exercises)
    ///            (see [`Solution::uuid`](crate::api::v2::Solution::uuid)).
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
    /// async fn get_solution_url(api_token: &str, solution_uuid: &str) -> anyhow::Result<String> {
    ///     let credentials = Credentials::from_api_token(api_token);
    ///     let client = api::v1::Client::builder().credentials(credentials).build();
    ///
    ///     anyhow::Ok(client.get_solution(solution_uuid).await?.solution.url)
    /// }
    /// ```
    ///
    /// [`ApiError`]: crate::core::Error#variant.ApiError
    pub async fn get_solution(&self, uuid: &str) -> Result<SolutionResponse> {
        self.get(format!("/solutions/{}", uuid).as_str(), None)
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
    ///     let client = api::v1::Client::builder().credentials(credentials).build();
    ///
    ///     anyhow::Ok(
    ///         client
    ///             .get_latest_solution(track, exercise)
    ///             .await?
    ///             .solution
    ///             .url,
    ///     )
    /// }
    /// ```
    ///
    /// [`ApiError`]: crate::core::Error#variant.ApiError
    pub async fn get_latest_solution(
        &self,
        track: &str,
        exercise: &str,
    ) -> Result<SolutionResponse> {
        let query = [("track_id", track), ("exercise_id", exercise)];
        self.get("/solutions/latest", Some(&query)).await
    }

    /// Returns the contents of a specific file that is part of a solution.
    ///
    /// # Arguments
    ///
    /// - `solution_uuid` - [UUID](Solution::uuid) of the solution containing the file.
    /// - `file_path` - Path to the file, as returned in [`Solution::files`].
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
    /// use futures::StreamExt;
    /// use mini_exercism::api;
    /// use mini_exercism::core::Credentials;
    ///
    /// async fn get_file_content(
    ///     api_token: &str,
    ///     track: &str,
    ///     exercise: &str,
    ///     file: &str,
    /// ) -> anyhow::Result<String> {
    ///     let credentials = Credentials::from_api_token(api_token);
    ///     let client = api::v1::Client::builder().credentials(credentials).build();
    ///
    ///     let solution = client.get_latest_solution(track, exercise).await?.solution;
    ///     let mut file_response = client.get_file(&solution.uuid, file).await;
    ///     let mut file_content: Vec<u8> = Vec::new();
    ///     while let Some(bytes) = file_response.next().await {
    ///         file_content.write_all(&bytes?)?;
    ///     }
    ///
    ///     anyhow::Ok(String::from_utf8(file_content).expect("File should be valid UTF-8"))
    /// }
    /// ```
    ///
    /// [`ApiError`]: crate::core::Error#variant.ApiError
    pub async fn get_file(
        &self,
        solution_uuid: &str,
        file_path: &str,
    ) -> impl Stream<Item = Result<Bytes>> {
        let result = self
            .api_client
            .get(format!("/solutions/{}/files/{}", solution_uuid, file_path).as_str())
            .send()
            .await
            .and_then(|response| response.error_for_status());

        // The result of `stream::once` is not `Unpin`, so calling `boxed()` will make sure it's
        // possible for callers to use `next()` on the returned `Stream` without pinning it first.
        match result {
            Ok(response) => Either::Left(response.bytes_stream().map_err(|err| err.into())),
            Err(error) => Either::Right(stream::once(async { Err(error.into()) }).boxed()),
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
    /// use mini_exercism::api::v1::SolutionTrack;
    /// use mini_exercism::core::Credentials;
    ///
    /// async fn get_language_track_details(
    ///     api_token: &str,
    ///     track: &str,
    /// ) -> anyhow::Result<SolutionTrack> {
    ///     let credentials = Credentials::from_api_token(api_token);
    ///     let client = api::v1::Client::builder().credentials(credentials).build();
    ///
    ///     anyhow::Ok(client.get_track(track).await?.track)
    /// }
    /// ```
    ///
    /// [`ApiError`]: crate::core::Error#variant.ApiError
    pub async fn get_track(&self, track: &str) -> Result<TrackResponse> {
        self.get(format!("/tracks/{}", track).as_str(), None).await
    }

    /// Validates the API token used to perform API requests. If the API token is invalid or
    /// if the query is performed without [`credentials`](ClientBuilder::credentials), the
    /// API will return `401 Unauthorized` and this method will return `false`. If another HTTP
    /// error is returned by the API, this method will return an [`ApiError`].
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
    ///     let client = api::v1::Client::builder().credentials(credentials).build();
    ///
    ///     client.validate_token().await.unwrap_or(false)
    /// }
    /// ```
    ///
    /// [`ApiError`]: crate::core::Error#variant.ApiError
    pub async fn validate_token(&self) -> Result<bool> {
        // This API call returns a payload, but it doesn't really contain useful information:
        // if the token is invalid, 401 will be returned.
        let response = self
            .api_client
            .get("/validate_token")
            .send()
            .await
            .and_then(|r| r.error_for_status());
        match response {
            Ok(_) => Ok(true),
            Err(error) if error.status() == Some(StatusCode::UNAUTHORIZED) => Ok(false),
            Err(error) => Err(error.into()),
        }
    }

    /// Sends a "ping" to the server to determine if service is up and available. The call
    /// returns information about the website and database.
    ///
    /// # Notes
    ///
    /// - This call does not require [`credentials`](ClientBuilder::credentials), but works
    ///   anyway if they are provided.
    /// - As of this writing, the [current implementation](https://github.com/exercism/website/blob/2580b8fa2b13cad7aa7e8a877551bbd8552bee8b/app/controllers/api/v1/ping_controller.rb)
    ///   of this endpoint always return `true` as status for all components. It makes sense
    ///   if you think about it: if the database or the Rails server misbehave, then the API
    ///   would be inaccessible anyway 😅 It also means that if the service is actually down,
    ///   this method will simply return an [`ApiError`].
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
    ///     let client = api::v1::Client::new();
    ///
    ///     let service_status = client.ping().await?.status;
    ///     println!(
    ///         "Status: website: {}, database: {}",
    ///         service_status.website, service_status.database,
    ///     );
    ///
    ///     anyhow::Ok(())
    /// }
    /// ```
    ///
    /// [`ApiError`]: crate::core::Error#variant.ApiError
    pub async fn ping(&self) -> Result<PingResponse> {
        self.get("/ping", None).await
    }

    async fn get<'a, R>(&self, url: &str, query: Option<&[(&'static str, &'a str)]>) -> Result<R>
    where
        R: DeserializeOwned,
    {
        let mut request = self.api_client.get(url);
        if let Some(query) = query {
            request = request.query(query);
        }
        Ok(request.send().await?.error_for_status()?.json().await?)
    }
}

/// Struct representing a response to a query for a solution on the
/// [Exercism website](https://exercism.org) v1 API.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SolutionResponse {
    /// Solution information.
    pub solution: Solution,
}

/// Struct representing information about a solution returned by the
/// [Exercism website](https://exercism.org) v1 API.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Solution {
    /// Solution unique ID.
    #[serde(rename = "id")]
    pub uuid: String,

    /// Solution URL. This URL's value depends on who performs the query
    /// versus who submitted the solution:
    ///
    /// | Condition                                                       | URL value used                  |
    /// |-----------------------------------------------------------------|---------------------------------|
    /// | Query user submitted the solution                               | Public exercise URL[^1]         |
    /// | Query user is mentoring the solution's submitter                | URL of the mentoring discussion |
    /// | Query user is a mentor and solution was submitted for mentoring | URL of the mentoring request    |
    /// | Solution has been published and is accessible to query user     | Public URL of the solution      |
    ///
    /// [^1]: This is not a typo: the URL returned is indeed the public URL of the exercise,
    /// not the private URL of the solution for the user.
    pub url: String,

    /// Information about the user that submitted the solution.
    pub user: SolutionUser,

    /// Information about the solution's exercise.
    pub exercise: SolutionExercise,

    /// Base URL that can be used to download solution files. To fetch a specific file,
    /// use `{{file_download_base_url}}/{{file path}}` (with `{{file path}}` replaced by
    /// the path of a file returned in [`files`](Self::files).
    pub file_download_base_url: String,

    /// List of files that are part of the solution. This includes files submitted by
    /// the user as well as files that are provided by the exercise project. Files can
    /// be fetched by pre-pending their path with [`file_download_base_url`](Self::file_download_base_url).
    pub files: Vec<String>,

    /// Information about the submission of the solution. Only present if
    /// the solution has been submitted by the user.
    #[serde(default)]
    pub submission: Option<SolutionSubmission>,
}

/// Struct representing information about the user who created a solution,
/// as returned by the [Exercism website](https://exercism.org) v1 API.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SolutionUser {
    /// [Exercism](https://exercism.org) user handle.
    pub handle: String,

    /// Whether the user performing the query is the one that created the solution.
    pub is_requester: bool,
}

/// Struct representing information about the exercise for a solution, as returned by
/// the [Exercism website](https://exercism.org) v1 API.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SolutionExercise {
    /// Exercise name. This is an internal name, like `forth`. Also called `slug`.
    #[serde(rename = "id")]
    pub name: String,

    /// URL of the exercise's instructions (e.g., the public exercise URL).
    pub instructions_url: String,

    /// Information about the track containing the exercise.
    pub track: SolutionTrack,
}

/// Struct representing information about a language track returned by the
/// [Exercism website](https://exercism.org) v1 API.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SolutionTrack {
    /// Name of the language track. This is an internal name, like `common-lisp`. Also called `slug`.
    #[serde(rename = "id")]
    pub name: String,

    /// Language track title.
    /// This is a textual representation of the track name, like `Common Lisp`.
    #[serde(rename = "language")]
    pub title: String,
}

/// Struct representing information about the submission of a solution, as returned by the
/// [Exercism website](https://exercism.org) v1 API.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SolutionSubmission {
    /// Date/time when the solution has been submitted, in ISO-8601 format.
    pub submitted_at: String,
}

/// Struct representing a response to a track query on the [Exercism website](https://exercism.org) v1 API.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrackResponse {
    /// Information about the language track.
    pub track: SolutionTrack,
}

/// Struct representing a response to a ping request to the [Exercism website](https://exercism.org) v1 API.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PingResponse {
    /// Information about the status of the [Exercism](https://exercism.org) services.
    pub status: ServiceStatus,
}

/// Struct representing the status of services, as returned by the [Exercism website](https://exercism.org) v1 API.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceStatus {
    /// Whether the [Exercism website](https://exercism.org) is up and running.
    pub website: bool,

    /// Whether the database backing the [Exercism website](https://exercism.org) is working.
    pub database: bool,
}
