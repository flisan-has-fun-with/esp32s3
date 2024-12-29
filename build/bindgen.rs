use std::path::PathBuf;

fn include_component(component: &str) -> String {
    let manifest_path: &'static str = env!("CARGO_MANIFEST_DIR");
    format!("{manifest_path}/.embuild/espressif/esp-idf/v5.2.2/components/{component}/include")
}

pub fn bindgen_vendor() {
    let esp_common: String = include_component("esp_common");
    let manifest_path: &'static str = env!("CARGO_MANIFEST_DIR");
    let bindings = bindgen::Builder::default()
        .header("./vendor/LilyGo-EPD47/src/epd_driver.h")
        .clang_arg(format!("-I{esp_common}"))
        .clang_arg(format!("-I{manifest_path}"))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Failed to generate bindings");

    bindings
        .write_to_file(manifest_path.parse::<PathBuf>().unwrap().join("src/board/screen/bindings.rs"))
        .expect("Failed to write bindings");

    println!("cargo::rerun-if-changed={manifest_path}/vendor");
}

