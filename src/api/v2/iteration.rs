//! Types related to iterations returned by the [Exercism website](https://exercism.org) v2 API.
//!
//! Solutions to exercises can have multiple iterations.

pub(crate) mod detail;

use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, IntoStaticStr};

use crate::api::v2::submission::analysis::{AnalyzerFeedback, RepresenterFeedback};
use crate::api::v2::{submission, tests};

/// Information about a specific iteration of a [`Solution`](crate::api::v2::solution::Solution)
/// submitted to the [Exercism website](https://exercism.org).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Iteration {
    /// Iteration unique ID.
    pub uuid: String,

    /// Unique ID of the iteration's submission.
    ///
    /// An iteration's submission is tied to the actual data sent for the iteration.
    ///
    /// Will be `None` for deleted iterations.
    #[serde(default)]
    pub submission_uuid: Option<String>,

    /// 1-based index of the iteration.
    ///
    /// This is incremented every time an iteration is submitted for an exercise.
    #[serde(rename = "idx")]
    pub index: i32,

    /// Iteration status.
    pub status: Status,

    /// Number of [essential](submission::analysis::AnalyzerCommentType::Essential) comments returned during this
    /// iteration's submission's automated analysis.
    pub num_essential_automated_comments: i32,

    /// Number of [actionable](submission::analysis::AnalyzerCommentType::Actionable) comments returned during this
    /// iteration's submission's automated analysis.
    pub num_actionable_automated_comments: i32,

    /// Number of [non-actionable](submission::analysis::AnalyzerCommentType::Informative) comments returned during this
    /// iteration's submission's automated analysis.
    ///
    /// # Notes
    ///
    /// In the analyzer documentation, this level of feedback is known as "informative".
    pub num_non_actionable_automated_comments: i32,

    /// Number of [celebratory](submission::analysis::AnalyzerCommentType::Celebratory) comments returned during this
    /// iteration's submission's automated analysis.
    pub num_celebratory_automated_comments: i32,

    /// How the iteration was submitted.
    ///
    /// This is a free-form string, but is probably constrained to a few values currently.
    /// Values seen so far:
    ///
    /// | Value | Submission method |
    /// |-------|-------------------|
    /// | `cli` | [Exercism CLI]    |
    /// | `api` | Online editor     |
    ///
    /// [Exercism CLI]: https://exercism.org/docs/using/solving-exercises/working-locally
    pub submission_method: String,

    /// Date/time when the iteration was created, in ISO-8601 format.
    pub created_at: String,

    /// Status of this iteration's submission's test run.
    pub tests_status: tests::Status,

    /// Feedback provided by the track's [representer](https://exercism.org/docs/building/tooling/representers) for
    /// this iteration.
    ///
    /// Will be `None` if the representer did not provide any feedback (or if the track has no representer).
    ///
    /// # Notes
    ///
    /// This field is only filled if automated feedback is sideloaded, which is not currently possible with the
    /// v2 API [`Client`](crate::api::v2::Client).
    #[serde(default, deserialize_with = "detail::deserialize_optional_feedback")]
    pub representer_feedback: Option<RepresenterFeedback>,

    /// Feedback provided by the track's [analyzer](https://exercism.org/docs/building/tooling/analyzers) for
    /// this iteration.
    ///
    /// Will be `None` if the analyzer did not provide any feedback (or if the track has no analyzer).
    ///
    /// # Notes
    ///
    /// This field is only filled if automated feedback is sideloaded, which is not currently possible with the
    /// v2 API [`Client`](crate::api::v2::Client).
    #[serde(default, deserialize_with = "detail::deserialize_optional_feedback")]
    pub analyzer_feedback: Option<AnalyzerFeedback>,

    /// Whether this iteration has been published.
    ///
    /// When a solution is published, multiple iterations can be published.
    pub is_published: bool,

    /// Whether this is the solution's latest iteration.
    ///
    /// # Notes
    ///
    /// This field is not sent by the v2 API for deleted iterations. Presumably, if the
    /// last iteration of a solution is deleted, then the next-to-last will have `is_latest`
    /// set to `true` in its stead.
    #[serde(default)]
    pub is_latest: bool,

    /// Information about the iteration's submitted files, including their content.
    ///
    /// # Notes
    ///
    /// This field is only filled if files are sideloaded, which is not currently possible with the v2 API [`Client`](crate::api::v2::Client).
    #[serde(default)]
    pub files: Vec<submission::files::File>,

    /// Collection of links pertaining to the iteration.
    pub links: Links,
}

/// Possible status of a solution iteration submitted to the [Exercism website](https://exercism.org).
#[derive(
    Debug,
    Default,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    Display,
    AsRefStr,
    IntoStaticStr,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Status {
    /// Iteration has been submitted but has not been queued for testing yet.
    ///
    /// # Notes
    ///
    /// If a track does not have a test runner, iterations would presumably stay in this state forever.
    #[default]
    Untested,

    /// Tests for this iteration have been queued, but have not finished executing.
    Testing,

    /// Tests for this iteration have failed.
    TestsFailed,

    /// Tests for this iteration have completed successfully; automated feedback analysis is in progress.
    Analyzing,

    /// Tests for this iteration have completed successfully and essential automated feedback has been provided.
    ///
    /// For more information about types of automated feedback, see [this analyzer document](https://github.com/exercism/docs/blob/main/building/tooling/analyzers/interface.md#type-optional).
    EssentialAutomatedFeedback,

    /// Tests for this iteration have completed successfully and actionable automated feedback has been provided.
    ///
    /// For more information about types of automated feedback, see [this analyzer document](https://github.com/exercism/docs/blob/main/building/tooling/analyzers/interface.md#type-optional).
    ActionableAutomatedFeedback,

    /// Tests for this iteration have completed successfully and celebratory automated feedback has been provided.
    ///
    /// For more information about types of automated feedback, see [this analyzer document](https://github.com/exercism/docs/blob/main/building/tooling/analyzers/interface.md#type-optional).
    CelebratoryAutomatedFeedback,

    /// Tests for this iteration have completed successfully and non-actionable automated feedback has been provided.
    ///
    /// For more information about types of automated feedback, see [this analyzer document](https://github.com/exercism/docs/blob/main/building/tooling/analyzers/interface.md#type-optional).
    ///
    /// # Notes
    ///
    /// In the analyzer documentation, this level of feedback is known as [informative](submission::analysis::AnalyzerCommentType::Informative).
    NonActionableAutomatedFeedback,

    /// Tests for this iteration have completed successfully; no automated feedback was provided.
    NoAutomatedFeedback,

    /// This iteration has been deleted by the user.
    Deleted,

    /// Unknown status.
    ///
    /// Included so that if new iteration statuses are introduced in the website API later,
    /// this crate won't break (hopefully).
    #[serde(skip_serializing, other)]
    Unknown,
}

/// Links pertaining to an [Exercism](https://exercism.org) iteration returned by the v2 API.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Links {
    /// URL of the iteration on the [Exercism website](https://exercism.org).
    ///
    /// This URL is only valid for the user that submitted the iteration.
    #[serde(rename = "self")]
    pub self_path: String,

    /// API URL that can be used to fetch the iteration's submission's automated feedback.
    ///
    /// Will be `None` if the iteration has no automated feedback (if it is deleted, for example).
    #[serde(default)]
    pub automated_feedback: Option<String>,

    /// API URL of the iteration. Performing an HTTP `DELETE` on this URL will delete the iteration.
    ///
    /// Will be `None` if the iteration is already deleted.
    #[serde(default)]
    pub delete: Option<String>,

    /// URL of the exercise on the [Exercism website](https://exercism.org).
    ///
    /// For the user that submitted the iteration, this URL will point to their solution.
    pub solution: String,

    /// API URL of the iteration's submission's test run.
    ///
    /// Will be `None` if the iteration has no test run (if it is deleted, for example).
    #[serde(default)]
    pub test_run: Option<String>,

    /// API URL of the iteration's submission's files (with content).
    ///
    /// Will be `None` for deleted iterations.
    #[serde(default)]
    pub files: Option<String>,
}
