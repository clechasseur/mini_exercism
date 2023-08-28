//! Types and functions to interact with the [Exercism website](https://exercism.org) API.

mod detail;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use strum_macros::Display;

use crate::api::website::detail::TrackFiltersBuilderError;
use crate::core::Result;

/// Default base URL for the [Exercism website](https://exercism.org) API.
pub const DEFAULT_WEBSITE_API_BASE_URL: &str = "https://exercism.org/api/v2";

define_api_client! {
    /// Client for the [Exercism website](https://exercism.org) API. This API is undocumented and
    /// is mostly used by the website itself to fetch information.
    pub struct Client(DEFAULT_WEBSITE_API_BASE_URL);
}

impl Client {
    /// Returns a list of [Exercism tracks](https://exercism.org/tracks).
    ///
    /// - If the request is performed anonymously, will return a list of all tracks
    ///   supported on the website.
    /// - If the request is performed with [`credentials`](ClientBuilder::credentials),
    ///   tracks that the user has joined will be identified by the
    ///   [`is_joined`](Track::is_joined) field.
    ///
    /// The list of tracks can optionally be filtered using [`TrackFilters`].
    ///
    /// # Errors
    ///
    /// - [`ApiError`]: Error while fetching track information from API
    ///
    /// [`ApiError`]: crate::core::Error#variant.ApiError
    pub async fn get_tracks(&self, filters: Option<TrackFilters>) -> Result<Tracks> {
        let mut request = self.api_client.get("/tracks");
        if let Some(filters) = filters {
            let query: Vec<_> = filters.into();
            request = request.query(&query);
        }

        Ok(request.send().await?.json().await?)
    }
}

/// Filters that can be applied when fetching language tracks from the [Exercism website](https://exercism.org) API
/// (see [`Client::get_tracks`]).
#[derive(Debug, Default, Builder)]
#[builder(
    derive(Debug),
    default,
    setter(strip_option),
    build_fn(private, name = "fallible_build", error = "TrackFiltersBuilderError")
)]
pub struct TrackFilters {
    /// Criteria used to filter language tracks.
    /// Applied to both track [`name`](Track::name)s (e.g. slugs) and [`title`](Track::title)s.
    #[builder(setter(into))]
    pub criteria: Option<String>,

    /// List of [`tags`](Track::tags) that must be attached to the language track.
    ///
    /// # Note
    ///
    /// This filter does not currently seem to work; whether this is the result of
    /// a bug in the Exercism website API or in this library remains to be determined.
    #[builder(setter(each(name = "tag", into)))]
    pub tags: Vec<String>,

    /// Language track's status filter.
    ///
    /// # Note
    ///
    /// Using this filter requires an authenticated query to the [Exercism website](https://exercism.org) API,
    /// otherwise you will get a `500 Internal Server Error`.
    pub status: Option<TrackStatusFilter>,
}

impl TrackFilters {
    /// Returns a builder for the [`TrackFilters`] type.
    pub fn builder() -> TrackFiltersBuilder {
        TrackFiltersBuilder::default()
    }
}

impl From<TrackFilters> for Vec<(String, String)> {
    /// Converts [`TrackFilters`] into a sequence of key/value pair
    /// that can be used as [query string parameters](reqwest::RequestBuilder::query).
    fn from(filters: TrackFilters) -> Self {
        let mut query = Self::new();

        if let Some(criteria) = filters.criteria {
            query.push(("criteria".to_string(), criteria));
        }

        filters.tags.into_iter().for_each(|tag| {
            query.push(("tags[]".to_string(), tag));
        });

        if let Some(status) = filters.status {
            query.push(("status".to_string(), status.to_string()));
        }

        query
    }
}

impl TrackFiltersBuilder {
    /// Builds a new [TrackFilters].
    pub fn build(&self) -> TrackFilters {
        self.fallible_build()
            .expect("All fields should have had default values")
    }
}

/// Possible status filter of [Exercism](https://exercism.org) language tracks.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Display)]
#[strum(serialize_all = "lowercase")]
pub enum TrackStatusFilter {
    /// Return all language tracks.
    #[default]
    All,

    /// Return only language tracks joined by the user.
    Joined,

    /// Return only language tracks *not* joined by the user.
    Unjoined,
}

/// Struct used to return [Exercism](https://exercism.org) language tracks from the website API.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tracks {
    /// List of [Exercism](https://exercism.org) language tracks. Usually sorted alphabetically by track name.
    pub tracks: Vec<Track>,
}

/// Struct representing a single language track returned by the [Exercism website](https://exercism.org) API.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Track {
    /// Name of the language track.
    /// This is an internal name, like `common-lisp`. Also called `slug`.
    #[serde(rename = "slug")]
    pub name: String,

    /// Language track title.
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
    /// Can contain many information, like `Object-oriented`, `Linux`, etc.
    pub tags: Vec<String>,

    /// Struct containing some links pertaining to the language track.
    pub links: TrackLinks,

    /// Whether this track has been joined by the user.
    /// Will be set to `false` for anonymous queries or unjoined tracks.
    #[serde(default)]
    pub is_joined: bool,

    /// Number of concepts learnt by the user in this track.
    /// Will be set to `0` for anonymous queries or unjoined tracks.
    #[serde(default)]
    pub num_learnt_concepts: usize,

    /// Number of exercises completed by the user in this track.
    /// Will be set to `0` for anonymous queries or unjoined tracks.
    #[serde(default)]
    pub num_completed_exercises: usize,
}

/// Struct containing links pertaining to an [Exercism](https://exercism.org) language track
/// returned by the website API.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrackLinks {
    /// URL of the language track on the [Exercism website](https://exercism.org).
    /// Corresponds to the track's [`web_url`](Track::web_url).
    #[serde(rename = "self")]
    pub self_url: String,

    /// URL of the language track's exercises on the [Exercism website](https://exercism.org).
    pub exercises: String,

    /// URL of the language track's concepts on the [Exercism website](https://exercism.org).
    pub concepts: String,
}

/// Struct representing a single exercise returned by the [Exercism website](https://exercism.org) API.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Exercise {
    /// Name of the exercise. This is an internal name, like `forth`. Also called `slug`.
    #[serde(rename = "slug")]
    pub name: String,

    /// Type of exercise.
    #[serde(rename = "type")]
    pub exercise_type: ExerciseType,

    /// Exercise title. This is a textual representation of the title, like `Forth`.
    pub title: String,

    /// URL of the icon representing this exercise on the [Exercism website](https://exercism.org).
    pub icon_url: String,

    /// Exercise difficulty rating.
    pub difficulty: ExerciseDifficulty,

    /// Short description of the exercise.
    pub blurb: String,

    /// Whether this is an "exernal" exercise. This is used to indicate exercises that are not
    /// tied to a user. When returned by the website API, this indicates that the request was
    /// performed anonymously.
    pub is_external: bool,

    /// Whether this exercise has been unlocked by the user. Will always be `false` when exercises
    /// are queried anonymously.
    pub is_unlocked: bool,

    /// Whether this is the next recommended exercise for the user in the language track. Will always
    /// be `false` when exercises are queried anonymously.
    pub is_recommended: bool,

    /// Struct containing some links pertaining to the exercise.
    pub links: ExerciseLinks,
}

/// Possible type of exercise on the [Exercism website](https://exercism.org).
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExerciseType {
    /// Tutorial exercise. Currently only known to apply to `hello-world`.
    Tutorial,

    /// Concept exercise, e.g. an exercise tied to a concept on the language track's syllabus.
    Concept,

    /// Practice exercise. Most exercise are in this category.
    Practice,

    /// Unknown exercise type. Included so that if new exercise types are introduced in
    /// the website API later, this crate will not break (hopefully).
    #[serde(other)]
    Unknown,
}

/// Possible difficulty rating of an exercise on the [Exercism website](https://exercism.org).
/// Internally, exercises have a difficulty rating between 1 and 10 (inclusive); however, on the
/// website, this is only represented by specific, named difficulty ratings.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExerciseDifficulty {
    /// Easy exercise. Internally, an exercise with a difficulty rating between 1 and 3 (inclusive).
    Easy,

    /// Medium exercise. Internally, an exercise with a difficulty rating between 4 and 7 (inclusive).
    Medium,

    /// Hard exercise. Internally, an exercise with a difficulty rating above 7.
    Hard,

    /// Unknown difficulty. Included so that if new exercise difficulty ratings are introduced
    /// in the website API later, this crate will not break (hopefully).
    #[serde(other)]
    Unknown,
}

/// Struct containing links pertaining to an [Exercism](https://exercism.org) exercise
/// returned by the website API.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExerciseLinks {
    /// Path of the exercise on the [Exercism](https://exercism.org), without the domain name.
    #[serde(rename = "self")]
    pub self_path: String,
}
