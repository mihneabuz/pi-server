use std::sync::atomic::{AtomicU64, Ordering};

use tracing::info;

#[derive(Debug, Clone)]
pub struct Statistics {
    served: u64,
}

pub static SERVED: AtomicU64 = AtomicU64::new(0);

impl Statistics {
    pub fn get() -> Self {
        let served = SERVED.load(Ordering::Relaxed);

        Self { served }
    }

    pub fn reset() {
        SERVED.store(0, Ordering::Relaxed);
    }

    pub fn trace() {
        let stats = Self::get();
        info!(?stats, "Statistics");
    }
}
