//! Types and functions to interact with the [Exercism website](https://exercism.org) v2 API.

mod detail;

use std::fmt::Display;

use derive_builder::Builder;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, IntoStaticStr};

use crate::api::v2::detail::{ExerciseFiltersBuilderError, TrackFiltersBuilderError};
use crate::core::Result;

/// Default base URL for the [Exercism website](https://exercism.org) v2 API.
pub const DEFAULT_V2_API_BASE_URL: &str = "https://exercism.org/api/v2";

define_api_client! {
    /// Client for the [Exercism website](https://exercism.org) v2 API.
    ///
    /// This API is undocumented and is mostly used by the website itself to fetch information.
    #[derive(Debug, Clone)]
    pub struct Client(DEFAULT_V2_API_BASE_URL);
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
    /// # Examples
    ///
    /// ```no_run
    /// use mini_exercism::api;
    /// use mini_exercism::api::v2::TrackFilters;
    /// use mini_exercism::api::v2::TrackStatusFilter::Joined;
    /// use mini_exercism::core::Credentials;
    ///
    /// async fn get_joined_tracks(api_token: &str) -> anyhow::Result<Vec<String>> {
    ///     let credentials = Credentials::from_api_token(api_token);
    ///     let client = api::v2::Client::builder().credentials(credentials).build();
    ///
    ///     let filters = TrackFilters::builder().status(Joined).build();
    ///     let tracks = client.get_tracks(Some(filters)).await?.tracks;
    ///
    ///     anyhow::Ok(tracks.into_iter().map(|track| track.name).collect())
    /// }
    /// ```
    ///
    /// [`ApiError`]: crate::core::Error::ApiError
    pub async fn get_tracks(&self, filters: Option<TrackFilters<'_>>) -> Result<TracksResponse> {
        self.get("/tracks", filters).await
    }

    /// Returns a list of exercises for an [Exercism](https://exercism.org) `track`,
    /// optionally loading the user's solutions.
    ///
    /// - If the request is performed anonymously, returns a list of all exercises in
    ///   the track. Each exercise's [`is_external`](Exercise::is_external) field will
    ///   be set to `true`.
    /// - If the request is performed with [`credentials`](ClientBuilder::credentials),
    ///   returns a list of all exercises in the track, with information about whether
    ///   each exercise has been [unlocked](Exercise::is_unlocked) by the user. Each
    ///   exercise's [`is_external`](Exercise::is_external) field will be set to `false`.
    ///   Additionally, if the `filters` parameter's [`include_solutions`](ExerciseFilters::include_solutions)
    ///   is set to `true`, the response will contain a list of solutions the user has submitted
    ///   for the track's exercises.
    ///
    /// The list of exercises can optionally be filtered using [`ExerciseFilters`].
    ///
    /// # Notes
    ///
    /// If the `filters` parameter's [`include_solutions`](ExerciseFilters::include_solutions) is
    /// set to `true`, the returned [`solutions`](ExercisesResponse::solutions) will return all
    /// solutions; the solutions are not filtered like exercises are.
    ///
    /// # Errors
    ///
    /// - [`ApiError`]: Error while fetching exercise information from API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mini_exercism::api;
    /// use mini_exercism::api::v2::ExerciseFilters;
    /// use mini_exercism::core::Credentials;
    ///
    /// async fn get_published_solution_uuids(
    ///     api_token: &str,
    ///     track: &str,
    /// ) -> anyhow::Result<Vec<String>> {
    ///     let credentials = Credentials::from_api_token(api_token);
    ///     let client = api::v2::Client::builder().credentials(credentials).build();
    ///
    ///     let filters = ExerciseFilters::builder().include_solutions(true).build();
    ///     let solutions = client.get_exercises(track, Some(filters)).await?.solutions;
    ///
    ///     anyhow::Ok(
    ///         solutions
    ///             .into_iter()
    ///             .filter(|solution| solution.published_at.is_some())
    ///             .map(|solution| solution.uuid)
    ///             .collect(),
    ///     )
    /// }
    /// ```
    ///
    /// [`ApiError`]: crate::core::Error::ApiError
    pub async fn get_exercises(
        &self,
        track: &str,
        filters: Option<ExerciseFilters<'_>>,
    ) -> Result<ExercisesResponse> {
        self.get(format!("/tracks/{}/exercises", track), filters)
            .await
    }

    async fn get<'a, U, F, R>(&self, url: U, filters: Option<F>) -> Result<R>
    where
        U: Display,
        F: Into<Vec<(&'static str, &'a str)>>,
        R: DeserializeOwned,
    {
        let mut request = self.api_client.get(url);
        if let Some(filters) = filters {
            let query: Vec<_> = filters.into();
            request = request.query(&query);
        }
        Ok(request.send().await?.error_for_status()?.json().await?)
    }
}

/// Filters that can be applied when fetching language tracks from the
/// [Exercism website](https://exercism.org) v2 API.
///
/// See [`Client::get_tracks`].
#[derive(Debug, Clone, Default, Builder)]
#[builder(
    derive(Debug),
    default,
    setter(strip_option),
    build_fn(private, name = "fallible_build", error = "TrackFiltersBuilderError")
)]
pub struct TrackFilters<'a> {
    /// Criteria used to filter language tracks.
    ///
    /// Applied to both track [`name`](Track::name)s (e.g. slugs) and [`title`](Track::title)s.
    #[builder(setter(into))]
    pub criteria: Option<&'a str>,

    /// List of [`tags`](Track::tags) that must be attached to the language track.
    ///
    /// # Note
    ///
    /// This filter does not currently seem to work; whether this is the result of
    /// a bug in the Exercism v2 API or in this library remains to be determined.
    #[builder(setter(into, each(name = "tag")))]
    pub tags: Vec<&'a str>,

    /// Language track's [status filter](TrackStatusFilter).
    ///
    /// # Note
    ///
    /// Using this filter requires an authenticated query to the [Exercism website](https://exercism.org)
    /// v2 API, otherwise you will get a `500 Internal Server Error` (even when asking for
    /// [`All`](TrackStatusFilter::All) tracks).
    pub status: Option<TrackStatusFilter>,
}

impl<'a> TrackFilters<'a> {
    /// Returns a builder for the [`TrackFilters`] type.
    pub fn builder() -> TrackFiltersBuilder<'a> {
        TrackFiltersBuilder::default()
    }
}

impl<'a> From<TrackFilters<'a>> for Vec<(&'static str, &'a str)> {
    /// Converts [`TrackFilters`] into a sequence of key/value pair
    /// that can be used as [query string parameters](reqwest::RequestBuilder::query).
    fn from(filters: TrackFilters<'a>) -> Self {
        let mut query = Self::new();

        if let Some(criteria) = filters.criteria {
            query.push(("criteria", criteria));
        }

        filters.tags.into_iter().for_each(|tag| {
            query.push(("tags[]", tag));
        });

        if let Some(status) = filters.status {
            query.push(("status", status.into()));
        }

        query
    }
}

impl<'a> TrackFiltersBuilder<'a> {
    /// Builds a new [TrackFilters].
    pub fn build(&self) -> TrackFilters<'a> {
        self.fallible_build()
            .expect("All fields should have had default values")
    }
}

/// Possible status filter of [Exercism](https://exercism.org) language tracks.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Display, IntoStaticStr)]
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

/// Response to a query for language tracks on the [Exercism website](https://exercism.org) v2 API.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TracksResponse {
    /// List of [Exercism](https://exercism.org) language tracks.
    ///
    /// Usually sorted alphabetically by track name, with tracks joined by the user first
    /// (if query is performed with [`credentials`](ClientBuilder::credentials)).
    pub tracks: Vec<Track>,
}

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
    pub links: TrackLinks,

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
pub struct TrackLinks {
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

/// Filters that can be applied when fetching exercises from the
/// [Exercism website](https://exercism.org) v2 API.
///
/// See [`Client::get_exercises`].
#[derive(Debug, Clone, Default, Builder)]
#[builder(
    derive(Debug),
    default,
    setter(strip_option),
    build_fn(private, name = "fallible_build", error = "ExerciseFiltersBuilderError")
)]
pub struct ExerciseFilters<'a> {
    /// Criteria used to filter exercises.
    ///
    /// Applied to both exercise [`name`](Exercise::name)s (e.g. slugs) and [`title`](Exercise::title)s.
    #[builder(setter(into))]
    pub criteria: Option<&'a str>,

    /// Whether to include solutions in the response.
    ///
    /// Only has an effect if the query is performed with [`credentials`](ClientBuilder::credentials).
    pub include_solutions: bool,
}

impl<'a> ExerciseFilters<'a> {
    /// Returns a builder for the [`ExerciseFilters`] type.
    pub fn builder() -> ExerciseFiltersBuilder<'a> {
        ExerciseFiltersBuilder::default()
    }
}

impl<'a> From<ExerciseFilters<'a>> for Vec<(&'static str, &'a str)> {
    /// Converts [`ExerciseFilters`] into a sequence of key/value pair
    /// that can be used as [query string parameters](reqwest::RequestBuilder::query).
    fn from(filters: ExerciseFilters<'a>) -> Self {
        let mut query = Self::new();

        if let Some(criteria) = filters.criteria {
            query.push(("criteria", criteria));
        }

        if filters.include_solutions {
            query.push(("sideload", "solutions"));
        }

        query
    }
}

impl<'a> ExerciseFiltersBuilder<'a> {
    /// Builds a new [ExerciseFilters].
    pub fn build(&self) -> ExerciseFilters<'a> {
        self.fallible_build()
            .expect("All fields should have had default values")
    }
}

/// Response to a query for exercises on the [Exercism website](https://exercism.org) v2 API.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExercisesResponse {
    /// List of exercises for the requested track.
    ///
    /// The ordering depends on the type of query performed and matches that seen on the website.
    pub exercises: Vec<Exercise>,

    /// List of solutions submitted for exercises in this track.
    ///
    /// Will only be filled if the [`include_solutions`](ExerciseFilters::include_solutions)
    /// field of the query's [`ExerciseFilters`] is set to `true`.
    ///
    /// # Note
    ///
    /// Even if [`include_solutions`](ExerciseFilters::include_solutions) is set to
    /// `true`, solutions will not be fetched if the API is queried anonymously.
    #[serde(default)]
    pub solutions: Vec<Solution>,
}

/// A single exercise returned by the [Exercism website](https://exercism.org) v2 API.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Exercise {
    /// Name of the exercise.
    ///
    /// This is an internal name, like `forth`. Also called `slug`.
    #[serde(rename = "slug")]
    pub name: String,

    /// Type of exercise.
    #[serde(rename = "type")]
    pub exercise_type: ExerciseType,

    /// Exercise title.
    ///
    /// This is a textual representation of the title, like `Forth`.
    pub title: String,

    /// URL of the icon representing this exercise on the [Exercism website](https://exercism.org).
    pub icon_url: String,

    /// Exercise difficulty rating.
    pub difficulty: ExerciseDifficulty,

    /// Short description of the exercise.
    pub blurb: String,

    /// Whether this is an "exernal" exercise.
    ///
    /// This is used to indicate exercises that are not tied to a user. When returned by the
    /// website API, this indicates that the request was performed anonymously.
    pub is_external: bool,

    /// Whether this exercise has been unlocked by the user.
    ///
    /// Will always be `false` when exercises are queried anonymously.
    pub is_unlocked: bool,

    /// Whether this is the next recommended exercise for the user in the language track.
    ///
    /// Will always be `false` when exercises are queried anonymously.
    pub is_recommended: bool,

    /// Links pertaining to the exercise.
    pub links: ExerciseLinks,
}

/// Possible type of exercise on the [Exercism website](https://exercism.org).
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExerciseType {
    /// Tutorial exercise.
    ///
    /// Currently only known to apply to `hello-world`.
    Tutorial,

    /// Concept exercise, e.g. an exercise tied to a concept on the language track's syllabus.
    Concept,

    /// Practice exercise.
    ///
    /// Most exercise are in this category.
    Practice,

    /// Unknown exercise type.
    ///
    /// Included so that if new exercise types are introduced in the website API later,
    /// this crate will not break (hopefully).
    #[serde(other)]
    Unknown,
}

/// Possible difficulty rating of an exercise on the [Exercism website](https://exercism.org).
///
/// Internally, exercises have a difficulty rating between 1 and 10 (inclusive); however, on the
/// website, this is only represented by specific, named difficulty ratings.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExerciseDifficulty {
    /// Easy exercise.
    ///
    /// Internally, an exercise with a difficulty rating between 1 and 3 (inclusive).
    Easy,

    /// Medium exercise.
    ///
    /// Internally, an exercise with a difficulty rating between 4 and 7 (inclusive).
    Medium,

    /// Hard exercise.
    ///
    /// Internally, an exercise with a difficulty rating above 7.
    Hard,

    /// Unknown difficulty.
    ///
    /// Included so that if new exercise difficulty ratings are introduced in the website API later,
    /// this crate will not break (hopefully).
    #[serde(other)]
    Unknown,
}

/// Links pertaining to an [Exercism](https://exercism.org) exercise returned by the v2 API.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExerciseLinks {
    /// Path of the exercise on the [Exercism website](https://exercism.org), without the domain name.
    #[serde(rename = "self")]
    pub self_path: String,
}

/// A solution to an exercise submitted to the [Exercism website](https://exercism.org).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Solution {
    /// Solution unique ID.
    ///
    /// This UUID can be used to fetch information about the solution from other API calls, like
    /// [`api::v1::Client::get_solution`](crate::api::v1::Client::get_solution).
    pub uuid: String,

    /// Private solution URL.
    ///
    /// This usually points to the exercise on the [Exercism website](https://exercism.org).
    pub private_url: String,

    /// Public solution URL.
    ///
    /// This points to the user's solution on the [Exercism website](https://exercism.org).
    ///
    /// # Note
    ///
    /// If the solution has not been [`Published`](SolutionStatus::Published), this URL is only
    /// valid for the authenticated user.
    pub public_url: String,

    /// Solution status.
    ///
    /// Also indicates whether the solution has been [`Published`](SolutionStatus::Published).
    pub status: SolutionStatus,

    /// Solution mentoring status.
    ///
    /// If no mentoring has been requested, will contain the value [`None`](SolutionMentoringStatus::None).
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

    /// Number of lines of code in the solution, excluding blank lines and comments.
    #[serde(default)]
    pub num_loc: Option<i32>,

    /// Whether this solution is out of date compared to the exercise.
    pub is_out_of_date: bool,

    /// Date/time when the solution was [`Published`](SolutionStatus::Published),
    /// in ISO-8601 format.
    ///
    /// Will be `None` if the solution hasn't been published.
    #[serde(default)]
    pub published_at: Option<String>,

    /// Date/time when the solution was marked as [`Completed`](SolutionStatus::Completed),
    /// in ISO-8601 format.
    ///
    /// Will be `None` if the solution hasn't been marked as complete yet.
    #[serde(default)]
    pub completed_at: Option<String>,

    /// Date/time of the solution's last update, in ISO-8601 format.
    pub updated_at: String,

    /// Date/time when the solution's last iteration was sumitted, in ISO-8601 format.
    ///
    /// Will be `None` if the solution hasn't yet been [`Iterated`](SolutionStatus::Iterated).
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

    /// Unknown status.
    ///
    /// Included so that if new solution statuses are introduced in the website API later,
    /// this crate won't break (hopefully).
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

    /// Unknown mentoring status.
    ///
    /// Included so that if new mentoring statuses are introduced in the website API later,
    /// this crate won't break (hopefully).
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

    /// Unknown tests status.
    ///
    /// Included so that if new tests statuses are introduced in the website API later,
    /// this crate won't break (hopefully).
    #[serde(other)]
    Unknown,
}

/// Exercise for which a solution was submitted on the [Exercism website](https://exercism.org).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SolutionExercise {
    /// Name of the exercise.
    ///
    /// This is an internal name, like `forth`. Also called `slug`.
    #[serde(rename = "slug")]
    pub name: String,

    /// Exercise title.
    ///
    /// This is a textual representation of the title, like `Forth`.
    pub title: String,

    /// URL of the icon representing this exercise on the [Exercism website](https://exercism.org).
    pub icon_url: String,
}

/// Language track of a solution submitted on the [Exercism website](https://exercism.org).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SolutionTrack {
    /// Name of the language track.
    ///
    /// This is an internal name, like `common-lisp`. Also called `slug`.
    #[serde(rename = "slug")]
    pub name: String,

    /// Language track title.
    ///
    /// This is a textual representation of the track name, like `Common Lisp`.
    pub title: String,

    /// URL of the icon representing this language track on the [Exercism website](https://exercism.org).
    pub icon_url: String,
}
