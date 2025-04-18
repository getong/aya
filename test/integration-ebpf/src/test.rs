#![no_std]
#![no_main]

use aya_ebpf::{
    bindings::{bpf_ret_code, xdp_action},
    macros::{flow_dissector, kprobe, kretprobe, tracepoint, uprobe, uretprobe, xdp},
    programs::{
        FlowDissectorContext, ProbeContext, RetProbeContext, TracePointContext, XdpContext,
    },
};
#[cfg(not(test))]
extern crate ebpf_panic;

#[xdp]
pub fn pass(ctx: XdpContext) -> u32 {
    match unsafe { try_pass(ctx) } {
        Ok(ret) => ret,
        Err(_) => xdp_action::XDP_ABORTED,
    }
}

unsafe fn try_pass(_ctx: XdpContext) -> Result<u32, u32> {
    Ok(xdp_action::XDP_PASS)
}

#[kprobe]
pub fn test_kprobe(_ctx: ProbeContext) -> u32 {
    0
}

#[kretprobe]
pub fn test_kretprobe(_ctx: RetProbeContext) -> u32 {
    0
}

#[tracepoint]
pub fn test_tracepoint(_ctx: TracePointContext) -> u32 {
    0
}

#[uprobe]
pub fn test_uprobe(_ctx: ProbeContext) -> u32 {
    0
}

#[uretprobe]
pub fn test_uretprobe(_ctx: RetProbeContext) -> u32 {
    0
}

#[flow_dissector]
pub fn test_flow(_ctx: FlowDissectorContext) -> u32 {
    // TODO: write an actual flow dissector. See tools/testing/selftests/bpf/progs/bpf_flow.c in the
    // Linux kernel for inspiration.
    bpf_ret_code::BPF_FLOW_DISSECTOR_CONTINUE
}
