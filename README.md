# aya-log - a logging library for eBPF programs

## Overview

`aya-log` is a logging library for eBPF programs written using [aya]. Think of
it as the [log] crate for eBPF.

## Installation

### User space

Add `aya-log` to `Cargo.toml`:

```toml
[dependencies]
aya-log = { git = "https://github.com/aya-rs/aya-log", branch = "main" }
```

### eBPF side

Add `aya-log-ebpf` and `ufmt` to `Cargo.toml`:

```toml
[dependencies]
aya-log-ebpf = { git = "https://github.com/aya-rs/aya-log", branch = "main" }
ufmt = "0.1"
```

## Example

Here's an example that uses `aya-log` in conjunction with the [simplelog] crate
to log eBPF messages to the terminal.

### User space code

```rust
use aya_log::BpfLogger;
use simplelog::{ColorChoice, ConfigBuilder, LevelFilter, TermLogger, TerminalMode};

BpfLogger::init(
    &mut bpf,
    TermLogger::new(
        LevelFilter::Trace,
        ConfigBuilder::new()
            .set_target_level(LevelFilter::Error)
            .set_location_level(LevelFilter::Error)
            .build(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    ),
).unwrap();
```

### eBPF code

```rust
use aya_log_ebpf::info;

fn try_xdp_firewall(ctx: XdpContext) -> Result<u32, ()> {
    if let Some(port) = tcp_dest_port(&ctx)? {
        if block_port(port) {
            info!(&ctx, "❌ blocked incoming connection on port: {}", port);
            return Ok(XDP_DROP);
        }
    }

    Ok(XDP_PASS)
}
```

[aya]: https://github.com/aya-rs/aya
[log]: https://docs.rs/log
[simplelog]: https://docs.rs/simplelog