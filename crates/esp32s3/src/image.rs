#[allow(unused)]
pub type FullScreenImage = [u8; 259200];
pub type StaticFullScreenImage = &'static [u8; 259200];
pub const BOARD: StaticFullScreenImage = include_bytes!("../image/value/board.bin");
pub const FERRIS: StaticFullScreenImage = include_bytes!("../image/value/ferris.bin");
pub const TEST_PATTERNS: StaticFullScreenImage = include_bytes!("../image/value/test_patterns.bin");
