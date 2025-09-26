use crate::api::detail::{IntoQuery, QueryBuilder};
use crate::api::v2::exercises::Filters;
use crate::http::middleware::RequestBuilder;

#[derive(Debug)]
pub struct FiltersBuilderError;

impl IntoQuery for Filters<'_> {
    fn into_query(self, request: RequestBuilder) -> RequestBuilder {
        request
            .build_query(("criteria", self.criteria))
            .build_query_if(self.include_solutions, ("sideload", Some("solutions")))
    }
}
