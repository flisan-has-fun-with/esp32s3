use env::crate_root;

mod bindgen;
mod build_vendor;
mod env;
mod png;
pub mod traitext;

fn main() {
    embuild::espidf::sysenv::output();

    crate_root();

    bindgen::bindgen_vendor();
    build_vendor::compile_vendor_lib();
    png::build();
}
