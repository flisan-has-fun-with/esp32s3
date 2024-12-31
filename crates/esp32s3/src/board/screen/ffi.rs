use anyhow::Result;

use super::{
    bindings::{uint_fast8_t, EPD_HEIGHT, EPD_WIDTH},
    memory::AllocatedMemory,
};

pub const FRAMEBUFFER_SIZE: usize = (EPD_WIDTH / 2 * EPD_HEIGHT) as usize;

pub struct Framebuffer(pub(super) AllocatedMemory<uint_fast8_t, FRAMEBUFFER_SIZE>);

impl Framebuffer {
    pub fn new() -> Result<Self> {
        let mut buffer = AllocatedMemory::new()?;
        buffer.fill(0xff);

        Ok(Framebuffer(buffer))
    }

    pub fn copy(&mut self, slice: &[uint_fast8_t]) {
        self.0.copy_from_slice(slice);
    }

    pub fn fill(&mut self, value: uint_fast8_t) {
        self.0.fill(value);
    }

    pub unsafe fn to_point(&mut self) -> *mut u8 {
        self.0.memory()
    }
}
