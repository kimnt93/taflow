use crate::error::TaResult;
use crate::stream::invalid_period;

/// True-range Twiggs Money Flow with Wilder-smoothed flow and volume.
#[derive(Debug, Clone)]
pub struct TwiggsMoneyFlow {
    period: usize,
    previous_close: Option<f64>,
    seed_flow: f64,
    seed_volume: f64,
    seed_count: usize,
    flow_average: Option<f64>,
    volume_average: Option<f64>,
    value: Option<f64>,
}

impl TwiggsMoneyFlow {
    /// Create a Twiggs Money Flow state with a non-zero Wilder period.
    pub fn new(period: usize) -> TaResult<Self> {
        if period == 0 {
            return Err(invalid_period("period", period, 1));
        }
        Ok(Self {
            period,
            previous_close: None,
            seed_flow: 0.0,
            seed_volume: 0.0,
            seed_count: 0,
            flow_average: None,
            volume_average: None,
            value: None,
        })
    }

    /// Append one high/low/close/volume bar and return the smoothed ratio.
    pub fn append(&mut self, high: f64, low: f64, close: f64, volume: f64) -> Option<f64> {
        let Some(previous) = self.previous_close.replace(close) else {
            self.value = None;
            return None;
        };
        let true_high = high.max(previous);
        let true_low = low.min(previous);
        let range = true_high - true_low;
        let flow = if range > 0.0 {
            volume * (2.0 * close - true_high - true_low) / range
        } else {
            0.0
        };
        if let (Some(old_flow), Some(old_volume)) = (self.flow_average, self.volume_average) {
            let n = self.period as f64;
            let new_flow = old_flow + (flow - old_flow) / n;
            let new_volume = old_volume + (volume - old_volume) / n;
            self.flow_average = Some(new_flow);
            self.volume_average = Some(new_volume);
            self.value = Some(if new_volume == 0.0 {
                0.0
            } else {
                new_flow / new_volume
            });
            return self.value;
        }
        self.seed_flow += flow;
        self.seed_volume += volume;
        self.seed_count += 1;
        if self.seed_count == self.period {
            let n = self.period as f64;
            let flow_average = self.seed_flow / n;
            let volume_average = self.seed_volume / n;
            self.flow_average = Some(flow_average);
            self.volume_average = Some(volume_average);
            self.value = Some(if volume_average == 0.0 {
                0.0
            } else {
                flow_average / volume_average
            });
        }
        self.value
    }

    /// Return the latest money-flow ratio, or `None` during warm-up.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Clear seed and Wilder state while retaining the configured period.
    pub fn reset(&mut self) {
        self.previous_close = None;
        self.seed_flow = 0.0;
        self.seed_volume = 0.0;
        self.seed_count = 0;
        self.flow_average = None;
        self.volume_average = None;
        self.value = None;
    }
}
