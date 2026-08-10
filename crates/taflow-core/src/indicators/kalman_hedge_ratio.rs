use crate::error::{TaError, TaResult};

/// Online Kalman estimate of the hedge ratio `β` in `y = α + β·x + v`.
///
/// Two-state filter with random-walk transition (`Q = δ / (1-δ)·I`) and
/// observation noise `R`, matching Wickra's dynamic hedge-ratio definition.
/// The primary output is `β`; `α`, the innovation, and `√S` are also exposed.
/// O(1) per bar — no linear-algebra dependency.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `KalmanHedgeRatio`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct KalmanHedgeRatio {
    alpha: f64,
    beta: f64,
    p_aa: f64,
    p_ab: f64,
    p_bb: f64,
    transition_variance: f64,
    observation_variance: f64,
    count: usize,
    value: Option<f64>,
    alpha_value: Option<f64>,
    innovation: Option<f64>,
    std_value: Option<f64>,
}

impl KalmanHedgeRatio {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(delta: f64, observation_variance: f64) -> TaResult<Self> {
        if !delta.is_finite() || !(0.0..1.0).contains(&delta) {
            return Err(TaError::InvalidParameter {
                name: "delta",
                value: delta.to_string(),
                reason: "must be finite and in (0, 1)",
            });
        }
        if !(observation_variance > 0.0) {
            return Err(TaError::InvalidParameter {
                name: "observation_variance",
                value: observation_variance.to_string(),
                reason: "must be > 0",
            });
        }
        Ok(Self {
            alpha: 0.0,
            beta: 0.0,
            p_aa: 0.0,
            p_ab: 0.0,
            p_bb: 0.0,
            transition_variance: delta / (1.0 - delta),
            observation_variance,
            count: 0,
            value: None,
            alpha_value: None,
            innovation: None,
            std_value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, x: f64, y: f64) -> Option<f64> {
        // Wickra starts from a zero covariance prior. Process noise is added
        // only after the first observation has established that prior.
        let process_noise = if self.count == 0 {
            0.0
        } else {
            self.transition_variance
        };
        let p_aa = self.p_aa + process_noise;
        let p_ab = self.p_ab;
        let p_bb = self.p_bb + process_noise;

        // Innovation and Kalman gain.
        let innovation = y - (self.alpha + self.beta * x);
        let projected_beta = p_bb * x + p_ab;
        let projected_alpha = p_ab * x + p_aa;
        let s = projected_beta * x + projected_alpha + self.observation_variance;
        let beta_gain = projected_beta / s;
        let alpha_gain = projected_alpha / s;

        // Update state.
        self.beta += beta_gain * innovation;
        self.alpha += alpha_gain * innovation;

        // Update covariance: P = (I - K·H)·P.
        self.p_bb = p_bb - beta_gain * projected_beta;
        self.p_ab = p_ab - beta_gain * projected_alpha;
        self.p_aa = p_aa - alpha_gain * projected_alpha;
        self.count += 1;

        self.value = Some(self.beta);
        self.alpha_value = Some(self.alpha);
        self.innovation = Some(innovation);
        self.std_value = Some(s.sqrt());
        self.value
    }

    /// Computes or updates `value` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Return the current smoothing factor, if available.
    ///
    pub fn alpha(&self) -> Option<f64> {
        self.alpha_value
    }

    /// Computes or updates `innovation` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn innovation(&self) -> Option<f64> {
        self.innovation
    }

    /// Return the current standard deviation, if available.
    ///
    pub fn std(&self) -> Option<f64> {
        self.std_value
    }

    /// Computes or updates `reset` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn reset(&mut self) {
        self.alpha = 0.0;
        self.beta = 0.0;
        self.p_aa = 0.0;
        self.p_ab = 0.0;
        self.p_bb = 0.0;
        self.count = 0;
        self.value = None;
        self.alpha_value = None;
        self.innovation = None;
        self.std_value = None;
    }
}
