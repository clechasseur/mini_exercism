//! Types related to solutions returned by the [Exercism website](https://exercism.org) v1 API.

use serde::{Deserialize, Serialize};

use crate::api::v1::track::Track;

/// Response to a query for a solution on the [Exercism website](https://exercism.org) v1 API.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Response {
    /// Solution information.
    pub solution: Solution,
}

/// A solution returned by the [Exercism website](https://exercism.org) v1 API.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Solution {
    /// Solution unique ID.
    #[serde(rename = "id")]
    pub uuid: String,

    /// Solution URL.
    ///
    /// This URL's value depends on who performs the query versus who submitted the solution:
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
    pub user: User,

    /// Information about the solution's exercise.
    pub exercise: Exercise,

    /// Base URL that can be used to download solution files.
    ///
    /// To fetch a specific file, use `{{file_download_base_url}}/{{file path}}`
    /// (with `{{file path}}` replaced by the path of a file returned in [`files`](Self::files)).
    pub file_download_base_url: String,

    /// List of files that are part of the solution.
    ///
    /// This includes files submitted by the user as well as files that are provided by the
    /// exercise project. Files can be fetched by pre-pending their path with
    /// [`file_download_base_url`](Self::file_download_base_url).
    pub files: Vec<String>,

    /// Information about the submission of the solution.
    ///
    /// Only present if the solution has been submitted by the user.
    #[serde(default)]
    pub submission: Option<Submission>,
}

/// User who created a solution, as returned by the [Exercism website](https://exercism.org) v1 API.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    /// [Exercism](https://exercism.org) user handle.
    pub handle: String,

    /// Whether the user performing the query is the one that created the solution.
    pub is_requester: bool,
}

/// Exercise for a solution, as returned by the [Exercism website](https://exercism.org) v1 API.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Exercise {
    /// Exercise name.
    ///
    /// This is an internal name, like `forth`. Also called `slug`.
    #[serde(rename = "id")]
    pub name: String,

    /// URL of the exercise's instructions (e.g., the public exercise URL).
    pub instructions_url: String,

    /// Information about the track containing the exercise.
    pub track: Track,
}

/// Submission of a solution, as returned by the [Exercism website](https://exercism.org) v1 API.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Submission {
    /// Date/time when the solution has been submitted, in ISO-8601 format.
    pub submitted_at: String,
}
