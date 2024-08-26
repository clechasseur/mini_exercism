//! Types related to solutions returned by the [Exercism website](https://exercism.org) v2 API.

use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumString, IntoStaticStr, VariantNames};

use crate::api::v2::iteration::Iteration;
use crate::api::v2::tests;

/// Response to a query for a solution on the [Exercism website](https://exercism.org) v2 API.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Response {
    /// Solution information.
    pub solution: Solution,

    /// Solution iterations, in ascending order of [`index`](Iteration::index).
    ///
    /// Will only be filled if `include_iterations` is set to `true` when calling [`get_solution`](crate::api::v2::Client::get_solution).
    #[serde(default)]
    pub iterations: Vec<Iteration>,
}

/// A solution to an exercise submitted to the [Exercism website](https://exercism.org).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Solution {
    /// Solution unique ID.
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
    /// If the solution has not been [`Published`](Status::Published), this URL is only
    /// valid for the authenticated user.
    pub public_url: String,

    /// Solution status.
    ///
    /// Also indicates whether the solution has been [`Published`](Status::Published).
    pub status: Status,

    /// Solution mentoring status.
    ///
    /// If no mentoring has been requested, will contain the value [`None`](MentoringStatus::None).
    pub mentoring_status: MentoringStatus,

    /// Status of tests for the solution's published iteration.
    pub published_iteration_head_tests_status: tests::Status,

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

    /// Date/time when the solution was [`Published`](Status::Published),
    /// in ISO-8601 format.
    ///
    /// Will be `None` if the solution hasn't been published.
    #[serde(default)]
    pub published_at: Option<String>,

    /// Date/time when the solution was marked as [`Completed`](Status::Completed),
    /// in ISO-8601 format.
    ///
    /// Will be `None` if the solution hasn't been marked as complete yet.
    #[serde(default)]
    pub completed_at: Option<String>,

    /// Date/time of the solution's last update, in ISO-8601 format.
    ///
    /// # Notes
    ///
    /// This timestamp is updated every time a solution is updated - even automatically. Because the Exercism
    /// platform sometimes re-processes solutions to re-run the tests when exercises change, this timestamp
    /// may be updated without user interaction.
    pub updated_at: String,

    /// Date/time when the solution's last iteration was sumitted, in ISO-8601 format.
    ///
    /// Will be `None` if the solution hasn't yet been [`Iterated`](Status::Iterated).
    ///
    /// # Notes
    ///
    /// Old solutions sometimes do not have this timestamp, even if iterations have been submitted.
    /// In such a case, it's possible to detect the existence of submissions via the [`num_iterations`](Self::num_iterations) field.
    #[serde(default)]
    pub last_iterated_at: Option<String>,

    /// Information about the exercise for which this solution was submitted.
    pub exercise: Exercise,

    /// Information about the language track containing the exercise for which
    /// this solution was submitted.
    pub track: Track,
}

/// Possible status of a solution to an exercise on the [Exercism website](https://exercism.org).
#[derive(
    Debug,
    Default,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    AsRefStr,
    Display,
    EnumString,
    IntoStaticStr,
    VariantNames,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Status {
    /// Exercise has been started, but no iteration has been submitted yet.
    #[default]
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
    #[serde(skip_serializing, other)]
    Unknown,
}

/// Possible mentoring status of a solution on the [Exercism website](https://exercism.org).
#[derive(
    Debug,
    Default,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    AsRefStr,
    Display,
    EnumString,
    IntoStaticStr,
    VariantNames,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum MentoringStatus {
    /// No mentoring has been required for this exercise.
    #[default]
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
    #[serde(skip_serializing, other)]
    Unknown,
}

/// Exercise for which a solution was submitted on the [Exercism website](https://exercism.org).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Exercise {
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

    /// URL of the icon representing this language track on the [Exercism website](https://exercism.org).
    pub icon_url: String,
}
