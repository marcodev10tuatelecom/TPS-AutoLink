#![forbid(unsafe_code)]

use core::fmt;

pub const PROTOCOL_NAME: &str = "TPS AutoLink";
pub const PROTOCOL_MAJOR: u16 = 1;
pub const PROTOCOL_MINOR: u16 = 0;
pub const MAX_PAYLOAD_LEN: u32 = 1_048_576;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProtocolVersion {
    pub major: u16,
    pub minor: u16,
}

impl ProtocolVersion {
    pub const V1_0: Self = Self {
        major: PROTOCOL_MAJOR,
        minor: PROTOCOL_MINOR,
    };

    #[must_use]
    pub const fn is_compatible_with(self, other: Self) -> bool {
        self.major == other.major
    }
}

impl fmt::Display for ProtocolVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}

#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MessageKind {
    Hello = 1,
    Capabilities = 2,
    Error = 255,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EnvelopeHeader {
    pub protocol_version: ProtocolVersion,
    pub message_kind: MessageKind,
    pub request_id: u64,
    pub payload_length: u32,
}

impl EnvelopeHeader {
    pub fn new(
        message_kind: MessageKind,
        request_id: u64,
        payload_length: u32,
    ) -> Result<Self, ProtocolError> {
        if payload_length > MAX_PAYLOAD_LEN {
            return Err(ProtocolError::PayloadTooLarge {
                requested: payload_length,
                maximum: MAX_PAYLOAD_LEN,
            });
        }

        Ok(Self {
            protocol_version: ProtocolVersion::V1_0,
            message_kind,
            request_id,
            payload_length,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtocolError {
    PayloadTooLarge { requested: u32, maximum: u32 },
}

impl fmt::Display for ProtocolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PayloadTooLarge { requested, maximum } => {
                write!(
                    f,
                    "payload length {requested} exceeds maximum allowed length {maximum}"
                )
            }
        }
    }
}

impl std::error::Error for ProtocolError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn v1_minor_versions_are_compatible() {
        let local = ProtocolVersion::V1_0;
        let remote = ProtocolVersion { major: 1, minor: 9 };

        assert!(local.is_compatible_with(remote));
    }

    #[test]
    fn different_major_versions_are_not_compatible() {
        let local = ProtocolVersion::V1_0;
        let remote = ProtocolVersion { major: 2, minor: 0 };

        assert!(!local.is_compatible_with(remote));
    }

    #[test]
    fn envelope_accepts_payload_at_limit() {
        let header = EnvelopeHeader::new(MessageKind::Hello, 7, MAX_PAYLOAD_LEN);

        assert!(header.is_ok());
    }

    #[test]
    fn envelope_rejects_payload_above_limit() {
        let error = EnvelopeHeader::new(MessageKind::Hello, 7, MAX_PAYLOAD_LEN + 1)
            .expect_err("payload above the limit must be rejected");

        assert_eq!(
            error,
            ProtocolError::PayloadTooLarge {
                requested: MAX_PAYLOAD_LEN + 1,
                maximum: MAX_PAYLOAD_LEN,
            }
        );
    }
}
