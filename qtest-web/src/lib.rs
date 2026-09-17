pub mod app;
pub mod platform;
pub mod qemu;
pub mod result;
pub mod ws;

use qtest::session::Session;
use std::sync::Arc;
use tokio::sync::{
    watch::{channel, Receiver, Sender},
    Mutex,
};

use platform::Platform;

#[derive(Clone)]
pub struct WebSession<T> {
    pub ws_tx: Sender<String>,
    pub qemu: Arc<Mutex<Option<Session>>>,
    pub platform: T,
}

impl<T: Platform> WebSession<T> {
    pub fn new(platform: T) -> (Self, Receiver<String>) {
        let (ws_tx, ws_rx) = channel("".to_string());

        let session = Self {
            qemu: Arc::new(Mutex::new(None)),
            ws_tx,
            platform,
        };

        (session, ws_rx)
    }
}
