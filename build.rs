use std::env;

fn main() {
    println!("cargo:rerun-if-env-changed=CI");

    // On CI, do not run integration tests.
    if env::var("CI").is_ok() {
        println!("cargo:rustc-cfg=skip_integration_tests")
    }
}
