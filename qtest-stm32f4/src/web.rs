use crate::{
    gpio::report::{GpioPinReport, GpioReport},
    timer::{
        report::{TimerChannelReport, TimerReport},
        Channel,
    },
    Peripherals,
};
use axum::{
    extract::{Json, Path, State},
    routing::get,
};
use axum_extra::extract::Query;
use qtest_web::{
    platform::Platform,
    result::{Error, Result},
    WebSession,
};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

impl Platform for Peripherals {
    fn add_routes(router: axum::Router<WebSession<Self>>) -> axum::Router<WebSession<Self>> {
        router
            .route("/stm32f4/gpio_info/{gpio_id}", get(gpio_info))
            .route("/stm32f4/timer_info/{timer_id}", get(timer_info))
    }
}

#[derive(Deserialize, Serialize)]
pub struct PeripheralPartQuery {
    #[serde(skip_serializing_if = "HashSet::is_empty", default)]
    parts: HashSet<usize>,
}

#[derive(Deserialize, Serialize)]
struct GpioInfo {
    registers: GpioReport,
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pins: HashMap<usize, GpioPinReport>,
}

#[derive(Deserialize, Serialize)]
struct TimerInfo {
    registers: TimerReport,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    channels: Vec<TimerChannelReport>,
}

async fn gpio_info(
    Path(gpio_id): Path<String>,
    Query(PeripheralPartQuery { parts }): Query<PeripheralPartQuery>,
    State(session): State<WebSession<Peripherals>>,
) -> Result<Json<GpioInfo>> {
    let gpio = session
        .platform
        .gpio(&gpio_id)
        .ok_or_else(|| Error::Other(format!("Invalid GPIO peripheral name: {gpio_id}")))?;
    let mut qtest_guard = session.qemu.lock().await;
    let qtest = qtest_guard.as_mut().ok_or(Error::QemuClosed)?;

    let gpio_report = gpio.read(qtest).await.map_err(|e| {
        tracing::error!("Error reading GPIO {gpio_id}: {e:?}");
        Error::Io(e)
    })?;

    let mut pins = HashMap::new();
    for pin in parts {
        let pin_report = gpio_report.pin_report(pin).map_err(Error::Io)?;
        pins.insert(pin, pin_report);
    }

    Ok(Json(GpioInfo {
        registers: gpio_report,
        pins,
    }))
}

async fn timer_info(
    Path(timer_id): Path<String>,
    Query(PeripheralPartQuery { parts }): Query<PeripheralPartQuery>,
    State(session): State<WebSession<Peripherals>>,
) -> Result<Json<TimerInfo>> {
    let timer = session
        .platform
        .get_timer(&timer_id)
        .ok_or_else(|| Error::Other(format!("Invalid timer peripheral name: {timer_id}")))?;
    let mut qtest_guard = session.qemu.lock().await;
    let qtest = qtest_guard.as_mut().ok_or(Error::QemuClosed)?;

    let timer_report = timer.read(qtest).await.map_err(Error::Io)?;

    let mut channels = Vec::new();
    for part in parts {
        let channel = Channel::try_from(part).map_err(Error::Io)?;
        channels.push(timer_report.channel_report(channel));
    }

    Ok(Json(TimerInfo {
        registers: timer_report,
        channels,
    }))
}
