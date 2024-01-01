use reqwest::RequestBuilder;

use crate::api::detail::{IntoQuery, QueryBuilder};
use crate::api::v2::tracks::Filters;

#[derive(Debug)]
pub struct FiltersBuilderError;

impl<'a> IntoQuery for Filters<'a> {
    fn into_query(self, request: RequestBuilder) -> RequestBuilder {
        request
            .build_query(("criteria", self.criteria))
            .build_query(("tags[]", self.tags))
            .build_query(("status", self.status))
    }
}
