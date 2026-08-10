use crate::error::TaResult;
use crate::indicators::Decycler;
use crate::stream::{invalid_period, StreamingIndicator};

/// Difference between fast and slow Ehlers decyclers.
#[derive(Debug, Clone)]
pub struct DecyclerOscillator {
    fast: Decycler,
    slow: Decycler,
    value: Option<f64>,
}

impl DecyclerOscillator {
    /// Create an oscillator requiring non-zero periods and `fast < slow`.
    pub fn new(fast: usize, slow: usize) -> TaResult<Self> {
        if fast == 0 {
            return Err(invalid_period("fast", fast, 1));
        }
        if slow <= fast {
            return Err(invalid_period("slow", slow, fast + 1));
        }
        Ok(Self {
            fast: Decycler::new(fast)?,
            slow: Decycler::new(slow)?,
            value: None,
        })
    }

    /// Append one sample and return fast decycler minus slow decycler.
    pub fn append(&mut self, input: f64) -> Option<f64> {
        self.value = Some(self.fast.append(input)? - self.slow.append(input)?);
        self.value
    }

    /// Return the latest oscillator value.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Reset both child filters and the latest value.
    pub fn reset(&mut self) {
        self.fast.reset();
        self.slow.reset();
        self.value = None;
    }
}

impl StreamingIndicator for DecyclerOscillator {
    type Output = f64;
    fn append(&mut self, input: f64) -> Option<f64> {
        Self::append(self, input)
    }
    fn value(&self) -> Option<f64> {
        self.value
    }
    fn reset(&mut self) {
        Self::reset(self)
    }
}
