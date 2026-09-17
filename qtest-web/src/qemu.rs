use crate::{platform::Platform, WebSession};
use qtest::socket::SocketListener;
use tokio::io::Result;

pub async fn qtest_handler<T: Platform>(url: String, session: WebSession<T>) -> Result<()> {
    tracing::debug!("Creating QTest socket listener at {url}...");
    let mut listener = SocketListener::new(&url).await?;
    tracing::info!("QTest socket listener created at {url}");

    loop {
        tracing::info!("No active QEMU session. Waiting for new session...");
        let mut qemu = listener.new_session().await?;
        tracing::info!("New QEMU session established");
        tracing::debug!("QEMU session: Enabling IRQ Intercept In...");
        let platform_qom = session.platform.qom_path();
        let mut irq_receiver = qemu.irq_intercept_in(platform_qom).await?;
        tracing::info!("QEMU session: IRQ Intercept In enabled");

        session.qemu.lock().await.replace(qemu);

        while let Some(irq) = irq_receiver.recv().await {
            tracing::info!("QEMU session: new IRQ received: {irq:?}");
            if let Some(peripheral) = session.platform.input_irq_to_peripheral(irq) {
                let peripheral_id = peripheral.name();
                tracing::info!("IRQ corresponds to peripheral '{peripheral_id}'");
                session.ws_tx.send(peripheral_id.to_string()).unwrap();
            }
        }

        tracing::warn!("QEMU IRQ channel closed");
        if let Some(session) = session.qemu.lock().await.take() {
            session.abort();
            tracing::warn!("QEMU session closed");
        }
    }
}
