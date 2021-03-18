// Author: Flynn Dreilinger <flynnd@stanford.edu>

use alloc::alloc::{GlobalAlloc, Layout};
use core::ptr;

// symbol from linker script
extern "Rust" {
    static __bss_end_inclusive: usize;
}

#[global_allocator]
static ALLOCATOR: Locked<Allocator> = Locked::new(Allocator::new());

pub const HEAP_SIZE: usize = 0x1000000; // 16MB stack size

/// A wrapper around spin::Mutex to permit trait implementations.
pub struct Locked<A> {
    inner: spin::Mutex<A>,
}

impl<A> Locked<A> {
    pub const fn new(inner: A) -> Self {
        Locked {
            inner: spin::Mutex::new(inner),
        }
    }

    pub fn lock(&self) -> spin::MutexGuard<A> {
        self.inner.lock()
    }
}

pub fn init_heap() {
    unsafe {
        (*ALLOCATOR.lock()).init(__bss_end_inclusive, HEAP_SIZE);
    }
}

pub struct Header {
    pub payload_size: usize,
    pub status: u8, // 0 if free, 1 if in use
}

pub struct Allocator {
    heap_start: usize,
    heap_end: usize,
    heap_size: usize,
}

impl Allocator {
    pub const fn new() -> Self {
        Self {
            heap_start: 0,
            heap_end: 0,
            heap_size: 0,
        }
    }

    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        self.heap_start = heap_start;
        self.heap_end = heap_start;
        self.heap_size = heap_size;
    }

    // Call extend_heap as needed to extend size of heap segment
    // Use extend_heap implementation as given
    unsafe fn extend_heap(&mut self, nbytes: usize) -> *mut u8 {
        let mut sp: *mut u8;
        asm!("mov {}, sp", out(reg) sp); // get sp register (current stack top)
        let stack_reserve = sp.offset(-(self.heap_size as isize)); // allow for 16MB growth in stack
        let prev_end = self.heap_end as *mut u8;
        if prev_end.offset(nbytes as isize) > stack_reserve {
            0 as *mut u8
        } else {
            self.heap_end = prev_end.offset(nbytes as isize) as usize;
            prev_end
        }
    }

    unsafe fn allocate(&mut self, mut nbytes: usize) -> Result<*mut u8, ()> {
        let mut sp: *mut u8;
        asm!("mov {}, sp", out(reg) sp); // get sp register (current stack top)
        let stack_reserve = sp.offset(-(self.heap_size as isize)); // allow for 16MB growth in stack
        nbytes = nbytes + 7 & !7; // round to nearest 8

        let mut heap_current = self.heap_start;
        let mut hdr: *mut Header;

        while heap_current != self.heap_end {
            // update the Header struct
            hdr = heap_current as *mut Header;
            if (*hdr).status == 0 && nbytes == (*hdr).payload_size {
                (*hdr).status = 1;
                return Ok((heap_current as *mut u8).offset(1 as isize));
            } else {
                // if there is enough space for a new Header, split the slot into two
                if (*hdr).status == 0 && nbytes + 8 < (*hdr).payload_size {
                    let original_size = (*hdr).payload_size;
                    (*hdr).status = 1;
                    (*hdr).payload_size = nbytes;

                    // advance to the next (newly created) Header
                    hdr = (heap_current as *mut u8)
                        .offset(8 as isize)
                        .offset(nbytes as isize) as *mut Header;
                    (*hdr).status = 0;
                    (*hdr).payload_size = original_size - 8 - nbytes;
                    return Ok((heap_current as *mut u8).offset(8 as isize));
                }
            }
            // advance to the next Header
            heap_current =
                (heap_current as *mut u8).offset(8 + (*hdr).payload_size as isize) as usize;
        }

        let prev_end = self.heap_end;
        if (prev_end as *mut u8)
            .offset(8 as isize)
            .offset(nbytes as isize)
            > stack_reserve
        {
            return Err(());
        } else {
            let mut hdr_0 = prev_end as *mut Header;
            (*hdr_0).payload_size = nbytes;
            (*hdr_0).status = 1;
            self.heap_end = (prev_end as *mut u8).offset(8 as isize) as usize;
        }
        return Ok(self.extend_heap(nbytes));
    }

    pub unsafe fn deallocate(&mut self, ptr: *mut u8) {
        if !ptr.is_null() {
            let mut hdr = (ptr as *mut u8).offset(-8) as *mut Header;
            (*hdr).status = 0;

            // advance
            let mut heap_current = hdr.offset(8 + (*hdr).payload_size as isize);
            let mut last_hdr = heap_current as *mut Header;
            while (*last_hdr).status == 0 && heap_current != self.heap_end as *mut Header {
                (*hdr).payload_size += 8 + (*last_hdr).payload_size;
                heap_current = heap_current
                    .offset(8)
                    .offset((*last_hdr).payload_size as isize);
                last_hdr = heap_current as *mut Header
            }
        }
    }
}

unsafe impl GlobalAlloc for Locked<Allocator> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut allocator = self.lock();
        if let Ok(alloc_start) = allocator.allocate(layout.size()) {
            return alloc_start;
        } else {
            ptr::null_mut()
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        let mut allocator = self.lock();
        allocator.deallocate(ptr);
    }
}
