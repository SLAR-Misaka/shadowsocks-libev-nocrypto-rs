# shadowsocks-libev-nocrypto-rs

Rust rewrite scaffold for [SLAR-Misaka/shadowsocks-libev-nocrypto](https://github.com/SLAR-Misaka/shadowsocks-libev-nocrypto).

> [!WARNING]
> The legacy `none` method sends **plaintext and unauthenticated traffic**.
> It provides no confidentiality, integrity, or peer authentication. Do not
> expose it as a publicly accessible proxy or rely on it for protection on an
> untrusted network. The Rust CLI requires an explicit acknowledgement when
> `-m none` is passed; no network forwarding is implemented yet.

## Status

This repository is **a buildable skeleton, not a working proxy**.

| Area | Status |
| --- | --- |
| Cargo workspace and GitHub Actions CI | Scaffolded |
| Address header codec (IPv4, IPv6, domain) | Implemented with unit tests |
| No-encryption identity transform and UDP payload framing | Implemented with unit tests |
| CLI entry points for `ss-server`, `ss-local`, `ss-tunnel`, `ss-redir`, `ss-manager` | Stubs; intentionally exit nonzero |
| TCP/UDP relay, configuration compatibility, ACL/DNS, SIP003 | Not implemented |
| AEAD/legacy encryption methods | Not implemented; scope to be decided |

## Layout

- `crates/protocol`: no-unsafe wire-format primitives, with no external dependencies.
- `crates/app`: shared CLI/runtime entry points, initially using Tokio and Clap.
- `docs/ARCHITECTURE.md`: packet boundaries, security and extension approach.
- `docs/ROADMAP.md`: implementation order and acceptance targets.

## Develop

Requires Rust **1.85+** (edition 2024); `rust-toolchain.toml` selects stable.

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo run -p nocrypto-app --bin ss-server -- --help
```

Running a stub without `--help` reports that forwarding is not yet implemented
and exits with an error rather than silently pretending to operate.

## Compatibility goals

1. Implement a real TCP `ss-server` and SOCKS5 `ss-local` for the `none` method.
2. Verify old C client ↔ Rust server and Rust client ↔ old C server interoperability on loopback.
3. Add UDP, JSON/CLI parity, DNS and ACL before optional tools such as `ss-redir`.
4. Measure memory, CPU and throughput on both x86_64 and low-power ARM devices.

The original C project is GPLv3-or-later; this rewrite follows that license.
See `LICENSE` and preserve appropriate upstream notices for any subsequently
ported code.
