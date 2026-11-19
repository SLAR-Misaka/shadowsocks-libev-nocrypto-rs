//! Address header: `ATYP | DST.ADDR | DST.PORT` (port is big-endian).

use std::error::Error;
use std::fmt;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

const ATYP_IPV4: u8 = 0x01;
const ATYP_DOMAIN: u8 = 0x03;
const ATYP_IPV6: u8 = 0x04;

/// Wire-level target address. Domain bytes remain opaque to preserve protocol
/// compatibility; validation/IDNA conversion belongs in the DNS layer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TargetAddr {
    Ip(SocketAddr),
    Domain { host: Vec<u8>, port: u16 },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddressError {
    Truncated { needed: usize, actual: usize },
    UnsupportedType(u8),
    InvalidDomainLength(usize),
}

impl fmt::Display for AddressError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Truncated { needed, actual } => write!(
                f,
                "truncated address header: need {needed} bytes, received {actual}"
            ),
            Self::UnsupportedType(value) => write!(f, "unsupported address type: {value:#04x}"),
            Self::InvalidDomainLength(value) => {
                write!(f, "domain length must be 1..=255 bytes, got {value}")
            }
        }
    }
}

impl Error for AddressError {}

impl TargetAddr {
    /// Parse one address header, returning the number of consumed bytes.
    /// Trailing payload bytes are intentionally left for the caller.
    pub fn decode(input: &[u8]) -> Result<(Self, usize), AddressError> {
        if input.is_empty() {
            return Err(AddressError::Truncated {
                needed: 1,
                actual: 0,
            });
        }
        match input[0] {
            ATYP_IPV4 => {
                require(input, 7)?;
                let ip = Ipv4Addr::new(input[1], input[2], input[3], input[4]);
                let port = u16::from_be_bytes([input[5], input[6]]);
                Ok((Self::Ip(SocketAddr::new(IpAddr::V4(ip), port)), 7))
            }
            ATYP_IPV6 => {
                require(input, 19)?;
                let mut octets = [0u8; 16];
                octets.copy_from_slice(&input[1..17]);
                let ip = Ipv6Addr::from(octets);
                let port = u16::from_be_bytes([input[17], input[18]]);
                Ok((Self::Ip(SocketAddr::new(IpAddr::V6(ip), port)), 19))
            }
            ATYP_DOMAIN => {
                require(input, 2)?;
                let len = usize::from(input[1]);
                validate_domain_length(len)?;
                let consumed = 2 + len + 2;
                require(input, consumed)?;
                let port = u16::from_be_bytes([input[2 + len], input[3 + len]]);
                Ok((
                    Self::Domain {
                        host: input[2..2 + len].to_vec(),
                        port,
                    },
                    consumed,
                ))
            }
            other => Err(AddressError::UnsupportedType(other)),
        }
    }

    /// Serialize a header without appending any application payload.
    pub fn encode(&self) -> Result<Vec<u8>, AddressError> {
        match self {
            Self::Ip(SocketAddr::V4(addr)) => {
                let mut out = Vec::with_capacity(7);
                out.push(ATYP_IPV4);
                out.extend_from_slice(&addr.ip().octets());
                out.extend_from_slice(&addr.port().to_be_bytes());
                Ok(out)
            }
            Self::Ip(SocketAddr::V6(addr)) => {
                let mut out = Vec::with_capacity(19);
                out.push(ATYP_IPV6);
                out.extend_from_slice(&addr.ip().octets());
                out.extend_from_slice(&addr.port().to_be_bytes());
                Ok(out)
            }
            Self::Domain { host, port } => {
                validate_domain_length(host.len())?;
                let mut out = Vec::with_capacity(2 + host.len() + 2);
                out.extend_from_slice(&[ATYP_DOMAIN, host.len() as u8]);
                out.extend_from_slice(host);
                out.extend_from_slice(&port.to_be_bytes());
                Ok(out)
            }
        }
    }
}

fn require(input: &[u8], needed: usize) -> Result<(), AddressError> {
    if input.len() < needed {
        Err(AddressError::Truncated {
            needed,
            actual: input.len(),
        })
    } else {
        Ok(())
    }
}

fn validate_domain_length(len: usize) -> Result<(), AddressError> {
    if (1..=255).contains(&len) {
        Ok(())
    } else {
        Err(AddressError::InvalidDomainLength(len))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ipv4_round_trip_and_trailing_payload() {
        let input = [0x01, 127, 0, 0, 1, 0x1f, 0x90, 0xaa];
        let (target, consumed) = TargetAddr::decode(&input).expect("valid IPv4 address");
        assert_eq!(consumed, 7);
        assert_eq!(target.encode().expect("encodes"), input[..consumed]);
        assert_eq!(&input[consumed..], &[0xaa]);
    }

    #[test]
    fn ipv6_round_trip() {
        let target = TargetAddr::Ip(SocketAddr::new(IpAddr::V6(Ipv6Addr::LOCALHOST), 5353));
        let bytes = target.encode().expect("encodes");
        assert_eq!(bytes.len(), 19);
        assert_eq!(TargetAddr::decode(&bytes), Ok((target, 19)));
    }

    #[test]
    fn domain_round_trip() {
        let target = TargetAddr::Domain {
            host: b"example.org".to_vec(),
            port: 443,
        };
        let bytes = target.encode().expect("encodes");
        assert_eq!(bytes[0..2], [0x03, 11]);
        assert_eq!(TargetAddr::decode(&bytes), Ok((target, bytes.len())));
    }

    #[test]
    fn reject_truncation_and_invalid_domains() {
        assert_eq!(
            TargetAddr::decode(&[0x01, 127]),
            Err(AddressError::Truncated {
                needed: 7,
                actual: 2
            })
        );
        assert_eq!(
            TargetAddr::decode(&[0x03, 0]),
            Err(AddressError::InvalidDomainLength(0))
        );
        assert_eq!(
            TargetAddr::decode(&[0x09]),
            Err(AddressError::UnsupportedType(0x09))
        );
        let too_long = TargetAddr::Domain {
            host: vec![b'a'; 256],
            port: 80,
        };
        assert_eq!(
            too_long.encode(),
            Err(AddressError::InvalidDomainLength(256))
        );
    }
}
