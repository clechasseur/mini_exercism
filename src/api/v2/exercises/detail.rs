use reqwest::RequestBuilder;

use crate::api::detail::{IntoQuery, QueryBuilder};
use crate::api::v2::exercises::Filters;

#[derive(Debug)]
pub struct FiltersBuilderError;

impl<'a> IntoQuery for Filters<'a> {
    fn into_query(self, request: RequestBuilder) -> RequestBuilder {
        request
            .build_query(("criteria", self.criteria))
            .build_query_if(self.include_solutions, ("sideload", Some("solutions")))
    }
}
