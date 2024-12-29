#[allow(unused)]
pub type FullScreenImage = [u8; 259200];
pub type StaticFullScreenImage = &'static [u8; 259200];
pub const FERRIS: StaticFullScreenImage = include_bytes!("../image/value/ferris.bin");
