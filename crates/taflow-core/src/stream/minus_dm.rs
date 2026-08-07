//! Incremental Minus Directional Movement (-DM).
use crate::error::{TaError, TaResult};
pub struct MinusDm {
    p: f64,
    n: usize,
    prev: Option<(f64, f64)>,
    sum: f64,
    v: Option<f64>,
}
impl MinusDm {
    pub fn new(p: usize) -> TaResult<Self> {
        if p == 0 {
            return Err(TaError::InvalidParameter {
                name: "timeperiod",
                value: "0".into(),
                reason: "must be >= 1",
            });
        }
        Ok(Self {
            p: p as f64,
            n: 0,
            prev: None,
            sum: 0.0,
            v: None,
        })
    }
    pub fn append(&mut self, h: f64, l: f64) -> Option<f64> {
        let Some((ph, pl)) = self.prev.replace((h, l)) else {
            return if self.p == 1.0 {
                self.v = Some(0.0);
                self.v
            } else {
                None
            };
        };
        let up = h - ph;
        let down = pl - l;
        let dm = if down > up && down > 0.0 { down } else { 0.0 };
        self.n += 1;
        let p = self.p as usize;
        if self.n < p - 1 {
            self.sum += dm;
            return None;
        }
        if self.n == p - 1 {
            self.sum += dm
        } else {
            self.sum = self.sum - self.sum / self.p + dm
        }
        self.v = Some(self.sum);
        self.v
    }
    pub fn value(&self) -> Option<f64> {
        self.v
    }
    pub fn reset(&mut self) {
        self.n = 0;
        self.prev = None;
        self.sum = 0.0;
        self.v = None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_batch() {
        let high: Vec<f64> = (0..40).map(|i| 100.0 + i as f64 * 0.1).collect();
        let low: Vec<f64> = (0..40).map(|i| 98.0 - i as f64 * 0.3).collect();
        let expected = crate::momentum::minus_directional_movement(&high, &low, 14).unwrap();
        let mut state = MinusDm::new(14).unwrap();

        for ((&high, &low), expected) in high.iter().zip(&low).zip(&expected) {
            match state.append(high, low) {
                Some(actual) => assert!((actual - expected).abs() < 1e-12),
                None => assert!(expected.is_nan()),
            }
        }
    }
}
