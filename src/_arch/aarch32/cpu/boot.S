// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2018-2021 Andre Richter <andre.o.richter@gmail.com>

.section ".text._start"

.global _start

_start:
    mov     sp, #0x8000000
    mov     fp, #0
    bl      runtime_init
hang: b hang

.globl dev_barrier
dev_barrier:
	b dsb

.globl dsb
dsb:
	mcr p15, 0, r0, c7, c10, 4
	bx lr

.global __aeabi_unwind_cpp_pr0
__aeabi_unwind_cpp_pr0:
    b unmangled_panic_wrapper