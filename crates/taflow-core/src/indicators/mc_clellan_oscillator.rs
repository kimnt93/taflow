use crate::error::TaResult;
/// Fast-minus-slow EMA of ratio-adjusted net advances.
#[derive(Debug, Clone)]
pub struct McClellanOscillator {
    fast: Option<f64>,
    slow: Option<f64>,
    count: usize,
    value: Option<f64>,
}
impl McClellanOscillator {
    /// Create an unseeded classic 19/39 oscillator state.
    pub fn new() -> TaResult<Self> {
        Ok(Self {
            fast: None,
            slow: None,
            count: 0,
            value: None,
        })
    }
    /// Append aggregate advancing and declining issue counts.
    pub fn append(&mut self, advancers: f64, decliners: f64) -> Option<f64> {
        self.count += 1;
        let breadth = (advancers - decliners) / (advancers + decliners).max(1.0) * 1000.0;
        let f = self.fast.map_or(breadth, |x| x + 0.1 * (breadth - x));
        let s = self.slow.map_or(breadth, |x| x + 0.05 * (breadth - x));
        self.fast = Some(f);
        self.slow = Some(s);
        self.value = Some(f - s);
        self.value
    }
    /// Return the latest oscillator value.
    pub fn value(&self) -> Option<f64> {
        self.value
    }
    /// Return the number of processed market ticks.
    pub fn len(&self) -> usize {
        self.count
    }
    /// Return whether no market ticks have been processed.
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
    /// Clear both EMA seeds and the latest value.
    pub fn reset(&mut self) {
        self.fast = None;
        self.slow = None;
        self.count = 0;
        self.value = None;
    }
}
