//! Types related to test runs submitted to the [Exercism website](https://exercism.org) v2 API.

use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, Display};

/// Possible status of a test run on the [Exercism website](https://exercism.org).
///
/// # Notes
///
/// Currently, tests status are returned in different places by the v2 API. Technically, they are
/// all different enums in the website code; however, since they all represent the same statuses,
/// the same enum will be used on the Rust side, for simplicity.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Display, AsRefStr)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Status {
    /// Tests have not been queued yet.
    #[default]
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
    #[serde(skip_serializing, other)]
    Unknown,
}
