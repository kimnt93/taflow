use crate::error::TaResult;
use crate::stream::operator_states::{validate_period, ContiguousWindow};

#[derive(Debug, Clone)]
/// Rolling volume-weighted average of typical prices.
pub struct RollingVolumeWeightedAveragePrice {
    prices: ContiguousWindow,
    volumes: ContiguousWindow,
    value: Option<f64>,
}

impl RollingVolumeWeightedAveragePrice {
    pub fn new(period: usize) -> TaResult<Self> {
        validate_period(period)?;
        Ok(Self {
            prices: ContiguousWindow::new(period),
            volumes: ContiguousWindow::new(period),
            value: None,
        })
    }

    pub fn append(&mut self, high: f64, low: f64, close: f64, volume: f64) -> Option<f64> {
        self.prices.push((high + low + close) / 3.0);
        self.volumes.push(volume);
        self.value = self.prices.is_full().then(|| {
            let prices = self.prices.window();
            let volumes = self.volumes.window();
            let total = volumes.iter().sum::<f64>();
            if total != 0.0 {
                prices
                    .iter()
                    .zip(volumes)
                    .map(|(&price, &volume)| price * volume)
                    .sum::<f64>()
                    / total
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
