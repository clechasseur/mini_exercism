fn main() {
    println!("cargo:rerun-if-env-changed=CI");

    // On CI, do not run integration tests.
    if option_env!("CI").is_some() {
        println!("cargo:rustc-cfg=skip_integration_tests")
    }
}
