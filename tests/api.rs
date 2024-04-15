#[cfg(all(not(skip_tests_with_real_endpoints), not(tarpaulin)))]
mod real_endpoints;
mod v1;
mod v2;
