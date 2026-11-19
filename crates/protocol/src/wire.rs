//! UDP datagrams in `none` mode contain an address header followed by bytes.
//! No IV, salt, MAC, or AEAD tag is present.

use crate::address::{AddressError, TargetAddr};

/// Decode one *Shadowsocks-side* UDP datagram. This is intentionally not the
/// SOCKS5 UDP-associate framing, which has a separate RSV/FRAG prefix.
pub fn decode_udp_payload(input: &[u8]) -> Result<(TargetAddr, &[u8]), AddressError> {
    let (target, consumed) = TargetAddr::decode(input)?;
    Ok((target, &input[consumed..]))
}

/// Build one Shadowsocks-side UDP datagram for the unencrypted wire format.
pub fn encode_udp_payload(target: &TargetAddr, payload: &[u8]) -> Result<Vec<u8>, AddressError> {
    let mut output = target.encode()?;
    output.extend_from_slice(payload);
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};

    #[test]
    fn udp_round_trip_preserves_payload() {
        let target = TargetAddr::Ip(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 53));
        let datagram = encode_udp_payload(&target, b"\0\xffhello").expect("encodes");
        let (decoded_target, decoded_data) = decode_udp_payload(&datagram).expect("decodes");
        assert_eq!(decoded_target, target);
        assert_eq!(decoded_data, b"\0\xffhello");
    }
}
