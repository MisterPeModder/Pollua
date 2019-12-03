extern crate rustc_version;

fn main() {
    check_rustc_version();
}

fn check_rustc_version() {
    if let rustc_version::Channel::Nightly = rustc_version::version_meta().unwrap().channel {
        println!("cargo:rustc-cfg=rust_nightly");
    }
}
