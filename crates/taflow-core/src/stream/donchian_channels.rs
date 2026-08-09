use super::donchian::{Donchian, DonchianValue};
use crate::error::TaResult;

#[derive(Debug, Clone)]
/// Canonical descriptive wrapper for the persistent Donchian channel state.
pub struct DonchianChannels {
    inner: Donchian,
}

impl DonchianChannels {
    pub fn new(period: usize) -> TaResult<Self> {
        Ok(Self {
            inner: Donchian::new(period)?,
        })
    }

    pub fn append(&mut self, high: f64, low: f64) -> Option<DonchianValue> {
        self.inner.append(high, low)
    }

    pub fn value(&self) -> Option<DonchianValue> {
        self.inner.value()
    }

    pub fn reset(&mut self) {
        self.inner.reset();
    }
}
