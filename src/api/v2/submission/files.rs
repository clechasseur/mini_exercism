//! Types related to files submitted to the [Exercism website](https://exercism.org) for a solution.

use serde::{Deserialize, Serialize};

/// Response to a query for files submitted for a solution on the [Exercism website](https://exercism.org) v2 API.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Response {
    /// Files that were part of the submission, including their content.
    pub files: Vec<File>,
}

/// Information about a file that is part of a submission, including its content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct File {
    /// Name of the file, including its path from the exercise directory's root.
    pub filename: String,

    /// File content.
    pub content: String,

    /// File digest (a hash of some kind).
    pub digest: String,
}
