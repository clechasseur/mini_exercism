use crate::api::detail::{IntoQuery, QueryBuilder};
use crate::api::v2::tracks::Filters;
use crate::http;

#[derive(Debug)]
pub struct FiltersBuilderError;

impl IntoQuery for Filters<'_> {
    fn into_query(self, request: http::RequestBuilder) -> http::RequestBuilder {
        request
            .build_query(("criteria", self.criteria))
            .build_query(("tags[]", self.tags))
            .build_query(("status", self.status))
    }
}
