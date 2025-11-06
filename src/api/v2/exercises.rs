//! Types related to list of exercises returned by the [Exercism website](https://exercism.org) v2 API.

pub(crate) mod detail;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::api::v2::exercise::Exercise;
use crate::api::v2::solution::Solution;

/// Filters that can be applied when fetching exercises from the
/// [Exercism website](https://exercism.org) v2 API.
#[derive(Debug, Clone, Default, Builder)]
#[builder(
    derive(Debug),
    default,
    setter(strip_option),
    build_fn(private, name = "fallible_build", error = "detail::FiltersBuilderError")
)]
pub struct Filters<'a> {
    /// Criteria used to filter exercises.
    ///
    /// Applied to both exercise [`name`](Exercise::name)s (e.g. slugs) and [`title`](Exercise::title)s.
    #[builder(setter(into))]
    pub criteria: Option<&'a str>,

    /// Whether to include solutions in the response.
    ///
    /// Only has an effect if the query is performed with [`credentials`](crate::api::v2::ClientBuilder::credentials).
    pub include_solutions: bool,
}

// noinspection DuplicatedCode
impl<'a> Filters<'a> {
    /// Returns a builder for the [`Filters`] type.
    #[cfg_attr(not(coverage), tracing::instrument(level = "trace"))]
    pub fn builder() -> FiltersBuilder<'a> {
        FiltersBuilder::default()
    }
}

//noinspection DuplicatedCode
impl<'a> FiltersBuilder<'a> {
    /// Builds a new [`Filters`].
    #[cfg_attr(not(coverage), tracing::instrument(ret, level = "trace"))]
    pub fn build(&self) -> Filters<'a> {
        self.fallible_build()
            .expect("All fields should have had default values")
    }
}

/// Response to a query for exercises on the [Exercism website](https://exercism.org) v2 API.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Response {
    /// List of exercises for the requested track.
    ///
    /// The ordering depends on the type of query performed and matches that seen on the website.
    pub exercises: Vec<Exercise>,

    /// List of solutions submitted for exercises in this track.
    ///
    /// Will only be filled if the [`include_solutions`](Filters::include_solutions)
    /// field of the query's [`Filters`] is set to `true`.
    ///
    /// # Note
    ///
    /// Even if [`include_solutions`](Filters::include_solutions) is set to `true`, solutions
    /// will not be fetched if the API is queried anonymously.
    #[serde(default)]
    pub solutions: Vec<Solution>,
}
