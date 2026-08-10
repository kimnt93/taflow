use crate::error::TaResult;
#[derive(Debug, Clone)]
pub struct McClellanOscillator {
    fast: Option<f64>,
    slow: Option<f64>,
    count: usize,
    value: Option<f64>,
}
impl McClellanOscillator {
    pub fn new() -> TaResult<Self> {
        Ok(Self {
            fast: None,
            slow: None,
            count: 0,
            value: None,
        })
    }
    pub fn append(
        &mut self,
        change: f64,
        _volume: f64,
        _new_high: f64,
        _new_low: f64,
    ) -> Option<f64> {
        self.count += 1;
        let f = self.fast.map_or(change, |x| x + 0.1 * (change - x));
        let s = self.slow.map_or(change, |x| x + 0.05 * (change - x));
        self.fast = Some(f);
        self.slow = Some(s);
        self.value = Some(f - s);
        self.value
    }
    pub fn value(&self) -> Option<f64> {
        self.value
    }
    pub fn len(&self) -> usize {
        self.count
    }
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
    pub fn reset(&mut self) {
        self.fast = None;
        self.slow = None;
        self.count = 0;
        self.value = None;
    }
}
