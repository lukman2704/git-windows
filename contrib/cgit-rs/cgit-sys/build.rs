use std::env;
use std::path::PathBuf;

pub fn main() -> std::io::Result<()> {
    let crate_root = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let git_root = crate_root.join("../../..");
    let dst = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    let make_output = std::process::Command::new("make")
        .env_remove("PROFILE")
        .current_dir(git_root.clone())
        .args([
            "CFLAGS=-fvisibility=hidden",
            "contrib/cgit-rs/cgit-sys/libcgit.a",
        ])
        .output()
        .expect("Make failed to run");
    if !make_output.status.success() {
        panic!(
            "Make failed:\n  stdout = {}\n  stderr = {}\n",
            String::from_utf8(make_output.stdout).unwrap(),
            String::from_utf8(make_output.stderr).unwrap()
        );
    }
    std::fs::copy(crate_root.join("libcgit.a"), dst.join("libcgit.a"))?;
    println!("cargo::rustc-link-search=native={}", dst.display());
    println!("cargo::rustc-link-lib=cgit");
    println!("cargo::rustc-link-lib=z");
    println!("cargo::rerun-if-changed={}", git_root.display());

    Ok(())
}