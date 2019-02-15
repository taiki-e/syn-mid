use std::env;
use std::process::Command;
use std::str;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let (minor, nightly) = match rustc_minor_version() {
        Some(x) => x,
        None => return,
    };

    if minor >= 31 || nightly {
        println!("cargo:rustc-cfg=min_const_fn");
    }
    if minor >= 33 || nightly {
        println!("cargo:rustc-cfg=const_let");
    }
    if nightly {
        println!("cargo:rustc-cfg=const_unstable");
    }
}

fn rustc_minor_version() -> Option<(u32, bool)> {
    env::var_os("RUSTC")
        .and_then(|rustc| Command::new(rustc).arg("--version").output().ok())
        .and_then(|output| {
            str::from_utf8(&output.stdout).ok().and_then(|version| {
                let nightly = version.contains("nightly");
                let mut pieces = version.split('.');
                if pieces.next() != Some("rustc 1") {
                    return None;
                }
                pieces.next()?.parse().ok().map(|minor| (minor, nightly))
            })
        })
}