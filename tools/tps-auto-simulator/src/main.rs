#![forbid(unsafe_code)]

use std::env;
use std::process::ExitCode;

use tps_auto_core::{CoreState, build_identity, initial_state};
use tps_auto_protocol::{EnvelopeHeader, MessageKind};

fn print_status() {
    let identity = build_identity();
    println!("TPS_AUTOLINK_SIMULATOR=READY");
    println!("PROTOCOL={}", identity.protocol_name);
    println!("PROTOCOL_VERSION={}", identity.protocol_version);
}

fn self_test() -> Result<(), String> {
    if initial_state() != CoreState::Ready {
        return Err("core did not enter READY state".to_owned());
    }

    let header = EnvelopeHeader::new(MessageKind::Hello, 1, 0)
        .map_err(|error| format!("failed to create HELLO envelope: {error}"))?;

    if header.request_id != 1 {
        return Err("unexpected request ID".to_owned());
    }

    print_status();
    println!("SELF_TEST=PASS");
    Ok(())
}

fn main() -> ExitCode {
    match env::args().nth(1).as_deref() {
        None => {
            print_status();
            ExitCode::SUCCESS
        }
        Some("--self-test") => match self_test() {
            Ok(()) => ExitCode::SUCCESS,
            Err(error) => {
                eprintln!("SELF_TEST=FAIL");
                eprintln!("ERROR={error}");
                ExitCode::from(1)
            }
        },
        Some("--version") => {
            let identity = build_identity();
            println!("tps-auto-simulator {}", identity.package_version);
            ExitCode::SUCCESS
        }
        Some(argument) => {
            eprintln!("unsupported argument: {argument}");
            eprintln!("usage: tps-auto-simulator [--self-test|--version]");
            ExitCode::from(2)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simulator_self_test_passes() {
        assert!(self_test().is_ok());
    }
}
