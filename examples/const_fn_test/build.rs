use std::env;
use std::process::Command;
use std::str;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    if nightly() {
        println!("cargo:rustc-cfg=nightly");
    }
}

fn nightly() -> bool {
    env::var_os("RUSTC")
        .and_then(|rustc| Command::new(rustc).arg("--version").output().ok())
        .and_then(|output| {
            str::from_utf8(&output.stdout)
                .ok()
                .map(|version| version.contains("nightly"))
        })
        .unwrap_or(false)
}
