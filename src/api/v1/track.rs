//! Types related to tracks returned by the [Exercism website](https://exercism.org) v1 API.

use serde::{Deserialize, Serialize};

/// Response to a track query on the [Exercism website](https://exercism.org) v1 API.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Response {
    /// Information about the language track.
    pub track: Track,
}

/// A language track returned by the [Exercism website](https://exercism.org) v1 API.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Track {
    /// Name of the language track.
    ///
    /// This is an internal name, like `common-lisp`. Also called `slug`.
    #[serde(rename = "id")]
    pub name: String,

    /// Language track title.
    ///
    /// This is a textual representation of the track name, like `Common Lisp`.
    #[serde(rename = "language")]
    pub title: String,
}
