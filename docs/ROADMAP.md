# Rewrite roadmap

## Phase 0 — foundations

- [x] Cargo workspace, Rust CLI binaries and CI pipeline.
- [x] Strict address codec and `none` datagram unit tests.
- [ ] Baseline fixtures: C `none` behavior for IPv4/IPv6/domain and UDP.
- [ ] Record target feature matrix, licensing and security decisions.

## Phase 1 — minimal interoperability

- [ ] Tokio TCP `ss-server`: parse target, resolve, connect, relay both ways.
- [ ] Tokio `ss-local`: SOCKS5 no-auth CONNECT and upstream relay.
- [ ] C ↔ Rust compatibility tests and deterministic loopback fixtures.
- [ ] Handle half-close, backpressure, timeouts, disconnects and cancellation.

## Phase 2 — functional parity

- [ ] Server UDP relay and SOCKS5 UDP ASSOCIATE.
- [ ] JSON/CLI configuration compatibility and validation.
- [ ] IPv4/IPv6 DNS resolution, ACL and per-session resource limits.
- [ ] `ss-tunnel` and Linux `ss-redir` (original destination / TPROXY).
- [ ] `ss-manager` and SIP003 external plugin compatibility.

## Phase 3 — release quality

- [ ] Fuzz parsers and harden exposed socket behavior.
- [ ] Compare CPU, RSS and throughput against C on x86_64/ARM.
- [ ] Cross-builds, packaging and reproducible release artifacts.
- [ ] Decide whether AEAD and obsolete stream ciphers are within scope.

All live traffic features remain **unimplemented** until their checklist items
and interoperability tests are complete.
