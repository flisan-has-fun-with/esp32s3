use crate::{env::{crate_root, edp47_lib, esp_idf_component, esp_idf_sdkconfig}, traitext::PathExt};

pub fn bindgen_vendor() {
    let esp_common: String = esp_idf_component("esp_common").join("include").coerce_to_string();

    let bindings = bindgen::Builder::default()
        .header(edp47_lib().join("src/epd_driver.h").coerce_to_string())
        .clang_arg(format!("-I{esp_common}"))
        .clang_arg(format!("-I{}", esp_idf_sdkconfig().coerce_to_string()))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .raw_line("#![allow(unused)]")
        .raw_line("#![allow(non_upper_case_globals)]")
        .raw_line("#![allow(non_camel_case_types)]")
        .generate()
        .expect("Failed to generate bindings");

    bindings
        .write_to_file(crate_root().join("src/board/screen/bindings.rs"))
        .expect("Failed to write bindings");

    println!("cargo::rerun-if-changed={}", edp47_lib().coerce_to_string());
}
