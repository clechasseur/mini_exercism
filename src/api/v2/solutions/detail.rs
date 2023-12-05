use reqwest::RequestBuilder;
use strum_macros::AsRefStr;

use crate::api::detail::{IntoQuery, QueryBuilder};
use crate::api::v2::solutions::detail::SyncStatus::{OutOfDate, UpToDate};
use crate::api::v2::solutions::{Filters, Paging};

#[derive(Debug)]
pub struct FiltersBuilderError;

impl<'a> IntoQuery for Filters<'a> {
    fn into_query(self, request: RequestBuilder) -> RequestBuilder {
        request
            .build_query(("criteria", self.criteria))
            .build_query(("track_slug", self.track))
            .build_query(("status", self.status))
            .build_query(("mentoring_status", self.mentoring_status))
            .build_query(("sync_status", self.is_out_of_date.map(SyncStatus::for_out_of_date)))
            .build_joined_query("tests_status", self.published_iteration_tests_statuses)
            .build_joined_query("head_tests_status", self.published_iteration_head_tests_statuses)
    }
}

impl IntoQuery for Paging {
    fn into_query(self, request: RequestBuilder) -> RequestBuilder {
        request
            .build_query(("page", Some(self.page.to_string())))
            .build_query(("per_page", self.per_page.map(|pp| pp.to_string())))
    }
}

#[derive(Debug, Copy, Clone, AsRefStr)]
#[strum(serialize_all = "snake_case")]
enum SyncStatus {
    UpToDate,
    OutOfDate,
}

impl SyncStatus {
    fn for_out_of_date(is_out_of_date: bool) -> Self {
        if is_out_of_date {
            OutOfDate
        } else {
            UpToDate
        }
    }
}
