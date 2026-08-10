use crate::error::TaResult;
use crate::stream::StreamingIndicator;
use std::f64::consts::PI;

/// Ehlers Hilbert-transform dominant-cycle period estimator.
#[derive(Debug, Clone)]
pub struct HilbertDominantCycle {
    smooth: [f64; 7],
    smooth_length: usize,
    detrender: [f64; 7],
    detrender_length: usize,
    quadrature: [f64; 7],
    quadrature_length: usize,
    in_phase: [f64; 7],
    in_phase_length: usize,
    previous_in_phase_two: f64,
    previous_quadrature_two: f64,
    previous_real: f64,
    previous_imaginary: f64,
    previous_period: f64,
    previous_smoothed_period: f64,
    count: usize,
    value: Option<f64>,
}

impl HilbertDominantCycle {
    /// Create an empty estimator; its first value is emitted on bar 50.
    pub fn new() -> TaResult<Self> {
        Ok(Self {
            smooth: [0.0; 7],
            smooth_length: 0,
            detrender: [0.0; 7],
            detrender_length: 0,
            quadrature: [0.0; 7],
            quadrature_length: 0,
            in_phase: [0.0; 7],
            in_phase_length: 0,
            previous_in_phase_two: 0.0,
            previous_quadrature_two: 0.0,
            previous_real: 0.0,
            previous_imaginary: 0.0,
            previous_period: 0.0,
            previous_smoothed_period: 0.0,
            count: 0,
            value: None,
        })
    }

    /// Append one price and update the bounded Hilbert-transform pipeline.
    pub fn append(&mut self, input: f64) -> Option<f64> {
        if !input.is_finite() {
            return self.value;
        }
        self.count += 1;
        push_front(&mut self.smooth, &mut self.smooth_length, input);
        if self.smooth_length < 4 {
            return None;
        }
        let smoothed =
            (4.0 * self.smooth[0] + 3.0 * self.smooth[1] + 2.0 * self.smooth[2] + self.smooth[3])
                / 10.0;
        let period = self.previous_period.clamp(6.0, 50.0);
        let adjustment = 0.075 * period + 0.54;
        if self.smooth_length < 7 {
            return None;
        }
        let detrender = hilbert(&self.smooth, smoothed, adjustment);
        push_front(&mut self.detrender, &mut self.detrender_length, detrender);
        if self.detrender_length < 7 {
            return None;
        }
        let quadrature_one = hilbert(&self.detrender, self.detrender[0], adjustment);
        let in_phase_one = self.detrender[3];
        push_front(
            &mut self.quadrature,
            &mut self.quadrature_length,
            quadrature_one,
        );
        push_front(&mut self.in_phase, &mut self.in_phase_length, in_phase_one);
        if self.quadrature_length < 7 || self.in_phase_length < 7 {
            return None;
        }
        let advanced_in_phase = hilbert(&self.in_phase, self.in_phase[0], adjustment);
        let advanced_quadrature = hilbert(&self.quadrature, self.quadrature[0], adjustment);
        let in_phase_two =
            0.2 * (in_phase_one - advanced_quadrature) + 0.8 * self.previous_in_phase_two;
        let quadrature_two =
            0.2 * (quadrature_one + advanced_in_phase) + 0.8 * self.previous_quadrature_two;
        let real = 0.2
            * (in_phase_two * self.previous_in_phase_two
                + quadrature_two * self.previous_quadrature_two)
            + 0.8 * self.previous_real;
        let imaginary = 0.2
            * (in_phase_two * self.previous_quadrature_two
                - quadrature_two * self.previous_in_phase_two)
            + 0.8 * self.previous_imaginary;
        self.previous_in_phase_two = in_phase_two;
        self.previous_quadrature_two = quadrature_two;
        self.previous_real = real;
        self.previous_imaginary = imaginary;
        let mut new_period = if imaginary.abs() > f64::EPSILON && real.abs() > f64::EPSILON {
            2.0 * PI / imaginary.atan2(real)
        } else {
            self.previous_period
        };
        new_period = new_period
            .min(1.5 * self.previous_period)
            .max(0.67 * self.previous_period)
            .clamp(6.0, 50.0);
        self.previous_period = 0.2 * new_period + 0.8 * self.previous_period;
        self.previous_smoothed_period =
            0.33 * self.previous_period + 0.67 * self.previous_smoothed_period;
        if self.count < 50 {
            return None;
        }
        self.value = Some(self.previous_smoothed_period);
        self.value
    }

    /// Return the latest dominant period, or `None` during warm-up.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Restore fresh-state behavior without reallocating.
    pub fn reset(&mut self) {
        self.smooth.fill(0.0);
        self.smooth_length = 0;
        self.detrender.fill(0.0);
        self.detrender_length = 0;
        self.quadrature.fill(0.0);
        self.quadrature_length = 0;
        self.in_phase.fill(0.0);
        self.in_phase_length = 0;
        self.previous_in_phase_two = 0.0;
        self.previous_quadrature_two = 0.0;
        self.previous_real = 0.0;
        self.previous_imaginary = 0.0;
        self.previous_period = 0.0;
        self.previous_smoothed_period = 0.0;
        self.count = 0;
        self.value = None;
    }
}

fn push_front(buffer: &mut [f64; 7], length: &mut usize, value: f64) {
    let used = (*length).min(6);
    buffer.copy_within(0..used, 1);
    buffer[0] = value;
    *length = (*length + 1).min(7);
}

fn hilbert(buffer: &[f64; 7], current: f64, adjustment: f64) -> f64 {
    (0.0962 * current + 0.5769 * buffer[2] - 0.5769 * buffer[4] - 0.0962 * buffer[6]) * adjustment
}

impl StreamingIndicator for HilbertDominantCycle {
    type Output = f64;
    fn append(&mut self, value: f64) -> Option<f64> {
        Self::append(self, value)
    }
    fn value(&self) -> Option<f64> {
        Self::value(self)
    }
    fn reset(&mut self) {
        Self::reset(self)
    }
}
