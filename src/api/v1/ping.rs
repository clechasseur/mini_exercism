//! Types related to ping requests to the [Exercism website](https://exercism.org) v1 API.

use serde::{Deserialize, Serialize};

/// Response to a ping request to the [Exercism website](https://exercism.org) v1 API.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Response {
    /// Information about the status of the [Exercism](https://exercism.org) services.
    pub status: ServiceStatus,
}

/// Status of services, as returned by the [Exercism website](https://exercism.org) v1 API.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceStatus {
    /// Whether the [Exercism website](https://exercism.org) is up and running.
    pub website: bool,

    /// Whether the database backing the [Exercism website](https://exercism.org) is working.
    pub database: bool,
}
