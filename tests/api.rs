#[cfg(all(mini_exercism_enable_tests_with_real_endpoints, not(tarpaulin), not(coverage)))]
mod real_endpoints;
mod v1;
mod v2;
