use std::env;

use rustc_version::version_meta;
use rustc_version::Channel::Nightly;

fn main() {
    println!("cargo:rerun-if-env-changed=CI");

    if version_meta().unwrap().channel <= Nightly {
        println!("cargo:rustc-cfg=nightly_rustc");
    }

    // On CI, do not run tests using the real endpoints.
    if env::var("CI").is_ok() {
        println!("cargo:rustc-cfg=skip_tests_with_real_endpoints")
    }
}
