#[cfg(all(not(skip_tests_with_real_endpoints), not(tarpaulin), not(coverage)))]
mod real_endpoints;
mod v1;
mod v2;
