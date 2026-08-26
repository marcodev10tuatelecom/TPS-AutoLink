#![forbid(unsafe_code)]

use tps_auto_protocol::{PROTOCOL_NAME, ProtocolVersion};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoreState {
    Ready,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BuildIdentity {
    pub package_version: &'static str,
    pub protocol_name: &'static str,
    pub protocol_version: ProtocolVersion,
}

#[must_use]
pub const fn build_identity() -> BuildIdentity {
    BuildIdentity {
        package_version: env!("CARGO_PKG_VERSION"),
        protocol_name: PROTOCOL_NAME,
        protocol_version: ProtocolVersion::V1_0,
    }
}

#[must_use]
pub const fn initial_state() -> CoreState {
    CoreState::Ready
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn core_starts_ready() {
        assert_eq!(initial_state(), CoreState::Ready);
    }

    #[test]
    fn build_identity_uses_protocol_v1() {
        let identity = build_identity();

        assert_eq!(identity.protocol_name, "TPS AutoLink");
        assert_eq!(identity.protocol_version, ProtocolVersion::V1_0);
    }
}
