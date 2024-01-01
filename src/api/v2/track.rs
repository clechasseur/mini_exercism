//! Types related to tracks returned by the [Exercism website](https://exercism.org) v2 API.

use serde::{Deserialize, Serialize};

/// A single language track returned by the [Exercism website](https://exercism.org) v2 API.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Track {
    /// Name of the language track.
    ///
    /// This is an internal name, like `common-lisp`. Also called `slug`.
    #[serde(rename = "slug")]
    pub name: String,

    /// Language track title.
    ///
    /// This is a textual representation of the track name, like `Common Lisp`.
    pub title: String,

    /// Total number of concepts taught by the track.
    pub num_concepts: usize,

    /// Total number of exercises available in the track.
    pub num_exercises: usize,

    /// URL of this language track on the [Exercism website](https://exercism.org).
    pub web_url: String,

    /// URL of the icon representing this language track on the [Exercism website](https://exercism.org).
    pub icon_url: String,

    /// List of tags attached to this language track.
    ///
    /// Can contain many information, like `Object-oriented`, `Linux`, etc.
    pub tags: Vec<String>,

    /// Links pertaining to the language track.
    pub links: Links,

    /// Whether this track has been joined by the user.
    ///
    /// Will be set to `false` for anonymous queries or unjoined tracks.
    #[serde(default)]
    pub is_joined: bool,

    /// Number of concepts learnt by the user in this track.
    ///
    /// Will be set to `0` for anonymous queries or unjoined tracks.
    #[serde(default)]
    pub num_learnt_concepts: usize,

    /// Number of exercises completed by the user in this track.
    ///
    /// Will be set to `0` for anonymous queries or unjoined tracks.
    #[serde(default)]
    pub num_completed_exercises: usize,
}

/// Links pertaining to an [Exercism](https://exercism.org) language track returned by the v2 API.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Links {
    /// URL of the language track on the [Exercism website](https://exercism.org).
    ///
    /// Corresponds to the track's [`web_url`](Track::web_url).
    #[serde(rename = "self")]
    pub self_url: String,

    /// URL of the language track's exercises on the [Exercism website](https://exercism.org).
    pub exercises: String,

    /// URL of the language track's concepts on the [Exercism website](https://exercism.org).
    pub concepts: String,
}
