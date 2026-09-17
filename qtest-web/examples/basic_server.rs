use clap::Parser;
use qtest_web::{app, qemu, ws, WebSession};

#[derive(Parser, Debug)]
#[command(author, version, about)]
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

    let (session, ws_rx) = WebSession::new(());

    let ws_handle = tokio::spawn(ws::ws_handler(ws_url, ws_rx));
    let qemu_handle = tokio::spawn(qemu::qtest_handler(qtest_url, session.clone()));
    let app_handle = tokio::spawn(app::app_handler(api_url, session));

    let _ = tokio::join! {
        ws_handle,
        qemu_handle,
        app_handle,
    };

    Ok(())
}
