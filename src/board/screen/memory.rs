use std::{ffi::c_void, ops::{Deref, DerefMut}};

use anyhow::{ensure, Result};
use esp_idf_svc::sys::{heap_caps_free, heap_caps_malloc, MALLOC_CAP_DEFAULT, MALLOC_CAP_SPIRAM};

pub struct AllocatedMemory<T, const S: usize> {
    memory: *mut [T; S],
    pointer: Box<[T; S]>
}

impl<T, const S: usize> AllocatedMemory<T, S> {
    pub fn new() -> Result<Self> {
        let allocated = unsafe { heap_caps_malloc(S, MALLOC_CAP_SPIRAM).cast::<[T; S]>() };
        ensure!(!allocated.is_null(), "The memory could not be allocated ({S} bytes)");

        log::info!("Allocated {S} bytes");

        Ok(Self {
            memory: allocated,
            pointer: unsafe { Box::<[T; S]>::from_raw(allocated) }
        })
    }

    pub unsafe fn memory(&mut self) -> *mut T {
        self.memory.cast::<T>()
    }
}

impl<T, const S: usize> Deref for AllocatedMemory<T, S> {
    type Target = [T; S];

    fn deref(&self) -> &Self::Target {
        &self.pointer
    }
}

impl<T, const S: usize> DerefMut for AllocatedMemory<T, S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pointer
    }
}


impl<T, const S: usize> Drop for AllocatedMemory<T, S> {
    fn drop(&mut self) {
        unsafe {
            heap_caps_free(self.memory as *mut c_void);
        }
    }
}
