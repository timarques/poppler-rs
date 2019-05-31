extern crate bindgen;
extern crate semver;

use std::env;
use std::path::PathBuf;

fn main() {
    panic!("{:?}", env::var("OUT_DIR"));
    println!("cargo:rustc-link-lib=poppler");

    let builder = bindgen::Builder::default()
        .clang_arg("-I").clang_arg("/usr/include/glib-2.0")
        .clang_arg("-I").clang_arg("/usr/lib/glib-2.0/include")
        .clang_arg("-I").clang_arg("/usr/include/cairo")
        .clang_arg("-I").clang_arg("poppler/glib")
        .clang_arg("-I").clang_arg("poppler/build/glib")
        .header("poppler/glib/poppler-document.h")
        .header("poppler/glib/poppler-page.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    builder
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
