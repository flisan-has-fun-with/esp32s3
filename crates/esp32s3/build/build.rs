mod bindgen;
mod build_vendor;
mod png;

fn main() {
    embuild::espidf::sysenv::output();

    bindgen::bindgen_vendor();
    build_vendor::compile_vendor_lib();
    png::build();
}
