use crate::error::TaResult;
use crate::stream::{invalid_period, StreamingIndicator};

/// Ehlers two-pole instantaneous trendline with its standard initial condition.
#[derive(Debug, Clone)]
pub struct InstantaneousTrendline {
    alpha: f64,
    inputs: [Option<f64>; 3],
    outputs: [Option<f64>; 2],
    count: usize,
    value: Option<f64>,
}

impl InstantaneousTrendline {
    /// Create an instantaneous trendline with a non-zero cycle period.
    pub fn new(period: usize) -> TaResult<Self> {
        if period == 0 {
            return Err(invalid_period("period", period, 1));
        }
        Ok(Self {
            alpha: 2.0 / (period as f64 + 1.0),
            inputs: [None; 3],
            outputs: [None; 2],
            count: 0,
            value: None,
        })
    }

    /// Append one sample and return the initialized or recursive trendline.
    pub fn append(&mut self, input: f64) -> Option<f64> {
        self.count += 1;
        self.inputs[2] = self.inputs[1];
        self.inputs[1] = self.inputs[0];
        self.inputs[0] = Some(input);
        let x0 = input;
        let x1 = self.inputs[1].unwrap_or(x0);
        let x2 = self.inputs[2].unwrap_or(x0);
        let output = if self.count >= 7 {
            let y1 = self.outputs[0].expect("trendline output history");
            let y2 = self.outputs[1].expect("trendline output history");
            let square = self.alpha * self.alpha;
            (self.alpha - square * 0.25) * x0 + 0.5 * square * x1
                - (self.alpha - 0.75 * square) * x2
                + 2.0 * (1.0 - self.alpha) * y1
                - (1.0 - self.alpha).powi(2) * y2
        } else {
            (x0 + 2.0 * x1 + x2) * 0.25
        };
        self.outputs[1] = self.outputs[0];
        self.outputs[0] = Some(output);
        self.value = Some(output);
        self.value
    }

    /// Return the latest trendline value.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Clear input, output, and initialization history.
    pub fn reset(&mut self) {
        self.inputs = [None; 3];
        self.outputs = [None; 2];
        self.count = 0;
        self.value = None;
    }
}

impl StreamingIndicator for InstantaneousTrendline {
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
