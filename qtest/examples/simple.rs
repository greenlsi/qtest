use qtest::socket::SocketListener;
use std::env;

#[tokio::main]
async fn main() {
    // Initialize tracing subscriber for logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG) // Show logs at DEBUG level or higher
        .init();

    // Get the address from command-line arguments or use the default
    let args: Vec<String> = env::args().collect();
    let address = args.get(1).map_or("localhost:3000", String::as_str);

    let mut listener = SocketListener::new(address).await.unwrap();
    tracing::info!("Socket listener ready. listening on {}", listener.address());

    loop {
        tracing::info!("Waiting for new session...");
        let mut session = listener.new_session().await.unwrap();
        tracing::info!("New session created");

        let mut irq_receiver = session.irq_intercept_in("/machine/soc").await.unwrap();
        tracing::debug!("IRQ Intercept In enabled");

        let handle = tokio::spawn(async move {
            while let Some(irq) = irq_receiver.recv().await {
                tracing::debug!("[IRQ] received IRQ: {irq:?}");
            }
            tracing::error!("IRQ receiver terminated");
        });

        let res = session
            .set_irq_in("/machine/soc/gpio[2]", "input-in", 13, 1)
            .await;
        tracing::debug!("Set IRQ In 1: {:?}", res);

        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        let res = session
            .set_irq_in("/machine/soc/gpio[2]", "input-in", 13, 0)
            .await;
        tracing::debug!("Set IRQ In 0: {:?}", res);

        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        let res = session
            .set_irq_in("/machine/soc/gpio[2]", "input-in", 13, 1)
            .await;
        tracing::debug!("Set IRQ In 1: {:?}", res);

        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        let res = session.read(0, 10000).await;
        tracing::debug!("Read: {:?}", res);

        tracing::info!("Terminating session...");
        handle.abort();
        session.abort();
        tracing::info!("Session terminated");
    }
}
