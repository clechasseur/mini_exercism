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
    /// * `uuid` - UUID of the solution to fetch. This can be provided by the mentoring
    /// interface, or returned by another API, like
    /// [`api::v2::Client::get_exercises`](crate::api::v2::Client::get_exercises)
    /// (see [`Solution::uuid`](crate::api::v2::Solution::uuid)).
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
    /// [`ApiError`]: crate::core::Error#variant.ApiError
    pub async fn get_solution(&self, uuid: &str) -> Result<SolutionResponse> {
        Ok(self
            .api_client
            .get(format!("/solutions/{}", uuid).as_str())
            .send()
            .await?
            .json()
            .await?)
    }

    /// Returns information about the latest solution submitted by the user for
    /// a given exercise.
    ///
    /// # Arguments
    ///
    /// * `track` - Name of the language track. Also called `slug`.
    /// * `exercise` - Name of the exercise. Also called `slug`.
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
    /// [`ApiError`]: crate::core::Error#variant.ApiError
    pub async fn get_latest_solution(
        &self,
        track: &str,
        exercise: &str,
    ) -> Result<SolutionResponse> {
        Ok(self
            .api_client
            .get("/solutions/latest")
            .query(&[("track_id", track), ("exercise_id", exercise)])
            .send()
            .await?
            .json()
            .await?)
    }

    /// Returns information about a language track.
    ///
    /// # Arguments
    ///
    /// * `track` - Name of the language track. Also called `slug`.
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
    /// [`ApiError`]: crate::core::Error#variant.ApiError
    pub async fn get_track(&self, track: &str) -> Result<TrackResponse> {
        Ok(self
            .api_client
            .get(format!("/tracks/{}", track).as_str())
            .send()
            .await?
            .json()
            .await?)
    }

    /// Validates the API token used to perform API requests. If the API token is invalid or
    /// if the query is performed without [`credentials`](ClientBuilder::credentials), a
    /// `401 Unauthorized` error will be returned.
    ///
    /// # Errors
    ///
    /// - [`ApiError`]: Error while validating API token
    ///
    /// [`ApiError`]: crate::core::Error#variant.ApiError
    pub async fn validate_token(&self) -> Result<ValidateTokenResponse> {
        Ok(self
            .api_client
            .get("/validate_token")
            .send()
            .await?
            .json()
            .await?)
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
    /// the path of a file returned in [`files`].
    pub file_download_base_url: String,

    /// List of files that are part of the solution. This includes files submitted by
    /// the user as well as files that are provided by the exercise project. Files can
    /// be fetched by pre-pending their path with [`file_download_base_url`].
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

/// Struct representing a response to a query to validate API token, as returned by
/// the [Exercism website](https://exercism.org) v1 API.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidateTokenResponse {
    /// Information about status of the API token.
    #[serde(rename = "status")]
    pub token_status: TokenStatus,
}

/// Struct representing the status of an API token.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TokenStatus {
    /// Token status. Will always contain the string `valid`; if the API token
    /// is invalid, then the query will simply fail with a `401 Unauthorized`.
    #[serde(rename = "token")]
    pub status: String,
}
