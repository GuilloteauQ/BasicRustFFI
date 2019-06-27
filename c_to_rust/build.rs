extern crate cc;
use std::env;
use std::process::Command;

fn main() {
    let target_dir = String::from("target/"); //env::var("CARGO_TARGET_DIR").unwrap();
    assert!(Command::new("cc")
        .args(&["-fPIC", "-O", "-g", "-c", "src/add.c", "src/add.h",])
        .status()
        .expect("Could not compile")
        .success());

    assert!(Command::new("mv")
        .args(&["add.o", target_dir.as_str(),])
        .status()
        .unwrap()
        .success());

    assert!(Command::new("ar")
        .args(&[
            "rcs",
            format!("{}/libadd.a", target_dir).as_str(),
            format!("{}/add.o", target_dir).as_str(),
        ])
        .status()
        .expect("Could not link")
        .success());

    println!("cargo:rustc-link-lib=add"); // the "-l" flag
    println!("cargo:rustc-link-search={}", target_dir); // the "-L" flag
}
