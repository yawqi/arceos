//! Talc memory allocator
//!
//! TODO: comments
//!

use super::{AllocError, AllocResult, BaseAllocator, ByteAllocator};
use talc::{ErrOnOom, Span, Talc};

pub struct TalcByteAllocator {
    inner: Talc<ErrOnOom>,
    total_bytes: usize,
    used_bytes: usize,
}

impl TalcByteAllocator {
    pub const fn new() -> TalcByteAllocator {
        TalcByteAllocator {
            inner: Talc::new(ErrOnOom),
            total_bytes: 0,
            used_bytes: 0,
        }
    }
}

impl BaseAllocator for TalcByteAllocator {
    fn init(&mut self, start: usize, size: usize) {
        let span = Span::from_base_size(start as *mut u8, size);
        let mapped_scan = unsafe { self.inner.claim(span).unwrap() };

        self.total_bytes += mapped_scan.size();
    }

    fn add_memory(&mut self, start: usize, size: usize) -> AllocResult {
        let span = Span::from_base_size(start as *mut u8, size);
        let mapped_scan = unsafe {
            self.inner
                .claim(span)
                .map_err(|_| AllocError::InvalidParam)?
        };
        self.total_bytes += mapped_scan.size();
        Ok(())
    }
}

impl ByteAllocator for TalcByteAllocator {
    fn alloc(&mut self, layout: core::alloc::Layout) -> AllocResult<core::ptr::NonNull<u8>> {
        let ptr = unsafe {
            self.inner
                .malloc(layout)
                .map_err(|_| AllocError::NotAllocated)?
        };
        self.used_bytes += layout.size();
        Ok(ptr)
    }

    fn dealloc(&mut self, pos: core::ptr::NonNull<u8>, layout: core::alloc::Layout) {
        unsafe {
            self.inner.free(pos, layout);
        }
        self.used_bytes -= layout.size();
    }

    fn total_bytes(&self) -> usize {
        self.total_bytes
    }

    fn available_bytes(&self) -> usize {
        self.total_bytes - self.used_bytes
    }

    fn used_bytes(&self) -> usize {
        self.used_bytes
    }
}
