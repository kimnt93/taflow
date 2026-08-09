use crate::error::TaResult;
use crate::stream::operator_states::validate_period;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
/// Rolling Shannon entropy over exact value frequencies.
pub struct RollingEntropy {
    ring: Box<[f64]>,
    head: usize,
    len: usize,
    counts: HashMap<u64, u32>,
    seen: HashSet<u64>,
    period: usize,
    value: Option<f64>,
}

impl RollingEntropy {
    #[inline]
    fn count_key(value: f64) -> u64 {
        if value == 0.0 {
            0.0f64.to_bits()
        } else {
            value.to_bits()
        }
    }

    pub fn new(period: usize) -> TaResult<Self> {
        validate_period(period)?;
        Ok(Self {
            ring: vec![0.0; period].into_boxed_slice(),
            head: 0,
            len: 0,
            counts: HashMap::with_capacity(period),
            seen: HashSet::with_capacity(period),
            period,
            value: None,
        })
    }

    pub fn append(&mut self, input: f64) -> Option<f64> {
        if self.len == self.period {
            let evicted = self.ring[self.head];
            if !evicted.is_nan() {
                let key = Self::count_key(evicted);
                let count = self.counts.get_mut(&key).expect("evicted value counted");
                *count -= 1;
                if *count == 0 {
                    self.counts.remove(&key);
                }
            }
        } else {
            self.len += 1;
        }
        self.ring[self.head] = input;
        self.head = (self.head + 1) % self.period;
        if !input.is_nan() {
            *self.counts.entry(Self::count_key(input)).or_insert(0) += 1;
        }
        self.value = (self.len == self.period).then(|| {
            let n = self.period as f64;
            let mut entropy = 0.0;
            self.seen.clear();
            for i in 0..self.period {
                let idx = (self.head + i) % self.period;
                let candidate = self.ring[idx];
                let probability = if candidate.is_nan() {
                    0.0
                } else {
                    let key = Self::count_key(candidate);
                    if !self.seen.insert(key) {
                        continue;
                    }
                    *self.counts.get(&key).expect("window value counted") as f64 / n
                };
                entropy -= probability * probability.ln();
            }
            entropy
        });
        self.value
    }

    pub fn value(&self) -> Option<f64> {
        self.value
    }

    pub fn reset(&mut self) {
        self.head = 0;
        self.len = 0;
        self.counts.clear();
        self.seen.clear();
        self.value = None;
    }
}
