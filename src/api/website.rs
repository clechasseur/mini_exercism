//! Types and functions to interact with the [Exercism website](https://exercism.org) API.

mod detail;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use strum_macros::Display;

use crate::api::website::detail::{ExerciseFiltersBuilderError, TrackFiltersBuilderError};
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
    pub async fn get_tracks(&self, filters: Option<TrackFilters>) -> Result<TracksResponse> {
        let mut request = self.api_client.get("/tracks");
        if let Some(filters) = filters {
            let query: Vec<_> = filters.into();
            request = request.query(&query);
        }

        Ok(request.send().await?.json().await?)
    }

    /// Returns a list of exercises for an [Exercism](https://exercism.org) [track],
    /// optionally loading the user's solutions.
    ///
    /// - If the request is performed anonymously, returns a list of all exercises in
    ///   the track. Each exercise's [`is_external`](Exercise::is_external) field will
    ///   be set to `true`.
    /// - If the request is performed with [`credentials`](ClientBuilder::credentials),
    ///   returns a list of all exercises in the track, with information about whether
    ///   each exercise has been [unlocked](Exercise::is_unlocked) by the user. Each
    ///   exercise's [`is_external`](Exercise::is_external) field will be set to `false`.
    ///   Additionally, if the [`filters`] parameter's [`include_solutions`](ExerciseFilters::include_solutions)
    ///   is set to `true`, the response will contain a list of solutions the user has
    ///   submitted for the track's exercises.
    ///
    /// The list of exercises can optionally be filtered using [`ExerciseFilters`].
    ///
    /// # Errors
    ///
    /// - [`ApiError`]: Error while fetching exercise information from API
    ///
    /// [`ApiError`]: crate::core::Error#variant.ApiError
    pub async fn get_exercises(
        &self,
        track: &str,
        filters: Option<ExerciseFilters>,
    ) -> Result<ExercisesResponse> {
        let mut request = self
            .api_client
            .get(format!("/tracks/{}/exercises", track).as_str());
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
#[strum(serialize_all = "snake_case")]
pub enum TrackStatusFilter {
    /// Return all language tracks.
    #[default]
    All,

    /// Return only language tracks joined by the user.
    Joined,

    /// Return only language tracks *not* joined by the user.
    Unjoined,
}

/// Struct representing a response to a query for language tracks on the
/// [Exercism website](https://exercism.org) API.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TracksResponse {
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

/// Filters that can be applied when fetching exercises from the
/// [Exercism website](https://exercism.org) API (see [`Client::get_exercises`]).
#[derive(Debug, Default, Builder)]
#[builder(
    derive(Debug),
    default,
    setter(strip_option),
    build_fn(private, name = "fallible_build", error = "ExerciseFiltersBuilderError")
)]
pub struct ExerciseFilters {
    /// Criteria used to filter exercises.
    /// Applied to both exercise [`name`](Exercise::name)s (e.g. slugs) and [`title`](Exercise::title)s.
    #[builder(setter(into))]
    pub criteria: Option<String>,

    /// Whether to include solutions in the response.
    /// Only has an effect if the query is specified for an authenticated user.
    pub include_solutions: bool,
}

impl ExerciseFilters {
    /// Returns a builder for the [`ExerciseFilters`] type.
    pub fn builder() -> ExerciseFiltersBuilder {
        ExerciseFiltersBuilder::default()
    }
}

impl From<ExerciseFilters> for Vec<(String, String)> {
    /// Converts [`ExerciseFilters`] into a sequence of key/value pair
    /// that can be used as [query string parameters](reqwest::RequestBuilder::query).
    fn from(filters: ExerciseFilters) -> Self {
        let mut query = Self::new();

        if let Some(criteria) = filters.criteria {
            query.push(("criteria".to_string(), criteria));
        }

        if filters.include_solutions {
            query.push(("sideload".to_string(), "solutions".to_string()));
        }

        query
    }
}

impl ExerciseFiltersBuilder {
    /// Builds a new [ExerciseFilters].
    pub fn build(&self) -> ExerciseFilters {
        self.fallible_build()
            .expect("All fields should have had default values")
    }
}

/// Struct representing a response to a query for exercises on the
/// [Exercism website](https://exercism.org) API.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExercisesResponse {
    /// List of exercises for the requested track. The ordering depends on
    /// the type of query performed and matches that seen on the website.
    pub exercises: Vec<Exercise>,

    /// List of solutions submitted for exercises in this track. Will only be filled
    /// if the [`include_solutions`](ExerciseFilters::include_solutions) field of
    /// the query's [`ExerciseFilters`] is set to `true`.
    ///
    /// # Note
    ///
    /// Even if [`include_solutions`](ExerciseFilters::include_solutions) is set to
    /// `true`, solutions will not be fetched if the API is queried anonymously.
    #[serde(default)]
    pub solutions: Vec<Solution>,
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
#[serde(rename_all = "snake_case")]
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
#[serde(rename_all = "snake_case")]
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

/// Struct representing a solution to an exercise submitted to
/// the [Exercism website](https://exercism.org).
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Solution {
    /// Solution unique ID. This UUID can be used to fetch information about
    /// the solution from other API calls.
    pub uuid: String,

    /// Private solution URL. This usually points to the exercise on the
    /// [Exercism website](https://exercism.org).
    pub private_url: String,

    /// Public solution URL. This points to the user's solution on the
    /// [Exercism website](https://exercism.org).
    ///
    /// # Note
    ///
    /// If the solution has not been [`Published`](SolutionStatus#variant.Published),
    /// this URL is only valid for the authenticated user.
    pub public_url: String,

    /// Solution status. Also indicates whether the solution has been
    /// [`Published`](SolutionStatus#variant.Published).
    pub status: SolutionStatus,

    /// Solution mentoring status. If no mentoring has been requested, will
    /// contain the value [`None`](SolutionMentoringStatus#variant.None).
    pub mentoring_status: SolutionMentoringStatus,

    /// Status of tests for the solution's published iteration.
    pub published_iteration_head_tests_status: SolutionTestsStatus,

    /// Whether user has unread notifications about this solution (from
    /// a mentoring session, for example).
    pub has_notifications: bool,

    /// Number of views of the solution on the Community Solutions page
    /// for the exercise.
    pub num_views: i32,

    /// Number of stars given to the solution on the Community Solutions
    /// page for the exercise.
    pub num_stars: i32,

    /// Number of comments left on the solution on the Community Solutions
    /// page for the exercise.
    pub num_comments: i32,

    /// Number of iterations submitted for the solution.
    pub num_iterations: i32,

    /// Number of lines of code in the solution.
    #[serde(default)]
    pub num_loc: Option<i32>,

    /// Whether this solution is out of date compared to the exercise.
    pub is_out_of_date: bool,

    /// Date/time when the solution was [`Published`](SolutionStatus#variant.Published),
    /// in ISO-8601 format. Will be `None` if the solution hasn't been published.
    #[serde(default)]
    pub published_at: Option<String>,

    /// Date/time when the solution was marked as [`Completed`](SolutionStatus#variant.Completed),
    /// in ISO-8601 format. Will be `None` if the solution hasn't been marked as complete yet.
    #[serde(default)]
    pub completed_at: Option<String>,

    /// Date/time of the solution's last update, in ISO-8601 format.
    pub updated_at: String,

    /// Date/time when the solution's last iteration was sumitted, in ISO-8601 format.
    /// Will be `None` if the solution hasn't yet been [`Iterated`](SolutionStatus#variant.Iterated).
    #[serde(default)]
    pub last_iterated_at: Option<String>,

    /// Information about the exercise for which this solution was submitted.
    pub exercise: SolutionExercise,

    /// Information about the language track containing the exercise for which
    /// this solution was submitted.
    pub track: SolutionTrack,
}

/// Possible status of a solution to an exercise on the [Exercism website](https://exercism.org).
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SolutionStatus {
    /// Exercise has been started, but no iteration has been submitted yet.
    Started,

    /// At least one iteration has been submitted for the exercise, but
    /// the exercise has not been completed yet.
    Iterated,

    /// Exercise has been marked as complete.
    Completed,

    /// Exercise has been marked as complete and the solution has been published.
    Published,

    /// Unknown status. Included so that if new solution statuses are introduced in
    /// the website API later, this crate won't break (hopefully).
    #[serde(other)]
    Unknown,
}

/// Possible mentoring status of a solution on the [Exercism website](https://exercism.org).
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SolutionMentoringStatus {
    /// No mentoring has been required for this exercise.
    None,

    /// Mentoring has been requested for this exercise, but the
    /// request hasn't been processed yet.
    Requested,

    /// Mentoring is currently in progress for this exercise.
    InProgress,

    /// Mentoring has completed for this exercise.
    Finished,

    /// Unknown mentoring status. Included so that if new mentoring statuses
    /// are introduced in the website API later, this crate won't break (hopefully).
    #[serde(other)]
    Unknown,
}

/// Possible status of tests for a solution on the [Exercism website](https://exercism.org).
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SolutionTestsStatus {
    /// Tests have not been queued yet for this exerise.
    NotQueued,

    /// Tests have been queued for execution, but have not completed yet.
    Queued,

    /// Test run has completed and all tests passed.
    Passed,

    /// Test run has completed and one or more test(s) failed.
    Failed,

    /// Test run has not been executed because an error occurred
    /// (like a compiler error).
    Errored,

    /// Test run has not been executed because an exception occurred
    /// (possibly a bug in the test runner setup).
    Exceptioned,

    /// Test run has been cancelled.
    Cancelled,

    /// Unknown tests status. Included so that if new tests statuses are
    /// introduced in the website API later, this crate won't break (hopefully).
    #[serde(other)]
    Unknown,
}

/// Struct containing information about the exercise for which a solution was submitted
/// on the [Exercism website](https://exercism.org).
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SolutionExercise {
    /// Name of the exercise. This is an internal name, like `forth`. Also called `slug`.
    #[serde(rename = "slug")]
    pub name: String,

    /// Exercise title. This is a textual representation of the title, like `Forth`.
    pub title: String,

    /// URL of the icon representing this exercise on the [Exercism website](https://exercism.org).
    pub icon_url: String,
}

/// Struct containing information about the language track of a solution submitted
/// on the [Exercism website](https://exercism.org).
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SolutionTrack {
    /// Name of the language track.
    /// This is an internal name, like `common-lisp`. Also called `slug`.
    #[serde(rename = "slug")]
    pub name: String,

    /// Language track title.
    /// This is a textual representation of the track name, like `Common Lisp`.
    pub title: String,

    /// URL of the icon representing this language track on the [Exercism website](https://exercism.org).
    pub icon_url: String,
}
