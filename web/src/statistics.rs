use std::{
    collections::HashMap,
    sync::atomic::{AtomicU64, Ordering},
    time::Duration,
};

use axum::http::StatusCode;
use tracing::{error, info};

#[derive(Debug, Clone)]
pub struct Statistics {
    served: u64,
    status_codes: HashMap<StatusCode, u64>,
}

#[allow(clippy::declare_interior_mutable_const)]
const ZERO: AtomicU64 = AtomicU64::new(0);

static SERVED: AtomicU64 = ZERO;
static STATUS: [AtomicU64; 500] = [ZERO; 500];

impl Statistics {
    pub fn get() -> Self {
        let served = SERVED.load(Ordering::Relaxed);
        let mut status_codes = HashMap::new();

        for (status, value) in STATUS.iter().enumerate() {
            let value = value.load(Ordering::Relaxed);
            if value > 0 {
                let status = StatusCode::from_u16(status as u16 + 100).unwrap();
                status_codes.insert(status, value);
            }
        }

        Self {
            served,
            status_codes,
        }
    }

    pub fn register(code: StatusCode) {
        SERVED.fetch_add(1, Ordering::Relaxed);

        let index = code.as_u16() - 100;
        if let Some(bucket) = STATUS.get(index as usize) {
            bucket.fetch_add(1, Ordering::Relaxed);
        } else {
            error!(?code, index, "Unexpected index for statistics");
        }
    }

    pub fn reset() {
        SERVED.store(0, Ordering::Relaxed);
    }

    pub fn trace() {
        let stats = Statistics::get();
        info!(total_served = stats.served, codes =? stats.status_codes);
    }
}

pub fn spawn_tracing() {
    tokio::spawn(async {
        let mut interval = tokio::time::interval(Duration::from_secs(60));

        loop {
            interval.tick().await;
            Statistics::trace();
        }
    });
}
