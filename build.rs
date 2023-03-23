use cmake::Config;
use std::env;

fn main() {
    let dst = Config::new("openssl_if").build();
    let out_dir = env::var("OUT_DIR").unwrap();
    println!("{}", out_dir);

    println!("cargo:rustc-link-search=native=/usr/local/lib");
    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib").display()
    );

    println!("cargo:rustc-link-lib=dylib=openssl_lib");
    println!("cargo:rustc-link-lib=dylib=ssl");
    println!("cargo:rustc-link-lib=dylib=crypto");

    // link c++ runtime
    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=dylib=c++");
    } else if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-lib=dylib=stdc++");
    } else if cfg!(target_os = "windows") {
        unimplemented!();
    }
}
