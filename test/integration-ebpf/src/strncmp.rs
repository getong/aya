#![no_std]
#![no_main]

use aya_ebpf::{
    cty::c_long,
    helpers::{bpf_probe_read_user_str_bytes, bpf_strncmp},
    macros::{map, uprobe},
    maps::Array,
    programs::ProbeContext,
};
use integration_common::strncmp::TestResult;
#[cfg(not(test))]
extern crate ebpf_panic;

#[map]
static RESULT: Array<TestResult> = Array::with_max_entries(1, 0);

#[uprobe]
pub fn test_bpf_strncmp(ctx: ProbeContext) -> Result<(), c_long> {
    let s1: *const u8 = ctx.arg(0).ok_or(-1)?;
    let mut b1 = [0u8; 3];
    let _: &[u8] = unsafe { bpf_probe_read_user_str_bytes(s1, &mut b1) }?;

    let ptr = RESULT.get_ptr_mut(0).ok_or(-1)?;
    let dst = unsafe { ptr.as_mut() };
    let TestResult(dst_res) = dst.ok_or(-1)?;
    *dst_res = bpf_strncmp(&b1, c"ff");

    Ok(())
}
