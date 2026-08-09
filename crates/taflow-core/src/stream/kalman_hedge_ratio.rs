use crate::error::{TaError, TaResult};

/// Online Kalman estimate of the hedge ratio `β` in `y = α + β·x + v`.
///
/// Two-state filter with random-walk transition (`Q = δ·I`) and observation
/// noise `R` (QuantStart "Dynamic Hedge Ratio"; pykalman `filter_update`).
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
    delta: f64,
    observation_variance: f64,
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
        if !(delta >= 0.0) {
            return Err(TaError::InvalidParameter {
                name: "delta",
                value: delta.to_string(),
                reason: "must be >= 0",
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
            beta: 1.0,
            p_aa: 1.0,
            p_ab: 0.0,
            p_bb: 1.0,
            delta,
            observation_variance,
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
        // Predict: θ stays, P += Q (Q = delta·I adds to the diagonal).
        let p_aa = self.p_aa + self.delta;
        let p_ab = self.p_ab;
        let p_bb = self.p_bb + self.delta;

        // Innovation and Kalman gain.
        let innovation = y - (self.alpha + self.beta * x);
        let s = p_aa + 2.0 * p_ab * x + p_bb * x * x + self.observation_variance;
        let k1 = (p_aa + p_ab * x) / s;
        let k2 = (p_ab + p_bb * x) / s;

        // Update state.
        self.alpha += k1 * innovation;
        self.beta += k2 * innovation;

        // Update covariance: P = (I - K·H)·P.
        let p_aa_new = (1.0 - k1) * p_aa - k1 * x * p_ab;
        let p_ab_new = (1.0 - k1) * p_ab - k1 * x * p_bb;
        let p_bb_new = -k2 * p_ab + (1.0 - k2 * x) * p_bb;
        self.p_aa = p_aa_new;
        self.p_ab = p_ab_new;
        self.p_bb = p_bb_new;

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
        self.beta = 1.0;
        self.p_aa = 1.0;
        self.p_ab = 0.0;
        self.p_bb = 1.0;
        self.value = None;
        self.alpha_value = None;
        self.innovation = None;
        self.std_value = None;
    }
}
