#[tokio::main]
async fn main() -> std::process::ExitCode {
    nocrypto_app::entrypoint(nocrypto_app::Service::Tunnel).await
}
