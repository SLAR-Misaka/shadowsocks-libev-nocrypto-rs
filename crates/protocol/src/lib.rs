//! Shadowsocks-compatible wire primitives for the intentionally unencrypted mode.
//!
//! **Security:** `none` provides no confidentiality, integrity, or authentication.
//! Do not use this as a security boundary on untrusted networks.

#![forbid(unsafe_code)]

pub mod address;
pub mod wire;

pub use address::{AddressError, TargetAddr};
pub use wire::{decode_udp_payload, encode_udp_payload};

/// The legacy `none` method is a wire-level identity transform.
/// It performs **no** cryptographic operations or authentication.
#[must_use]
pub fn identity_payload(payload: &[u8]) -> &[u8] {
    payload
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn none_mode_is_identity() {
        let payload = b"this is plaintext, not ciphertext";
        assert_eq!(identity_payload(payload), payload);
    }
}
