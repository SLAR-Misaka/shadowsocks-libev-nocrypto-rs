# Architecture notes

## Crate boundaries

- `nocrypto-protocol`: pure parsing/serialization of Shadowsocks address
  headers and unencrypted datagram payloads. No IO, DNS, logging or global state.
- `nocrypto-app`: application launchers and eventually async transport,
  lifecycle management, ACL, DNS, plugin processes and platform integration.
- Keep Windows/Linux-specific socket options out of the protocol crate.

## Wire-level behavior

`none` mode uses the same target-address framing as the legacy implementation:

```text
TCP stream:  ATYP | DST.ADDR | DST.PORT | PAYLOAD...
UDP packet:  ATYP | DST.ADDR | DST.PORT | PAYLOAD
IPv4:        0x01 | 4 address bytes  | 2-byte network-order port
Domain:      0x03 | 1-byte length   | domain bytes | 2-byte port
IPv6:        0x04 | 16 address bytes | 2-byte network-order port
```

There is no IV, salt, MAC or AEAD tag in `none` mode. SOCKS5 UDP ASSOCIATE
framing (`RSV | FRAG | ATYP | ...`) is a **separate client-side boundary** and
must not be mistaken for the Shadowsocks-side datagram format.

`TargetAddr::decode` returns a consumed-byte count so a TCP parser can accept
partial socket reads and retain any trailing early application data.

## Safety and interoperability

- Never describe `none` as secure. It allows observation and modification of
  destinations and payloads by anyone on the transport path.
- Design an explicit, opt-in insecure-mode gate before enabling real sockets.
- Restrict live interoperability tests to loopback and isolated networks.
- Validate lengths, reserved fields, port parsing and memory limits before IO.
- Do not blindly copy the C implementation's permissive `strncmp("none", 4)`
  selection: Rust should accept exactly `none`, subject to a documented parity
  decision.
- Runtime components should expose graceful cancellation and bounded buffers.

## Next interfaces (not yet implemented)

1. Async transport abstraction for inbound/outbound TCP streams and UDP peers.
2. SOCKS5 handshake/parser module at the local-proxy boundary.
3. Address resolution + ACL evaluation independent of transport.
4. Config loader accepting established JSON keys and CLI precedence.
5. Linux transparent sockets isolated behind platform-specific modules.
