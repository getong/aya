// Socket Filter program for testing with an arbitrary program.
// This is mainly used in tests with consideration for old kernels.

#![no_std]
#![no_main]

use aya_ebpf::{macros::socket_filter, programs::SkBuffContext};
#[cfg(not(test))]
extern crate ebpf_panic;

// Introduced in kernel v3.19.
#[socket_filter]
pub fn simple_prog(_ctx: SkBuffContext) -> i64 {
    0
}
