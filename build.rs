use std::env;

use rustc_version::Channel::Nightly;
use rustc_version::version_meta;

fn main() {
    println!("cargo:rerun-if-env-changed=CI");

    println!("cargo:rustc-check-cfg=cfg(nightly_rustc)");
    if version_meta().unwrap().channel <= Nightly {
        println!("cargo:rustc-cfg=nightly_rustc");
    }

    // On CI, do not run tests using the real endpoints.
    println!("cargo:rustc-check-cfg=cfg(skip_tests_with_real_endpoints)");
    if env::var("CI").is_ok() {
        println!("cargo:rustc-cfg=skip_tests_with_real_endpoints")
    }
}
