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
    pub async fn get_solution(&self, uuid: Option<String>) -> Result<SolutionResponse> {
        let request = self
            .api_client
            .get(format!("/solutions/{}", uuid.unwrap()).as_str());

        Ok(request.send().await?.json().await?)
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SolutionResponse {
    pub solution: Solution,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Solution {
    #[serde(rename = "id")]
    pub uuid: String,

    pub url: String,

    pub user: SolutionUser,

    pub exercise: SolutionExercise,

    pub file_download_base_url: String,

    pub files: Vec<String>,

    #[serde(default)]
    pub submission: Option<SolutionSubmission>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SolutionUser {
    pub handle: String,

    pub is_requester: bool,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SolutionExercise {
    #[serde(rename = "id")]
    pub name: String,

    pub instructions_url: String,

    pub track: SolutionTrack,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SolutionTrack {
    #[serde(rename = "id")]
    pub name: String,

    #[serde(rename = "language")]
    pub title: String,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SolutionSubmission {
    pub submitted_at: String,
}
