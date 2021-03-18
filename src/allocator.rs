// Author: Flynn Dreilinger <flynnd@stanford.edu>

use alloc::alloc::{GlobalAlloc, Layout};
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::cell::UnsafeCell;
use core::ptr;

// symbol from linker script
extern "Rust" {
    static __bss_end_inclusive: usize;
}

pub const HEAP_SIZE: usize = 0x1000000; // 16MB stack size

#[global_allocator]
static ALLOCATOR: Allocator = Allocator::new();

pub fn init_heap() {
    unsafe {
        ALLOCATOR.init(__bss_end_inclusive);
    }
}

pub struct Header {
    pub payload_size: usize,
    pub status: u8, // 0 if free, 1 if in use
}

pub struct Allocator {
    heap_start: UnsafeCell<usize>,
    heap_end: UnsafeCell<usize>,
    heap_size: usize,
}

unsafe impl Sync for Allocator {}

impl Allocator {
    pub const fn new() -> Self {
        Self {
            heap_start: UnsafeCell::new(0 as usize),
            heap_end: UnsafeCell::new(0 as usize),
            heap_size: HEAP_SIZE,
        }
    }

    pub unsafe fn init(&self, heap_start: usize) {
        *(self.heap_start.get()) = heap_start;
        *(self.heap_end.get()) = heap_start;
    }

    // Call extend_heap as needed to extend size of heap segment
    // Use extend_heap implementation as given
    unsafe fn extend_heap(&self, nbytes: usize) -> *mut u8 {
        let mut sp: *mut u8;
        asm!("mov {}, sp", out(reg) sp); // get sp register (current stack top)
        let stack_reserve = sp.offset(-(self.heap_size as isize)); // allow for 16MB growth in stack
        let prev_end = *(self.heap_end.get()) as *mut u8;
        if prev_end.offset(nbytes as isize) > stack_reserve {
            0 as *mut u8
        } else {
            *(self.heap_end.get()) = prev_end.offset(nbytes as isize) as usize;
            prev_end
        }
    }

    unsafe fn allocate(&self, mut nbytes: usize) -> Result<*mut u8, ()> {
        let mut sp: *mut u8;
        asm!("mov {}, sp", out(reg) sp); // get sp register (current stack top)
        let stack_reserve = sp.offset(-(self.heap_size as isize)); // allow for 16MB growth in stack
        nbytes = nbytes + 7 & !7; // round to nearest 8

        let mut heap_current = *(self.heap_start.get());
        let mut hdr: *mut Header;

        while heap_current != *(self.heap_end.get()) {
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

        let prev_end = *(self.heap_end.get());
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
            *(self.heap_end.get()) = (prev_end as *mut u8).offset(8 as isize) as usize;
        }
        return Ok(self.extend_heap(nbytes));
    }

    pub unsafe fn deallocate(&self, ptr: *mut u8) {
        if !ptr.is_null() {
            let mut hdr = (ptr as *mut u8).offset(-8) as *mut Header;
            (*hdr).status = 0;

            // advance
            let mut heap_current = hdr.offset(8 + (*hdr).payload_size as isize);
            let mut last_hdr = heap_current as *mut Header;
            while (*last_hdr).status == 0 && heap_current != *(self.heap_end.get()) as *mut Header {
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

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        self.deallocate(ptr);
    }
}


#[test_case]
fn test_vector() {
    let mut xs = Vec::new();
    xs.push(42);
    assert_eq!(xs.pop(), Some(42));
}

#[test_case]
fn test_box() {
    let boxed: Box<u8> = Box::new(5);
    let val: u8 = *boxed;
    assert_eq!(val, 5);
}

//--------------------------------------------------------------------------------------------------
//
//--------------------------------------------------------------------------------------------------

/*
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
*/

//--------------------------------------------------------------------------------------------------
//
//--------------------------------------------------------------------------------------------------

/*

use alloc::alloc::{GlobalAlloc, Layout};
use core::cell::UnsafeCell;
use core::ptr;

extern "Rust" {
    static __bss_end_inclusive: usize;
}

pub const HEAP_SIZE: usize = 0x1000000; // 16MB stack size

// Bump pointer allocator for *single* core systems
struct BumpPointerAlloc {
    head: UnsafeCell<usize>,
    end: usize,
}

unsafe impl Sync for BumpPointerAlloc {}

unsafe impl GlobalAlloc for BumpPointerAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // `interrupt::free` is a critical section that makes our allocator safe
        // to use from within interrupts
        let head = self.head.get();
        let size = layout.size();
        let align = layout.align();
        let align_mask = !(align - 1);

        // move start up to the next alignment boundary
        let start = (*head + align - 1) & align_mask;

        if start + size > self.end {
            // a null pointer signal an Out Of Memory condition
            ptr::null_mut()
        } else {
            *head = start + size;
            start as *mut u8
        }
    }

    unsafe fn dealloc(&self, _: *mut u8, _: Layout) {
        // this allocator never deallocates memory
    }
}

// Declaration of the global memory allocator
// NOTE the user must ensure that the memory region `[0x2000_0100, 0x2000_0200]`
// is not used by other parts of the program
#[global_allocator]
static HEAP: BumpPointerAlloc = BumpPointerAlloc {
    head: UnsafeCell::new(0x1000000),
    end: 0x1000000 + HEAP_SIZE,
};
*/
