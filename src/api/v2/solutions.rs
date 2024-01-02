//! Types related to list of solutions returned by the [Exercism website](https://exercism.org) v2 API.

pub(crate) mod detail;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, Display, IntoStaticStr};

use crate::api::v2::solution::{MentoringStatus, Solution, Status};
use crate::api::v2::tests;

/// Filters that can be applied when fetching solutions from the
/// [Exercism website](https://exercism.org) v2 API.
#[derive(Debug, Clone, Default, Builder)]
#[builder(
    derive(Debug),
    default,
    setter(strip_option),
    build_fn(private, name = "fallible_build", error = "detail::FiltersBuilderError")
)]
pub struct Filters<'a> {
    /// Criteria used to filter solutions.
    ///
    /// Applied to both the solution's exercise's title or its track's title.
    #[builder(setter(into))]
    pub criteria: Option<&'a str>,

    /// Name (e.g. slug) of the track containing solution's exercise.
    #[builder(setter(into))]
    pub track: Option<&'a str>,

    /// Solution [status](Status).
    pub status: Option<Status>,

    /// Solution's [mentoring status](MentoringStatus).
    pub mentoring_status: Option<MentoringStatus>,

    /// Whether the solution is out-of-date or not.
    ///
    /// If set, only solutions that are out-of-date (`true`) or up-to-date (`false`)
    /// will be included.
    pub is_out_of_date: Option<bool>,

    /// Possible status of the solution's published iteration's tests.
    ///
    /// # Notes
    ///
    /// The difference between a "head test run" and a normal test run is somewhat explained
    /// in the Exercism website source code [here](https://github.com/exercism/website/blob/main/app/models/submission.rb).
    #[builder(setter(into, each(name = "published_iteration_tests_status")))]
    pub published_iteration_tests_statuses: Vec<tests::Status>,

    /// Possible status of the solution's published iteration's head tests.
    ///
    /// Corresponds to the value found in [`Solution::published_iteration_head_tests_status`](crate::api::v2::solution::Solution::published_iteration_head_tests_status).
    ///
    /// # Notes
    ///
    /// The difference between a "head test run" and a normal test run is somewhat explained
    /// in the Exercism website source code [here](https://github.com/exercism/website/blob/main/app/models/submission.rb).
    #[builder(setter(into, each(name = "published_iteration_head_tests_status")))]
    pub published_iteration_head_tests_statuses: Vec<tests::Status>,
}

impl<'a> Filters<'a> {
    /// Returns a builder for the [`Filters`] type.
    pub fn builder() -> FiltersBuilder<'a> {
        FiltersBuilder::default()
    }
}

impl<'a> FiltersBuilder<'a> {
    /// Adds a filter to only return out-of-date solutions.
    pub fn out_of_date(&mut self) -> &mut Self {
        self.is_out_of_date(true)
    }

    /// Adds a filter to only return up-to-date solutions.
    pub fn up_to_date(&mut self) -> &mut Self {
        self.is_out_of_date(false)
    }

    /// Builds a new [`Filters`].
    pub fn build(&self) -> Filters<'a> {
        self.fallible_build()
            .expect("All fields should have had default values")
    }
}

/// Paging information when performing paged queries for solutions to the
/// [Exercism website](https://exercism.org) v2 API.
#[derive(Debug, Copy, Clone)]
pub struct Paging {
    /// Page number (1-based).
    pub page: i64,

    /// Number of solutions to return in each page.
    ///
    /// If not specified, the default value (documented [here](https://github.com/exercism/website/blob/main/app/commands/solution/search_user_solutions.rb#L1))
    /// will be used.
    pub per_page: Option<i64>,
}

impl Paging {
    /// Returns [`Paging`] information for the given `page`.
    ///
    /// The page size will be left blank, so the default value will be used. The page size
    /// can also be set via [`and_per_page`](Self::and_per_page).
    pub fn for_page(page: i64) -> Self {
        Self { page, per_page: None }
    }

    /// Further specifies the page size for the request.
    ///
    /// # Examples
    ///
    /// ```
    /// use mini_exercism::api::v2::solutions::Paging;
    ///
    /// let paging = Paging::for_page(1).and_per_page(10);
    /// assert_eq!(1, paging.page);
    /// assert_eq!(Some(10), paging.per_page);
    /// ```
    pub fn and_per_page(self, per_page: i64) -> Self {
        Self { page: self.page, per_page: Some(per_page) }
    }
}

/// Possible ways to sort solutions returned by the [Exercism website](https://exercism.org) v2 API.
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
pub enum SortOrder {
    /// Return solutions from newest to oldest.
    NewestFirst,

    /// Return solutions from oldest to newest.
    OldestFirst,

    /// Return solutions based on number of stars (descending).
    ///
    /// The value used is the same as [`Solution::num_stars`](crate::api::v2::solution::Solution::num_stars).
    #[default]
    MostStarred,
}

/// Response to a query for solutions on the [Exercism website](https://exercism.org) v2 API.
/// Responses are paginated, so this only returns one page of results.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Response {
    /// List of solutions in the current page.
    pub results: Vec<Solution>,

    /// Metadata containing paging information.
    pub meta: ResponseMeta,
}

/// Metadata attached to a response to a query for solutions on the [Exercism website](https://exercism.org) v2 API.
/// Contains paging information.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResponseMeta {
    /// Current page number (1-based).
    pub current_page: i64,

    /// Total number of solutions matching the query.
    pub total_count: i64,

    /// Total number of pages that could be returned for the query.
    pub total_pages: i64,
}
