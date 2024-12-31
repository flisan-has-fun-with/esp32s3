use std::path::PathBuf;

use glob::glob;

use crate::traitext::PathExt;

pub const IS_DEBUG: bool = cfg!(debug_assertions);

pub fn crate_root() -> PathBuf {
    env!("CARGO_MANIFEST_DIR").parse::<PathBuf>().unwrap()
}

pub fn workspace_root() -> PathBuf {
    crate_root().join("../../")
}

pub fn espressif_utils() -> PathBuf {
    workspace_root().join(".embuild/espressif")
}

pub fn esp_idf() -> PathBuf {
    espressif_utils().join("esp-idf/v5.2.2")
}

pub fn esp_idf_component(component: &str) -> PathBuf {
    esp_idf().join("components").join(component)
}

pub fn esp_idf_sdkconfig() -> PathBuf {
    glob(&xtensa_target().join("esp-idf-sys-*/out/build/bootloader/config").coerce_to_string())
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
}

pub fn xtensa_target() -> PathBuf {
    let dir = if IS_DEBUG { "debug" } else { "release" };

    workspace_root()
        .join("target/xtensa-esp32s3-espidf")
        .join(dir)
        .join("build")
}

pub fn edp47_lib() -> PathBuf {
    crate_root()
        .join("vendor/LilyGo-EPD47")
}

