// These tests hit the real Exercism.org endpoints.
// To prevent being rate limited (and possibly angering the maintainers), they are not run on CI.
// To enable them locally, create a `.cargo/config.toml` in your home directory and include the
// following configuration (in addition to any existing ones you have):
//
// [build]
// rustflags = "--cfg mini_exercism_enable_tests_with_real_endpoints"
//
// For more info on configuring Cargo, see:
// https://doc.rust-lang.org/cargo/reference/config.html#configuration

mod v1;
mod v2;
