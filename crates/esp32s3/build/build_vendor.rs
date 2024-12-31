use glob::glob;

use crate::{env::{edp47_lib, esp_idf_component, esp_idf_sdkconfig, espressif_utils, workspace_root}, traitext::PathExt};

pub fn compile_vendor_lib() {
    let sources = glob(&edp47_lib().join("src/*.c").coerce_to_string())
        .unwrap()
        .map(|path| path.unwrap().to_str().unwrap().to_string());

    cc::Build::new()
        .files(sources)
        .includes(
            glob(&format!("{}/**/include", esp_idf_component("").coerce_to_string()))
                .unwrap()
                .map(|path| path.unwrap())
                .filter(|path| {
                    !path.components().any(|cmp| [
                        "linux",
                        "test_gdbstub_host",
                        "esp32c2", "esp32c3", "esp32c6", "esp32h2", "esp32", "esp32p4", "esp32s2",
                    ].contains(&cmp.as_os_str().to_str().unwrap()))
                })
                .map(|path| path.to_str().unwrap().to_string())
        )
        .include(esp_idf_sdkconfig().coerce_to_string())
        .include(esp_idf_component("driver").join("deprecated").coerce_to_string())
        .include(esp_idf_component("esp_rom").join("include/esp32s3").coerce_to_string())
        .include(esp_idf_component("freertos").join("config/include/freertos").coerce_to_string())
        .include(esp_idf_component("freertos").join("FreeRTOS-Kernel/portable/xtensa/include/freertos").coerce_to_string())
        .include(esp_idf_component("newlib").join("platform_include").coerce_to_string())
        .flag("-Wno-unused-parameter")
        .flag("-Wno-sign-compare")
        .flag("-Wno-attributes")  // section attributes are causing errors.
        .flag("-Wno-cpp")  // Some header files is deprecated
        .flag("-Wno-implicit-function-declaration")  // xTaskCreatePinnedToCore is not resolved
                                                     // at compile time
        .flag("-mlongcalls")
        .compiler(
            espressif_utils()
                .join("tools/xtensa-esp-elf/esp-13.2.0_20230928/xtensa-esp-elf/bin/xtensa-esp32s3-elf-gcc")
                .coerce_to_string()
        )
        .compile("edp47");

    println!("cargo:rustc-link-arg=-mlongcalls");
    println!("cargo:check-if-changed={}", workspace_root().join("sdkconfig.defaults").coerce_to_string());
}
