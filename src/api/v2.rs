//! Types and functions to interact with the [Exercism website](https://exercism.org) v2 API.

pub mod exercise;
pub mod exercises;
pub mod iteration;
pub mod solution;
pub mod solutions;
pub mod submission;
pub mod tests;
pub mod track;
pub mod tracks;
pub mod user;

use crate::Result;

/// Default base URL for the [Exercism website](https://exercism.org) v2 API.
pub const DEFAULT_V2_API_BASE_URL: &str = "https://exercism.org/api/v2";

define_api_client! {
    /// Client for the [Exercism website](https://exercism.org) v2 API.
    ///
    /// This API is undocumented and is mostly used by the website itself to fetch information.
    pub struct Client(DEFAULT_V2_API_BASE_URL);
}

impl Client {
    /// Returns a list of [Exercism tracks](https://exercism.org/tracks).
    ///
    /// - If the request is performed anonymously, will return a list of all tracks
    ///   supported on the website.
    /// - If the request is performed with [`credentials`](ClientBuilder::credentials),
    ///   tracks that the user has joined will be identified by the
    ///   [`is_joined`](track::Track::is_joined) field.
    ///
    /// The list of tracks can optionally be filtered using [`Filters`](tracks::Filters).
    ///
    /// # Errors
    ///
    /// - [`ApiError`]: Error while fetching track information from API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mini_exercism::api;
    /// use mini_exercism::api::v2::tracks::Filters;
    /// use mini_exercism::api::v2::tracks::StatusFilter::Joined;
    /// use mini_exercism::core::Credentials;
    ///
    /// async fn get_joined_tracks(api_token: &str) -> anyhow::Result<Vec<String>> {
    ///     let credentials = Credentials::from_api_token(api_token);
    ///     let client = api::v2::Client::builder().credentials(credentials).build();
    ///
    ///     let filters = Filters::builder().status(Joined).build();
    ///     let tracks = client.get_tracks(Some(filters)).await?.tracks;
    ///
    ///     Ok(tracks.into_iter().map(|track| track.name).collect())
    /// }
    /// ```
    ///
    /// [`ApiError`]: crate::Error::ApiError
    pub async fn get_tracks(
        &self,
        filters: Option<tracks::Filters<'_>>,
    ) -> Result<tracks::Response> {
        self.api_client
            .get("/tracks")
            .query(filters)
            .execute()
            .await
    }

    /// Returns a list of exercises for an [Exercism](https://exercism.org) `track`,
    /// optionally loading the user's solutions.
    ///
    /// - If the request is performed anonymously, returns a list of all exercises in
    ///   the track. Each exercise's [`is_external`](exercise::Exercise::is_external) field will
    ///   be set to `true`.
    /// - If the request is performed with [`credentials`](ClientBuilder::credentials),
    ///   returns a list of all exercises in the track, with information about whether
    ///   each exercise has been [unlocked](exercise::Exercise::is_unlocked) by the user. Each
    ///   exercise's [`is_external`](exercise::Exercise::is_external) field will be set to `false`.
    ///   Additionally, if the `filters` parameter's [`include_solutions`](exercises::Filters::include_solutions)
    ///   is set to `true`, the response will contain a list of solutions the user has submitted
    ///   for the track's exercises.
    ///
    /// The list of exercises can optionally be filtered using [`Filters`](exercises::Filters).
    ///
    /// # Notes
    ///
    /// If the `filters` parameter's [`include_solutions`](exercises::Filters::include_solutions) is
    /// set to `true`, the returned [`solutions`](exercises::Response::solutions) will return all
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
    /// use mini_exercism::api::v2::exercises::Filters;
    /// use mini_exercism::core::Credentials;
    ///
    /// async fn get_published_solution_uuids(
    ///     api_token: &str,
    ///     track: &str,
    /// ) -> anyhow::Result<Vec<String>> {
    ///     let credentials = Credentials::from_api_token(api_token);
    ///     let client = api::v2::Client::builder().credentials(credentials).build();
    ///
    ///     let filters = Filters::builder().include_solutions(true).build();
    ///     let solutions = client.get_exercises(track, Some(filters)).await?.solutions;
    ///
    ///     Ok(solutions
    ///         .into_iter()
    ///         .filter(|solution| solution.published_at.is_some())
    ///         .map(|solution| solution.uuid)
    ///         .collect())
    /// }
    /// ```
    ///
    /// [`ApiError`]: crate::Error::ApiError
    pub async fn get_exercises(
        &self,
        track: &str,
        filters: Option<exercises::Filters<'_>>,
    ) -> Result<exercises::Response> {
        self.api_client
            .get(format!("/tracks/{}/exercises", track))
            .query(filters)
            .execute()
            .await
    }

    /// Returns a list of [Exercism](https://exercism.org) solutions for the user.
    ///
    /// This request cannot be performed anonymously; doing so will result in an [`ApiError`].
    ///
    /// The list of solutions can optionally be filtered using [`Filters`](solutions::Filters).
    ///
    /// The list is paginated. By default, the first page is returned. To iterate pages, pass in
    /// [`paging`](solutions::Paging) information. It's also possible to control the [`sort_order`](solutions::SortOrder)
    /// of the solutions; if not specified, the default sort order is to return solutions with the
    /// [most stars first](solutions::SortOrder::MostStarred).
    ///
    /// # Errors
    ///
    /// - [`ApiError`]: Error while fetching solutions information from API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mini_exercism::api;
    /// use mini_exercism::api::v2::solution::Solution;
    /// use mini_exercism::api::v2::solutions::{Filters, Paging, SortOrder};
    /// use mini_exercism::core::Credentials;
    ///
    /// async fn get_user_solutions(
    ///     api_token: &str,
    ///     filters: Option<Filters<'_>>,
    ///     sort_order: Option<SortOrder>,
    /// ) -> anyhow::Result<Vec<Solution>> {
    ///     let credentials = Credentials::from_api_token(api_token);
    ///     let client = api::v2::Client::builder().credentials(credentials).build();
    ///
    ///     let mut solutions = Vec::new();
    ///     let mut page = 1i64;
    ///     loop {
    ///         let paging = Paging::for_page(page);
    ///         let paged_solutions = client
    ///             .get_solutions(filters.clone(), Some(paging), sort_order)
    ///             .await?
    ///             .results;
    ///         if paged_solutions.is_empty() {
    ///             break;
    ///         }
    ///
    ///         solutions.extend(paged_solutions.into_iter());
    ///         page += 1;
    ///     }
    ///
    ///     Ok(solutions)
    /// }
    /// ```
    ///
    /// [`ApiError`]: crate::Error::ApiError
    pub async fn get_solutions(
        &self,
        filters: Option<solutions::Filters<'_>>,
        paging: Option<solutions::Paging>,
        sort_order: Option<solutions::SortOrder>,
    ) -> Result<solutions::Response> {
        self.api_client
            .get("/solutions")
            .query(filters)
            .query(paging)
            .query(("order", sort_order))
            .execute()
            .await
    }

    /// Returns information about a specific solution submitted by the user.
    ///
    /// This request cannot be performed anonymously; doing so will result in an [`ApiError`].
    ///
    /// It's possible to also sideload the solution's iterations.
    ///
    /// # Errors
    ///
    /// - [`ApiError`]: Error while fetching solution information from API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mini_exercism::api;
    /// use mini_exercism::api::v2::iteration::Iteration;
    /// use mini_exercism::core::Credentials;
    ///
    /// async fn get_solution_iterations(
    ///     api_token: &str,
    ///     solution_uuid: &str,
    /// ) -> anyhow::Result<Vec<Iteration>> {
    ///     let credentials = Credentials::from_api_token(api_token);
    ///     let client = api::v2::Client::builder().credentials(credentials).build();
    ///
    ///     Ok(client.get_solution(solution_uuid, true).await?.iterations)
    /// }
    /// ```
    ///
    /// [`ApiError`]: crate::Error::ApiError
    pub async fn get_solution(
        &self,
        uuid: &str,
        include_iterations: bool,
    ) -> Result<solution::Response> {
        self.api_client
            .get(format!("/solutions/{}", uuid))
            .query(("sideload", include_iterations.then_some("iterations")))
            .execute()
            .await
    }

    /// Returns information about the files submitted for a solution iteration.
    ///
    /// This request cannot be performed anonymously, unless the submission's iteration has been [published](crate::api::v2::iteration::Iteration::is_published).
    ///
    /// # Errors
    ///
    /// - [`ApiError`]: Error while fetching submitted files information from API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mini_exercism::api;
    /// use mini_exercism::api::v2::submission;
    /// use mini_exercism::core::Credentials;
    ///
    /// async fn get_solution_files(
    ///     api_token: &str,
    ///     solution_uuid: &str,
    /// ) -> anyhow::Result<Vec<submission::files::File>> {
    ///     let credentials = Credentials::from_api_token(api_token);
    ///     let client = api::v2::Client::builder().credentials(credentials).build();
    ///
    ///     Ok(client
    ///         .get_solution(solution_uuid, true)
    ///         .await?
    ///         .iterations
    ///         .into_iter()
    ///         .find(|iteration| iteration.is_latest)
    ///         .map(|iteration| iteration.files)
    ///         .unwrap_or_default())
    /// }
    /// ```
    ///
    /// [`ApiError`]: crate::Error::ApiError
    pub async fn get_submission_files(
        &self,
        solution_uuid: &str,
        submission_uuid: &str,
    ) -> Result<submission::files::Response> {
        self.api_client
            .get(format!("/solutions/{}/submissions/{}/files", solution_uuid, submission_uuid))
            .execute()
            .await
    }
}
