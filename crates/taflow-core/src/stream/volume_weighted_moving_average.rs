//! Persistent volume-weighted moving average state.

use super::operator_states::{validate_period, ContiguousWindow};
use crate::error::TaResult;

#[derive(Debug, Clone)]
pub struct VolumeWeightedMovingAverage {
    prices: ContiguousWindow,
    volumes: ContiguousWindow,
    value: Option<f64>,
}

impl VolumeWeightedMovingAverage {
    pub fn new(period: usize) -> TaResult<Self> {
        validate_period(period)?;
        Ok(Self {
            prices: ContiguousWindow::new(period),
            volumes: ContiguousWindow::new(period),
            value: None,
        })
    }

    pub fn append(&mut self, price: f64, volume: f64) -> Option<f64> {
        self.prices.push(price);
        self.volumes.push(volume);
        self.value = self.prices.is_full().then(|| {
            let prices = self.prices.window();
            let volumes = self.volumes.window();
            let total_volume = volumes.iter().sum::<f64>();
            if total_volume != 0.0 {
                prices
                    .iter()
                    .zip(volumes)
                    .map(|(&price, &volume)| price * volume)
                    .sum::<f64>()
                    / total_volume
            } else {
                0.0
            }
        });
        self.value
    }

    pub fn value(&self) -> Option<f64> {
        self.value
    }

    pub fn reset(&mut self) {
        self.prices.clear();
        self.volumes.clear();
        self.value = None;
    }
}
