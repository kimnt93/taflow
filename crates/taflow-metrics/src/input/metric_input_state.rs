use crate::{MetricError, MetricResult};

/// Missing-value behavior applied before a converter mutates its state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NanPolicy {
    /// Ignore NaNs without advancing conversion or observation state.
    Omit,
    /// Reject the first NaN without advancing conversion state.
    Raise,
}

impl TryFrom<&str> for NanPolicy {
    type Error = MetricError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "omit" => Ok(Self::Omit),
            "raise" => Ok(Self::Raise),
            _ => Err(MetricError::InvalidParameter {
                name: "nan_policy",
                value: value.to_owned(),
                reason: "expected 'omit' or 'raise'",
            }),
        }
    }
}

/// Semantic domain selected by a metric factory.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MetricInputKind {
    /// Decimal simple returns, where `0.01` means one percent.
    Returns,
    /// Logarithmic returns converted with `expm1`.
    LogReturns,
    /// Strictly positive equity, NAV, or adjusted price levels.
    Equity,
    /// Period P&L converted causally to returns from the supplied capital.
    PeriodPnl { initial_equity: f64 },
    /// Raw period P&L for P&L-native metrics.
    RawPnl,
    /// Realized P&L for closed trades.
    Trades,
}

impl MetricInputKind {
    pub(crate) fn domain(self) -> &'static str {
        match self {
            Self::Returns => "return",
            Self::LogReturns => "log-return",
            Self::Equity => "equity",
            Self::PeriodPnl { .. } => "period-P&L",
            Self::RawPnl => "raw-P&L",
            Self::Trades => "trade",
        }
    }

    pub(crate) fn discriminant_matches(self, other: Self) -> bool {
        std::mem::discriminant(&self) == std::mem::discriminant(&other)
    }
}

/// Persistent input converter shared by canonical metric classes.
///
/// `append` returns a normalized simple return for return, log-return, equity,
/// and return-based period-P&L modes. Raw-P&L and trade modes return the input
/// unchanged. `None` means an omitted NaN or the first accepted equity level.
#[derive(Debug, Clone)]
pub struct MetricInputState {
    kind: MetricInputKind,
    nan_policy: NanPolicy,
    previous_equity: Option<f64>,
    current_pnl_equity: Option<f64>,
    len: usize,
    position: usize,
}

impl MetricInputState {
    /// Construct and validate an input converter.
    pub fn new(kind: MetricInputKind, nan_policy: NanPolicy) -> MetricResult<Self> {
        let current_pnl_equity = match kind {
            MetricInputKind::PeriodPnl { initial_equity }
                if !initial_equity.is_finite() || initial_equity <= 0.0 =>
            {
                return Err(MetricError::InvalidParameter {
                    name: "initial_equity",
                    value: initial_equity.to_string(),
                    reason: "must be finite and greater than zero",
                });
            }
            MetricInputKind::PeriodPnl { initial_equity } => Some(initial_equity),
            _ => None,
        };

        Ok(Self {
            kind,
            nan_policy,
            previous_equity: None,
            current_pnl_equity,
            len: 0,
            position: 0,
        })
    }

    /// Append one value and return its normalized observation when usable.
    pub fn append(&mut self, value: f64) -> MetricResult<Option<f64>> {
        let position = self.position;
        if value.is_nan() {
            if self.nan_policy == NanPolicy::Raise {
                return Err(self.invalid(
                    value,
                    position,
                    "NaN is forbidden by nan_policy='raise'",
                ));
            }
            self.position += 1;
            return Ok(None);
        }
        if !value.is_finite() {
            return Err(self.invalid(value, position, "infinite values are not supported"));
        }
        if self.kind == MetricInputKind::Returns && value < -1.0 {
            return Err(self.invalid(
                value,
                position,
                "simple returns must be greater than or equal to -1",
            ));
        }

        let normalized = match self.kind {
            MetricInputKind::Returns | MetricInputKind::RawPnl | MetricInputKind::Trades => {
                Some(value)
            }
            MetricInputKind::LogReturns => {
                let result = value.exp_m1();
                if !result.is_finite() {
                    return Err(self.invalid(
                        value,
                        position,
                        "conversion with expm1 must produce a finite simple return",
                    ));
                }
                Some(result)
            }
            MetricInputKind::Equity => {
                if value <= 0.0 {
                    return Err(self.invalid(value, position, "equity must be greater than zero"));
                }
                match self.previous_equity {
                    Some(previous) => {
                        let result = value / previous - 1.0;
                        if !result.is_finite() {
                            return Err(self.invalid(
                                value,
                                position,
                                "level conversion must produce a finite simple return",
                            ));
                        }
                        self.previous_equity = Some(value);
                        Some(result)
                    }
                    None => {
                        self.previous_equity = Some(value);
                        None
                    }
                }
            }
            MetricInputKind::PeriodPnl { .. } => {
                let equity = self
                    .current_pnl_equity
                    .expect("validated period-P&L mode owns equity state");
                if equity == 0.0 {
                    return Err(self.invalid(
                        value,
                        position,
                        "P&L cannot continue after equity reaches zero",
                    ));
                }
                let next_equity = equity + value;
                if !next_equity.is_finite() || next_equity < 0.0 {
                    return Err(self.invalid(
                        value,
                        position,
                        "P&L must leave finite non-negative equity",
                    ));
                }
                let result = value / equity;
                if !result.is_finite() {
                    return Err(self.invalid(
                        value,
                        position,
                        "P&L conversion must produce a finite simple return",
                    ));
                }
                self.current_pnl_equity = Some(next_equity);
                Some(result)
            }
        };

        self.position += 1;
        if normalized.is_some() {
            self.len += 1;
        }
        Ok(normalized)
    }

    /// Restore fresh converter behavior while preserving mode and configuration.
    pub fn reset(&mut self) {
        self.previous_equity = None;
        self.current_pnl_equity = match self.kind {
            MetricInputKind::PeriodPnl { initial_equity } => Some(initial_equity),
            _ => None,
        };
        self.len = 0;
        self.position = 0;
    }

    /// Return the number of usable normalized observations emitted.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Return whether no usable observations have been emitted.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Return the configured semantic input domain.
    pub fn kind(&self) -> MetricInputKind {
        self.kind
    }

    /// Return the configured missing-value policy.
    pub fn nan_policy(&self) -> NanPolicy {
        self.nan_policy
    }

    pub(crate) fn skip_missing_pair(&mut self) {
        self.position += 1;
    }

    fn invalid(&self, value: f64, position: usize, reason: &'static str) -> MetricError {
        MetricError::InvalidObservation {
            domain: self.kind.domain(),
            position,
            value: value.to_string(),
            reason,
        }
    }
}
