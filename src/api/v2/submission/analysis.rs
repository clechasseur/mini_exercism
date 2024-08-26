//! Types related to submission analysis returned by the [Exercism website](https://exercism.org) v2 API.
//!
//! A submission analysis contains information about the output of the [analyzer](https://exercism.org/docs/building/tooling/analyzers)
//! and/or [representer](https://exercism.org/docs/building/tooling/representers) for the track.

use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, IntoStaticStr};

use crate::api::v2::user::Flair;

/// Feedback returned by the track [analyzer](https://exercism.org/docs/building/tooling/analyzers) for a submission.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnalyzerFeedback {
    /// Analysis summary. Not all analyzers provide this.
    #[serde(default)]
    pub summary: Option<String>,

    /// Comments returned by the analyzer.
    #[serde(default)]
    pub comments: Vec<AnalyzerComment>,
}

/// A specific comment returned by the track [analyzer](https://exercism.org/docs/building/tooling/analyzers).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnalyzerComment {
    /// Type of comment.
    ///
    /// For more information about analyzer comment types, see [this document](https://github.com/exercism/docs/blob/main/building/tooling/analyzers/interface.md#type-optional).
    #[serde(default, rename = "type")]
    pub comment_type: AnalyzerCommentType,

    /// HTML rendering of the comment.
    pub html: String,
}

/// Type of comment returned by the track [analyzer](https://exercism.org/docs/building/tooling/analyzers).
///
/// For more information about analyzer comment types, see [this document](https://github.com/exercism/docs/blob/main/building/tooling/analyzers/interface.md#type-optional).
#[derive(
    Debug,
    Default,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    Display,
    AsRefStr,
    IntoStaticStr,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum AnalyzerCommentType {
    /// Feedback about a fix that is absolutely essential.
    ///
    /// The analyzer documentation mentions that on the website (presumably meaning through the online editor),
    /// students are "soft-blocked" on essential comments (presumably meaning they cannot submit their solution).
    Essential,

    /// A comment with specific instructions on how to improve the solution.
    Actionable,

    /// Feedback that is informative, but not necessary (possibly a different way of doing something).
    #[default]
    Informative,

    /// A comment praising the student for the use of a specific technique in their submission.
    Celebratory,

    /// Unknown comment type.
    ///
    /// Included so that if new comment types are introduced in the website API later, this crate won't break (hopefully).
    #[serde(skip_serializing, other)]
    Unknown,
}

/// Feedback returned by the track [representer](https://exercism.org/docs/building/tooling/representers) for a submission.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepresenterFeedback {
    /// HTML rendering of the feedback.
    pub html: String,

    /// Information about the author of the feedback.
    pub author: FeedbackAuthor,

    /// Information about the editor of the feedback.
    ///
    /// Looking at the current website code, it's not clear if this field is ever set.
    #[serde(default)]
    pub editor: Option<FeedbackAuthor>,
}

/// Information about the author of some [`RepresenterFeedback`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FeedbackAuthor {
    /// Author's full name.
    pub name: String,

    /// Author's reputation.
    pub reputation: i32,

    /// Author's ["flair"](Flair), if any.
    #[serde(default)]
    pub flair: Option<Flair>,

    /// URL of the author's avatar on the website.
    pub avatar_url: String,

    /// URL of the author's public profile.
    ///
    /// Will only be set if the author made their profile public.
    #[serde(default)]
    pub profile_url: Option<String>,
}
