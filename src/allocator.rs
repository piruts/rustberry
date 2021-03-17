use alloc::alloc::{GlobalAlloc, Layout};
use linked_list_allocator::LockedHeap;

use core::ptr;

// symbol from linker script
extern "Rust" {
    static __bss_end_inclusive: usize;
}

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

pub const HEAP_SIZE: usize = 0x100000; // 16MB stack size

static mut rpiA_heap_start: *mut u8 = __bss_end_inclusive as *mut u8;
static mut rpiA_heap_end: *mut u8 =
    (__bss_end_inclusive as *mut u8).offset(HEAP_SIZE as isize) as *mut u8;

pub struct Header {
    pub payload_size: usize,
    pub status: u8, // 0 if free, 1 if in use
}

pub struct Allocator {
    heap_start: usize,
    heap_size: usize,
}

impl Allocator {
    pub const fn new() -> Self {
        Self {
            heap_start: 0,
            heap_size: 0,
        }
    }

    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        self.heap_size = heap_size;
        self.heap_start = heap_start;
    }

    // Call extend_heap as needed to extend size of heap segment
    // Use extend_heap implementation as given
    unsafe fn extend_heap(&mut self, mut nbytes: usize) -> *mut u8 {
        let mut sp: *mut u8 = 0 as *mut u8;
        asm!("mov {}, sp", out(reg) sp); // get sp register (current stack top)
        let mut stack_reserve = sp.offset(-(HEAP_SIZE as isize)); // allow for 16MB growth in stack
        let mut prev_end = heap_end;
        if prev_end.offset(nbytes as isize) > stack_reserve {
            0 as *mut u8
        } else {
            heap_end = prev_end.offset(nbytes as isize);
            prev_end
        }
    }

    unsafe fn allocate(&mut self, mut nbytes: usize) -> Result<*mut u8, ()> {
        let mut sp: *mut u8 = 0 as *mut u8;
        asm!("mov {}, sp", out(reg) sp); // get sp register (current stack top)
        let mut stack_reserve = sp.offset(-(HEAP_SIZE as isize)); // allow for 16MB growth in stack

        let mut stack_reserve = sp.offset(-(HEAP_SIZE as isize));
        nbytes = nbytes + 7 & !7; // round to nearest 8

        let mut heap_current = heap_start;
        let mut hdr = heap_current as *mut Header;

        while heap_current != heap_end {
            // update the Header struct
            hdr = heap_current as *mut Header;
            if (*hdr).status == 0 && nbytes == (*hdr).payload_size {
                (*hdr).status = 1;
                return Ok(heap_current.offset(8 as isize));
            } else {
                // if there is enough space for a new Header, split the slot into two
                if (*hdr).status == 0 && nbytes + 8 < (*hdr).payload_size {
                    let mut original_size = (*hdr).payload_size;
                    (*hdr).status = 1;
                    (*hdr).payload_size = nbytes;

                    // advance to the next (newly created) Header
                    hdr = heap_current.offset(8 as isize).offset(nbytes as isize) as *mut Header;
                    (*hdr).status = 0;
                    (*hdr).payload_size = original_size - 8 - nbytes;
                    return Ok(heap_current.offset(8 as isize));
                }
            }
            // advance to the next Header
            heap_current = heap_current.offset(8 + (*hdr).payload_size as isize);
        }

        let mut prev_end = heap_end;
        if prev_end.offset(8 as isize).offset(nbytes as isize) > stack_reserve {
            return Err(());
        } else {
            let mut hdr_0 = prev_end as *mut Header;
            (*hdr_0).payload_size = nbytes;
            (*hdr_0).status = 1;
            heap_end = prev_end.offset(8 as isize)
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
            while (*last_hdr).status == 0 && heap_current != self.heap_end as *mut u8 {
                (*hdr).payload_size += 8 + (*last_hdr).payload_size;
                heap_current = heap_current
                    .offset(8)
                    .offset((*last_hdr).payload_size as isize);
                last_hdr = heap_current as *mut Header
            }
        }
    }
}

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if let Ok(alloc_start) = self.allocate(layout.size()) {
            return alloc_start;
        } else {
            ptr::null_mut()
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.deallocate(ptr);
    }
}
