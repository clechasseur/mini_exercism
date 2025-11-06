//! Types related to list of tracks returned by the [Exercism website](https://exercism.org) v2 API.

pub(crate) mod detail;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumString, IntoStaticStr, VariantNames};

use crate::api::v2::track::Track;

/// Filters that can be applied when fetching language tracks from the
/// [Exercism website](https://exercism.org) v2 API.
#[derive(Debug, Clone, Default, Builder)]
#[builder(
    derive(Debug),
    default,
    setter(strip_option),
    build_fn(private, name = "fallible_build", error = "detail::FiltersBuilderError")
)]
pub struct Filters<'a> {
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

    /// Language track's [status filter](StatusFilter).
    ///
    /// # Note
    ///
    /// Using this filter requires an authenticated query to the [Exercism website](https://exercism.org)
    /// v2 API, otherwise you will get a `500 Internal Server Error` (even when asking for
    /// [`All`](StatusFilter::All) tracks).
    pub status: Option<StatusFilter>,
}

// noinspection DuplicatedCode
impl<'a> Filters<'a> {
    /// Returns a builder for the [`Filters`] type.
    #[cfg_attr(not(coverage), tracing::instrument(level = "trace"))]
    pub fn builder() -> FiltersBuilder<'a> {
        FiltersBuilder::default()
    }
}

// noinspection DuplicatedCode
impl<'a> FiltersBuilder<'a> {
    /// Builds a new [`Filters`].
    #[cfg_attr(not(coverage), tracing::instrument(ret, level = "trace"))]
    pub fn build(&self) -> Filters<'a> {
        self.fallible_build()
            .expect("All fields should have had default values")
    }
}

/// Possible status filter of [Exercism](https://exercism.org) language tracks.
#[derive(
    Debug,
    Default,
    Copy,
    Clone,
    PartialEq,
    Eq,
    AsRefStr,
    Display,
    EnumString,
    IntoStaticStr,
    VariantNames,
)]
#[strum(serialize_all = "snake_case")]
pub enum StatusFilter {
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
pub struct Response {
    /// List of [Exercism](https://exercism.org) language tracks.
    ///
    /// Usually sorted alphabetically by track name, with tracks joined by the user first
    /// (if query is performed with [`credentials`](crate::api::v2::ClientBuilder::credentials)).
    pub tracks: Vec<Track>,
}
