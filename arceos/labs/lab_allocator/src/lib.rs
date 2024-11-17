//! Allocator algorithm in lab.

#![no_std]
#![allow(unused_variables)]

use allocator::AllocError::NoMemory;
use allocator::{AllocError, AllocResult, BaseAllocator, BuddyByteAllocator, ByteAllocator};
use core::alloc::{GlobalAlloc, Layout};
use core::cmp::max;
use core::ptr::NonNull;
use log::info;

/// [ Buddy | keep | free ]
/// |       |      |      |
/// 256KB   keep          MEMORY_END

const BUDDY_SIZE: usize = 1 << 18;
const MEMORY_END: usize = 0xffffffc088000000;

pub struct LabByteAllocator {
    buddy:BuddyByteAllocator,
    start: usize,
    count: usize,
    free:usize,
    keep:usize
}

impl LabByteAllocator {
    pub const fn new() -> Self {
        Self {
            buddy: BuddyByteAllocator::new(),
            start: 0,
            count: 0,
            free: 0,
            keep: 0
        }
    }
}
fn align_up(pos: usize, align: usize) -> usize {
    (pos + align - 1) & !(align - 1)
}
impl BaseAllocator for LabByteAllocator {
    fn init(&mut self, start: usize, size: usize) {
        self.buddy.init(start,BUDDY_SIZE);
        self.start = start+BUDDY_SIZE;
        self.keep=self.start;
        info!("{:X}",MEMORY_END-start);
    }
    fn add_memory(&mut self, start: usize, size: usize) -> AllocResult {
        unreachable!()
    }
}

impl ByteAllocator for LabByteAllocator {
    fn alloc(&mut self, layout: Layout) -> AllocResult<NonNull<u8>> {
        match layout.align() {
            8=>{
                self.buddy.alloc(layout)
            },
            1=>{
                self.count += 1;

                let size = layout.size();
                match (self.count-1) %2{
                    0=>{
                        self.free = MEMORY_END- size;
                        if self.free < self.keep {
                            return Err(NoMemory);
                        }
                        NonNull::new(self.free as *mut u8).ok_or(NoMemory)
                    },
                    _=>{
                        self.keep+= size;
                        if self.keep >  MEMORY_END{
                            return Err(NoMemory);
                        }
                        NonNull::new((self.keep - size) as *mut u8).ok_or(NoMemory)
                    }
                }
            }
            _ => {unreachable!()}
        }
    }
    fn dealloc(&mut self, pos: NonNull<u8>, layout: Layout) {
        match layout.align() {
            8=>{
                self.buddy.dealloc(pos, layout);
            },
            _=>{
                self.count-=1;
                if self.count==0{
                    self.keep = self.start;
                }
            }
        }
    }
    fn total_bytes(&self) -> usize {
        MEMORY_END - self.start
    }
    fn used_bytes(&self) -> usize {
        (self.keep - self.start) +  (MEMORY_END - self.free)
    }
    fn available_bytes(&self) -> usize {
        self.free-self.keep
    }
}
