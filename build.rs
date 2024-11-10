use std::env;
use std::path::PathBuf;
use std::process::Command;

fn docker_tdlib(out_dir: &str) {
    // Obtain build directory
    let td_dir = format!("{out_dir}/tdlib");

    // Run build script
    let build_script = Command::new("./build.sh").arg(&td_dir).status().unwrap();
    assert!(build_script.success());
}

// fn parse_tdlib_api() {}

fn main() {
    // Tell Cargo to rebuild if the build scripts change
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rerun-if-changed={manifest_dir}/build.rs");
    println!("cargo:rerun-if-changed={manifest_dir}/build.sh");
    println!("cargo:rerun-if-changed={manifest_dir}/headers.h");
    println!("cargo:rerun-if-changed={manifest_dir}/build/Dockerfile");

    // Get out_dir
    let out_dir = env::var("OUT_DIR").unwrap();

    // Build library
    docker_tdlib(&out_dir);

    // Run linkers
    let include_dir = format!("{out_dir}/bin/include");
    let lib_dir = format!("{out_dir}/bin/lib");

    // Make header paths
    println!("cargo:root={out_dir}");
    println!("cargo:include={include_dir}");

    // Create links
    // SRC: tdlib-sys
    println!("cargo:rustc-link-search=native={lib_dir}");
    println!("cargo:rustc-link-search=native={}/bin", out_dir);
    println!("cargo:rustc-link-lib=static=tdjson_static");
    println!("cargo:rustc-link-lib=static=tdjson_private");
    println!("cargo:rustc-link-lib=static=tdclient");
    println!("cargo:rustc-link-lib=static=tdcore");
    println!("cargo:rustc-link-lib=static=tdmtproto");
    println!("cargo:rustc-link-lib=static=tdapi");
    println!("cargo:rustc-link-lib=static=tdactor");
    println!("cargo:rustc-link-lib=static=tddb");
    println!("cargo:rustc-link-lib=static=tdsqlite");
    println!("cargo:rustc-link-lib=static=tdnet");
    println!("cargo:rustc-link-lib=static=tdutils");

    // C++ linkers
    // println!("cargo:rustc-link-lib=stdc++");
    println!("cargo:rustc-link-lib=c++");
    // Make outpath
    let out_path = PathBuf::from(out_dir);

    // Link libz
    pkg_config::Config::new()
        .atleast_version("1.3.1")
        .probe("zlib")
        .unwrap();

    // Link openssh
    pkg_config::Config::new()
        .atleast_version("3.4.0")
        .probe("openssl")
        .unwrap();

    let bindings_file = out_path.join("bindings.rs");
    if !bindings_file.as_path().exists() {
        // The bindgen::Builder is the main entry point
        // to bindgen, and lets you build up options for
        // the resulting bindings.
        let bindings = bindgen::Builder::default()
            .clang_arg(format!("-I{include_dir}"))
            // The input header we would like to generate
            // bindings for.
            .header("headers.h")
            // Tell cargo to invalidate the built crate whenever any of the
            // included header files changed.
            .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
            // Finish the builder and generate the bindings.
            .generate()
            // Unwrap the Result and panic on failure.
            .expect("Unable to generate bindings");

        // Write the bindings to the $OUT_DIR/bindings.rs file.
        bindings
            .write_to_file(bindings_file.as_path())
            .expect("Couldn't write bindings!");
    }

    // Open generated td_lib
    let td_api = std::fs::read_to_string(
        out_path
            .join("td")
            .join("generate")
            .join("scheme")
            .join("td_api.tl"),
    )
    .unwrap();

    let td_api_path = out_path.join("td_api.rs");

    println!("cargo:rerun-if-changed={}", td_api_path.to_str().unwrap());
    if !td_api_path.as_path().exists() {
        println!("cargo:warning=\"compiling td_api\"");
        td_api_rs::compile(&td_api, td_api_path).unwrap();
    }
}
