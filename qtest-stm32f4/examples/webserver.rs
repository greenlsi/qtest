use clap::Parser;
use qtest_stm32f4::Peripherals;
use qtest_web::{app, qemu, ws, WebSession};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(about = "QTest Web Server for STM32F4")]
struct Args {
    /// Logging level
    #[arg(long, env = "LOG_LEVEL", default_value = "debug")]
    log_level: String,
    /// API URL
    #[arg(long, env = "API_URL", default_value = "127.0.0.1:8080")]
    api_url: String,
    /// WebSocket URL
    #[arg(long, env = "WS_URL", default_value = "127.0.0.1:8081")]
    ws_url: String,
    /// QTest URL
    #[arg(long, env = "QTEST_URL", default_value = "localhost:3000")]
    qtest_url: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let log_level = args
        .log_level
        .parse::<tracing::Level>()
        .unwrap_or(tracing::Level::DEBUG);
    tracing_subscriber::fmt().with_max_level(log_level).init();

    let (api_url, ws_url, qtest_url) = (args.api_url, args.ws_url, args.qtest_url);
    tracing::debug!("QTest HTTP API URL: {api_url}");
    tracing::debug!("QTest WebSocket URL: {ws_url}");
    tracing::debug!("QTest TCP socket URL: {qtest_url}");

    // Create WebSession with STM32F4 Peripherals platform
    let (session, ws_rx) = WebSession::new(Peripherals::new());

    // Spawn WebSocket handler
    let ws_handle = tokio::spawn(ws::ws_handler(ws_url, ws_rx));

    // Spawn QEMU QTest handler
    let qemu_handle = tokio::spawn(qemu::qtest_handler(qtest_url, session.clone()));

    // Spawn HTTP API handler
    let app_handle = tokio::spawn(app::app_handler(api_url, session));

    let _ = tokio::join! {
        ws_handle,
        qemu_handle,
        app_handle,
    };

    Ok(())
}
