use crate::{
    platform::Platform,
    result::{Error, Response, Result},
    WebSession,
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use qtest::utils::set_peripheral_irq;
use tokio::{io, net::TcpListener};

#[derive(serde::Deserialize)]
struct ReadQuery {
    addr: String,
    size: Option<usize>,
}

#[derive(serde::Deserialize)]
struct ClockStepQuery {
    ns: Option<usize>,
}

#[derive(serde::Deserialize)]
struct IrqQuery {
    line: usize,
    level: isize,
}

/// Handler for the `/status` route. Returns whether QEMU is connected.
async fn status<T>(State(web_session): State<WebSession<T>>) -> Json<bool> {
    let qemu_connected = web_session.qemu.lock().await.is_some();
    Json(qemu_connected)
}

async fn clock_step<T>(
    Query(ClockStepQuery { ns }): Query<ClockStepQuery>,
    State(session): State<WebSession<T>>,
) -> Result<Response> {
    match session.qemu.lock().await.as_mut() {
        Some(session) => match session.clock_step(ns).await {
            Ok(resp) => Ok(resp.into()),
            Err(e) => Err(Error::Io(e)),
        },
        None => Err(Error::QemuClosed),
    }
}

async fn clock_set<T>(
    Query(ClockStepQuery { ns }): Query<ClockStepQuery>,
    State(session): State<WebSession<T>>,
) -> Result<Json<usize>> {
    let ns = ns.ok_or(Error::Other("ns query parameter is required".to_string()))?;
    match session.qemu.lock().await.as_mut() {
        Some(session) => match session.clock_set(ns).await {
            Ok(val) => Ok(Json(val)),
            Err(e) => Err(Error::Io(e)),
        },
        None => Err(Error::QemuClosed),
    }
}

macro_rules! read_routes {
    ($($name:ident, $type:ty);*) => {
        $(
            async fn $name<T>(
                Query(query): Query<ReadQuery>,
                State(web_session): State<WebSession<T>>,
            ) -> Result<Json<$type>> {
                let address = query.addr;
                let s = address.strip_prefix("0x").unwrap_or(&address);
                let address = usize::from_str_radix(s, 16)
                    .map_err(|e| Error::Other(format!("failed to parse address '{address}': {e}")))?;
                let mut qemu_guard = web_session.qemu.lock().await;
                let qemu_session = qemu_guard.as_mut().ok_or(Error::QemuClosed)?;
                match qemu_session.$name(address).await {
                    Ok(val) => Ok(Json(val)),
                    Err(e) => Err(Error::Other(format!(
                        "failed to read byte from address '{address:#x}': {e}"
                    ))),
                }
            }
        )*
    };
}

read_routes! {
    readb, u8;
    readw, u16;
    readl, u32;
    readq, u64
}

macro_rules! read_routes_str {
    ($($name:ident),*) => {
        $(
            async fn $name<T>(
                Query(ReadQuery { addr, size }): Query<ReadQuery>,
                State(web_session): State<WebSession<T>>,
            ) -> Result<Json<String>> {
                let size = size.ok_or(Error::Other("size query parameter is required".to_string()))?;
                let s = addr.strip_prefix("0x").unwrap_or(&addr);
                let addr = usize::from_str_radix(s, 16)
                    .map_err(|e| Error::Other(format!("failed to parse address '{addr}': {e}")))?;
                let mut qemu_guard = web_session.qemu.lock().await;
                let qemu_session = qemu_guard.as_mut().ok_or(Error::QemuClosed)?;
                match qemu_session.$name(addr, size).await {
                    Ok(val) => Ok(Json(val)),
                    Err(e) => Err(Error::Other(format!(
                        "failed to read byte from address '{addr:#x}': {e}"
                    ))),
                }
            }
        )*
    };
}

read_routes_str! { read, b64read }

async fn peripheral_irq<T: Platform>(
    Path(peripheral_id): Path<String>,
    Query(IrqQuery { line, level }): Query<IrqQuery>,
    State(web_session): State<WebSession<T>>,
) -> Result<StatusCode> {
    match web_session.qemu.lock().await.as_mut() {
        Some(qtest) => {
            set_peripheral_irq(qtest, &web_session.platform, &peripheral_id, line, level)
                .await
                .map_err(Error::Io)?;
            Ok(StatusCode::OK)
        }
        None => Err(Error::QemuClosed),
    }
}

fn create_router<T: Platform>() -> Router<WebSession<T>> {
    // Create route with standard routes
    let router = Router::new()
        .route("/status", get(status::<T>))
        .route("/clock_step", post(clock_step::<T>))
        .route("/clock_set", post(clock_set::<T>))
        .route("/readb", get(readb::<T>))
        .route("/readw", get(readw::<T>))
        .route("/readl", get(readl::<T>))
        .route("/readq", get(readq::<T>))
        .route("/read", get(read::<T>))
        .route("/b64read", get(b64read::<T>))
        .route("/peripheral_irq/{peripheral_id}", post(peripheral_irq::<T>));
    // Add platform specific routes
    T::add_routes(router)
}

pub async fn app_handler<T: Platform>(app_url: String, session: WebSession<T>) -> io::Result<()> {
    tracing::debug!("Initializing Axum router...");
    let app = create_router().with_state(session);
    tracing::debug!("Axum router initialized");

    tracing::debug!("Initializing TCP listener for HTTP server...");
    let listener = TcpListener::bind(&app_url).await?;
    tracing::info!("HTTP server listening on {app_url}");

    axum::serve(listener, app).await
}
