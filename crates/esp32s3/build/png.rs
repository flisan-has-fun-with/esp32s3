use core::str;
use std::{
    fs::File,
    path::{Path, PathBuf},
};

use glob::glob;

pub const ESP_WIDTH: u32 = 960;
pub const ESP_HEIGHT: u32 = 540;

pub fn build() {
    let manifest_path: &'static str = env!("CARGO_MANIFEST_DIR");
    let write_dir = format!("{manifest_path}/image/value")
        .parse::<PathBuf>()
        .unwrap();

    std::fs::create_dir_all(&write_dir).unwrap();

    let mut image_rs_content = format!(
        r"
#[allow(unused)]
pub type FullScreenImage = [u8; {size}];
pub type StaticFullScreenImage = &'static [u8; {size}];
",
        size = ESP_WIDTH * ESP_HEIGHT / 2
    )
    .trim_start()
    .to_string();

    for image in glob(&format!("{manifest_path}/image/*.png")).unwrap() {
        let image = image.unwrap();
        let image_name = image.file_stem().unwrap().to_str().unwrap();

        let buffer_value = process_png(&image);
        std::fs::write(write_dir.join(format!("{image_name}.bin")), &buffer_value).unwrap();

        image_rs_content.push_str(&format!(
            "pub const {}: StaticFullScreenImage = include_bytes!(\"../image/value/{}.bin\");\n",
            image_name.to_ascii_uppercase(),
            image_name,
        ));
    }

    println!("cargo::rerun-if-changed=./image");

    std::fs::write(
        PathBuf::try_from(format!("{manifest_path}/src/image.rs")).unwrap(),
        image_rs_content.as_bytes(),
    )
    .unwrap();
}

fn process_png(png: &Path) -> Vec<u8> {
    let decoder = png::Decoder::new(File::open(png).unwrap());
    let mut reader = decoder.read_info().unwrap();
    let mut image_bytes = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut image_bytes).unwrap();

    assert!(
        info.width == ESP_WIDTH && info.height == ESP_HEIGHT,
        "The image is not in shape {ESP_WIDTH}x{ESP_HEIGHT}: {}",
        png.to_string_lossy()
    );

    image_bytes
        .windows(6)
        .step_by(6)
        .map(|x| [[x[0], x[1], x[2]], [x[3], x[4], x[5]]])
        .map(to_grayscale)
        .map(|(l, r)| (r, l))
        .map(pixel_to_buffer_value)
        .collect()
}

fn to_grayscale([left, right]: [[u8; 3]; 2]) -> (u8, u8) {
    if (left[0] != left[1] || left[1] != left[2]) || (right[0] != right[1] || right[1] != right[2])
    {
        panic!("The image contains non-grayscale pixel");
    }

    (left[0], right[0])
}

fn pixel_to_buffer_value((left, right): (u8, u8)) -> u8 {
    let left = left / 16;
    let right = right / 16;

    left << 4 | right
}
