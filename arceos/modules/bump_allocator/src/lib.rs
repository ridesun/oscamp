#![no_std]
extern crate alloc;

use core::alloc::Layout;
use core::cmp::max;
use core::ptr::NonNull;
use allocator::{AllocError, AllocResult, BaseAllocator, ByteAllocator, PageAllocator};

/// Early memory allocator
/// Use it before formal bytes-allocator and pages-allocator can work!
/// This is a double-end memory range:
/// - Alloc bytes forward
/// - Alloc pages backward
///
/// [ bytes-used | avail-area | pages-used ]
/// |            | -->    <-- |            |
/// start       b_pos        p_pos       end
///
/// For bytes area, 'count' records number of allocations.
/// When it goes down to ZERO, free bytes-used area.
/// For pages area, it will never be freed!

pub struct EarlyAllocator<const PAGE_SIZE:usize>{
    start:  usize,
    end:    usize,
    count:  usize,
    byte_pos:  usize,
    page_pos:  usize,
}

impl<const PAGE_SIZE: usize>  EarlyAllocator<PAGE_SIZE> {
    pub const fn new()->Self{
        Self {
            start: 0,
            end: 0,
            count: 0,
            byte_pos: 0,
            page_pos: 0,
        }
    }
}

fn align_up(pos: usize, align: usize) -> usize {
    (pos + align - 1) & !(align - 1)
}
fn align_down(pos: usize, align: usize) -> usize {
    pos &!(align - 1)
}

impl<const PAGE_SIZE: usize>  BaseAllocator for EarlyAllocator<PAGE_SIZE> {
    fn init(&mut self, start: usize, size: usize) {
        self.start = start;
        self.end = start + size;
        self.byte_pos = start;
        self.page_pos = self.end;
    }

    fn add_memory(&mut self, _start: usize, _size: usize) -> AllocResult {
        todo!()
    }
}

impl<const PAGE_SIZE: usize>  ByteAllocator for EarlyAllocator<PAGE_SIZE> {
    fn alloc(&mut self, layout: Layout) -> AllocResult<NonNull<u8>> {
        let start = align_up(self.byte_pos, layout.align());
        let next = start + max(
            layout.size().next_power_of_two(),
            max(layout.align(), core::mem::size_of::<usize>()),
        );
        if next > self.page_pos {
            Err(AllocError::NoMemory)
        } else {
            self.byte_pos = next;
            self.count += 1;
            NonNull::new(start as *mut u8).ok_or(AllocError::NoMemory)
        }
    }

    fn dealloc(&mut self, _pos: NonNull<u8>, layout: Layout) {
        self.count -= 1;
        if self.count == 0 {
            self.byte_pos = self.start;
        }else {
            self.byte_pos -= max(
                layout.size().next_power_of_two(),
                max(layout.align(), core::mem::size_of::<usize>()),
            );
        }
    }

    fn total_bytes(&self) -> usize {
        self.end - self.start
    }

    fn used_bytes(&self) -> usize {
        self.byte_pos - self.start
    }

    fn available_bytes(&self) -> usize {
        self.page_pos - self.byte_pos
    }
}

impl<const PAGE_SIZE: usize>  PageAllocator for EarlyAllocator<PAGE_SIZE> {
    const PAGE_SIZE: usize = PAGE_SIZE;

    fn alloc_pages(&mut self, num_pages: usize, align_pow2: usize) -> AllocResult<usize> {
        if align_pow2 % PAGE_SIZE != 0 {
            return Err(AllocError::InvalidParam);
        }
        let align_pow2 = align_pow2 / PAGE_SIZE;
        if !align_pow2.is_power_of_two() {
            return Err(AllocError::InvalidParam);
        }
        let _align_log2 = align_pow2.trailing_zeros() as usize;
        let next=align_down(self.page_pos-num_pages*PAGE_SIZE, align_pow2);
        if next < self.byte_pos {
            Err(AllocError::NoMemory)
        } else {
            self.page_pos = next;
            Ok(next)
        }
    }

    fn dealloc_pages(&mut self, _pos: usize, _num_pages: usize) {
        todo!()
    }

    fn total_pages(&self) -> usize {
        (self.end - self.start) / PAGE_SIZE
    }

    fn used_pages(&self) -> usize {
        (self.end - self.page_pos) / PAGE_SIZE
    }

    fn available_pages(&self) -> usize {
        (self.page_pos - self.byte_pos) / PAGE_SIZE
    }
}
