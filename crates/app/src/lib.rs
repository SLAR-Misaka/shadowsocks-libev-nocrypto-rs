//! CLI/runtime scaffolding. Networking deliberately remains unimplemented.

#![forbid(unsafe_code)]

use clap::Parser;
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Debug, Clone, Copy)]
pub enum Service {
    Server,
    Local,
    Tunnel,
    Redir,
    Manager,
}

impl Service {
    pub const fn name(self) -> &'static str {
        match self {
            Self::Server => "ss-server",
            Self::Local => "ss-local",
            Self::Tunnel => "ss-tunnel",
            Self::Redir => "ss-redir",
            Self::Manager => "ss-manager",
        }
    }
}

#[derive(Debug, Parser)]
#[command(version, about = "Rust rewrite scaffold; network forwarding is not implemented yet")]
pub struct Cli {
    /// Legacy JSON configuration file (parsing is not yet implemented).
    #[arg(short = 'c', long = "config", value_name = "PATH")]
    pub config: Option<PathBuf>,
    /// Cipher method. Only the intentionally insecure `none` is planned for MVP.
    #[arg(short = 'm', long = "method")]
    pub method: Option<String>,
    /// Remote Shadowsocks host (reserved for future compatibility).
    #[arg(short = 's', long = "server")]
    pub server: Option<String>,
    /// Remote Shadowsocks port.
    #[arg(short = 'p', long = "server-port")]
    pub server_port: Option<u16>,
    /// Local listen port.
    #[arg(short = 'l', long = "local-port")]
    pub local_port: Option<u16>,
    /// Request UDP relay (not yet implemented).
    #[arg(short = 'u', long = "udp")]
    pub udp: bool,
    /// Explicit acknowledgement that `none` carries plaintext unauthenticated traffic.
    #[arg(long = "acknowledge-insecure-none")]
    pub acknowledge_insecure_none: bool,
}

pub async fn entrypoint(service: Service) -> ExitCode {
    let cli = Cli::parse();
    if let Some(method) = cli.method.as_deref() {
        if method != "none" {
            eprintln!("{}: unsupported method {method:?} in Rust MVP", service.name());
            return ExitCode::from(2);
        }
        if !cli.acknowledge_insecure_none {
            eprintln!(
                "{}: method=none has no encryption/authentication; pass --acknowledge-insecure-none",
                service.name()
            );
            return ExitCode::from(2);
        }
    }
    eprintln!(
        "{}: scaffold only; no sockets opened or traffic forwarded yet",
        service.name()
    );
    ExitCode::FAILURE
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cli_accepts_legacy_flag_spellings() {
        let cli = Cli::try_parse_from([
            "ss-local", "-s", "localhost", "-p", "8388", "-l", "1080", "-m", "none",
            "--acknowledge-insecure-none",
        ])
        .expect("parses common flags");
        assert_eq!(cli.server.as_deref(), Some("localhost"));
        assert_eq!(cli.server_port, Some(8388));
        assert_eq!(cli.local_port, Some(1080));
        assert!(cli.acknowledge_insecure_none);
    }
}
