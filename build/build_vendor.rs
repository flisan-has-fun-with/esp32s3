use glob::glob;

pub fn compile_vendor_lib() {
    let manifest_path: &'static str = env!("CARGO_MANIFEST_DIR");
    let esp_idf_sys_sdkconfig = glob(&format!("{manifest_path}/target/xtensa-esp32s3-espidf/debug/build/esp-idf-sys-*/out/build/bootloader/config"))
        .unwrap()
        .next()
        .unwrap()
        .unwrap();
    let esp_idf_sys_sdkconfig = esp_idf_sys_sdkconfig
        .to_str()
        .unwrap();

    let sources = glob(&format!("{manifest_path}/vendor/LilyGo-EPD47/src/*.c")).unwrap()
        .map(|path| path.unwrap().to_str().unwrap().to_string());

    // let sources = [
    //     format!("{manifest_path}/vendor/LilyGo-EPD47/src/epd_driver.h"),
    //     format!("{manifest_path}/vendor/LilyGo-EPD47/src/epd_driver.c"),
    //     format!("{manifest_path}/vendor/LilyGo-EPD47/src/font.c"),
    //     format!("{manifest_path}/vendor/LilyGo-EPD47/src/ed047tc1.c"),
    // ];

    let mut cc = cc::Build::new();
    let cc = cc
        .files(sources)
        .includes(
            glob(&format!("{manifest_path}/.embuild/espressif/esp-idf/v5.2.2/components/**/include"))
                .unwrap()
                .map(|path| path.unwrap())
                .filter(|path| {
                    !path.components().any(|cmp| [
                        "linux",
                        "test_gdbstub_host",
                        "esp_system",
                        "esp32c2", "esp32c3", "esp32c6", "esp32h2", "esp32", "esp32p4", "esp32s2",
                    ].contains(&cmp.as_os_str().to_str().unwrap()))
                })
                .map(|path| path.to_str().unwrap().to_string())
        )
        .include(format!("{manifest_path}/includes"))
        .include(format!("{manifest_path}/.embuild/espressif/esp-idf/v5.2.2/components/driver/deprecated"))
        .include(format!("{manifest_path}/.embuild/espressif/esp-idf/v5.2.2/components/esp_rom/include/esp32s3"))
        .include(format!("{manifest_path}/.embuild/espressif/esp-idf/v5.2.2/components/newlib/platform_include"))
        .include(format!("{manifest_path}/.embuild/espressif/esp-idf/v5.2.2/components/freertos/config/include/freertos"))
        .include(format!("{manifest_path}/.embuild/espressif/esp-idf/v5.2.2/components/freertos/FreeRTOS-Kernel/portable/xtensa/include/freertos"))
        .include(esp_idf_sys_sdkconfig)
        .flag("-Wno-unused-parameter")
        .flag("-Wno-sign-compare")
        .flag("-mlongcalls")
        .compiler(format!("{manifest_path}/.embuild/espressif/tools/xtensa-esp-elf/esp-13.2.0_20230928/xtensa-esp-elf/bin/xtensa-esp32s3-elf-gcc"));

    println!("cargo:rustc-link-arg=-mlongcalls");
    // panic!("{:#?}", cc.get_compiler());

    cc.compile("edp47")
}
