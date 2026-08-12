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

/// Semantic domain selected by a metric instance input method.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MetricInputKind {
    /// Decimal simple returns, where `0.01` means one percent.
    Returns,
    /// Logarithmic returns converted with `expm1`.
    LogReturns,
    /// Strictly positive equity, NAV, or adjusted price levels.
    Equity,
    /// Period P&L converted causally to returns from the supplied capital.
    PeriodPnl { initial_capital: f64 },
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
    bound: bool,
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
            MetricInputKind::PeriodPnl { initial_capital }
                if !initial_capital.is_finite() || initial_capital <= 0.0 =>
            {
                return Err(MetricError::InvalidParameter {
                    name: "initial_capital",
                    value: initial_capital.to_string(),
                    reason: "must be finite and greater than zero",
                });
            }
            MetricInputKind::PeriodPnl { initial_capital } => Some(initial_capital),
            _ => None,
        };

        Ok(Self {
            kind,
            bound: true,
            nan_policy,
            previous_equity: None,
            current_pnl_equity,
            len: 0,
            position: 0,
        })
    }

    /// Construct a converter whose semantic domain will be selected by the
    /// first `from_*` lifecycle call.
    pub fn unbound(nan_policy: NanPolicy) -> Self {
        Self {
            kind: MetricInputKind::Returns,
            bound: false,
            nan_policy,
            previous_equity: None,
            current_pnl_equity: None,
            len: 0,
            position: 0,
        }
    }

    /// Select a semantic input domain without changing metric observations.
    /// Re-selecting the same domain is allowed so repeated `from_*` calls can
    /// extend one stream; changing domains after selection is rejected.
    pub fn bind(&mut self, kind: MetricInputKind) -> MetricResult<()> {
        if self.bound {
            if self.kind == kind {
                return Ok(());
            }
            return Err(MetricError::InvalidParameter {
                name: "input_kind",
                value: format!("{kind:?}"),
                reason: "input domain is already selected for this metric",
            });
        }

        let validated = Self::new(kind, self.nan_policy)?;
        self.kind = validated.kind;
        self.bound = true;
        self.current_pnl_equity = validated.current_pnl_equity;
        Ok(())
    }

    /// Append one value and return its normalized observation when usable.
    pub fn append(&mut self, value: f64) -> MetricResult<Option<f64>> {
        self.ensure_bound()?;
        self.validate_next(value)?;
        if value.is_nan() {
            self.position += 1;
            return Ok(None);
        }
        Ok(self.append_validated(value))
    }

    /// Validate the next value without changing converter state.
    pub(crate) fn validate_next(&self, value: f64) -> MetricResult<()> {
        let position = self.position;
        if value.is_nan() {
            return if self.nan_policy == NanPolicy::Raise {
                Err(self.invalid(value, position, "NaN is forbidden by nan_policy='raise'"))
            } else {
                Ok(())
            };
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
        match self.kind {
            MetricInputKind::LogReturns if !value.exp_m1().is_finite() => Err(self.invalid(
                value,
                position,
                "conversion with expm1 must produce a finite simple return",
            )),
            MetricInputKind::Equity if value <= 0.0 => {
                Err(self.invalid(value, position, "equity must be greater than zero"))
            }
            MetricInputKind::Equity => {
                if let Some(previous) = self.previous_equity {
                    if !(value / previous - 1.0).is_finite() {
                        return Err(self.invalid(
                            value,
                            position,
                            "level conversion must produce a finite simple return",
                        ));
                    }
                }
                Ok(())
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
                if !(value / equity).is_finite() {
                    return Err(self.invalid(
                        value,
                        position,
                        "P&L conversion must produce a finite simple return",
                    ));
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }

    /// Convert a finite, non-missing value already accepted by `validate_next`.
    pub(crate) fn append_validated(&mut self, value: f64) -> Option<f64> {
        let normalized = match self.kind {
            MetricInputKind::Returns | MetricInputKind::RawPnl | MetricInputKind::Trades => {
                Some(value)
            }
            MetricInputKind::LogReturns => {
                let result = value.exp_m1();
                Some(result)
            }
            MetricInputKind::Equity => match self.previous_equity {
                Some(previous) => {
                    let result = value / previous - 1.0;
                    self.previous_equity = Some(value);
                    Some(result)
                }
                None => {
                    self.previous_equity = Some(value);
                    None
                }
            },
            MetricInputKind::PeriodPnl { .. } => {
                let equity = self
                    .current_pnl_equity
                    .expect("validated period-P&L mode owns equity state");
                let next_equity = equity + value;
                let result = value / equity;
                self.current_pnl_equity = Some(next_equity);
                Some(result)
            }
        };

        self.position += 1;
        if normalized.is_some() {
            self.len += 1;
        }
        normalized
    }

    /// Convert a chronological slice with the semantic-domain branch hoisted
    /// out of the hot loop, invoking `consume` for every usable observation.
    pub fn extend(
        &mut self,
        values: &[f64],
        mut consume: impl FnMut(f64) -> MetricResult<()>,
    ) -> MetricResult<()> {
        self.ensure_bound()?;
        match self.kind {
            MetricInputKind::Returns => {
                for &value in values {
                    if value.is_nan() {
                        self.handle_missing()?;
                    } else if !value.is_finite() {
                        return Err(self.invalid(
                            value,
                            self.position,
                            "infinite values are not supported",
                        ));
                    } else if value < -1.0 {
                        return Err(self.invalid(
                            value,
                            self.position,
                            "simple returns must be greater than or equal to -1",
                        ));
                    } else {
                        self.accept(value, &mut consume)?;
                    }
                }
            }
            MetricInputKind::RawPnl | MetricInputKind::Trades => {
                for &value in values {
                    if value.is_nan() {
                        self.handle_missing()?;
                    } else if !value.is_finite() {
                        return Err(self.invalid(
                            value,
                            self.position,
                            "infinite values are not supported",
                        ));
                    } else {
                        self.accept(value, &mut consume)?;
                    }
                }
            }
            MetricInputKind::LogReturns => {
                for &value in values {
                    if value.is_nan() {
                        self.handle_missing()?;
                    } else if !value.is_finite() {
                        return Err(self.invalid(
                            value,
                            self.position,
                            "infinite values are not supported",
                        ));
                    } else {
                        let simple_return = value.exp_m1();
                        if !simple_return.is_finite() {
                            return Err(self.invalid(
                                value,
                                self.position,
                                "conversion with expm1 must produce a finite simple return",
                            ));
                        }
                        self.accept(simple_return, &mut consume)?;
                    }
                }
            }
            MetricInputKind::Equity => {
                for &value in values {
                    if value.is_nan() {
                        self.handle_missing()?;
                        continue;
                    }
                    if !value.is_finite() {
                        return Err(self.invalid(
                            value,
                            self.position,
                            "infinite values are not supported",
                        ));
                    }
                    if value <= 0.0 {
                        return Err(self.invalid(
                            value,
                            self.position,
                            "equity must be greater than zero",
                        ));
                    }
                    if let Some(previous) = self.previous_equity {
                        let simple_return = value / previous - 1.0;
                        if !simple_return.is_finite() {
                            return Err(self.invalid(
                                value,
                                self.position,
                                "level conversion must produce a finite simple return",
                            ));
                        }
                        self.previous_equity = Some(value);
                        self.position += 1;
                        self.len += 1;
                        consume(simple_return)?;
                    } else {
                        self.previous_equity = Some(value);
                        self.position += 1;
                    }
                }
            }
            MetricInputKind::PeriodPnl { .. } => {
                for &value in values {
                    if value.is_nan() {
                        self.handle_missing()?;
                        continue;
                    }
                    if !value.is_finite() {
                        return Err(self.invalid(
                            value,
                            self.position,
                            "infinite values are not supported",
                        ));
                    }
                    let equity = self
                        .current_pnl_equity
                        .expect("validated period-P&L mode owns equity state");
                    if equity == 0.0 {
                        return Err(self.invalid(
                            value,
                            self.position,
                            "P&L cannot continue after equity reaches zero",
                        ));
                    }
                    let next_equity = equity + value;
                    if !next_equity.is_finite() || next_equity < 0.0 {
                        return Err(self.invalid(
                            value,
                            self.position,
                            "P&L must leave finite non-negative equity",
                        ));
                    }
                    let simple_return = value / equity;
                    if !simple_return.is_finite() {
                        return Err(self.invalid(
                            value,
                            self.position,
                            "P&L conversion must produce a finite simple return",
                        ));
                    }
                    self.current_pnl_equity = Some(next_equity);
                    self.accept(simple_return, &mut consume)?;
                }
            }
        }
        Ok(())
    }

    /// Ingest already validated simple returns from a trusted native fan-out.
    pub(crate) fn extend_normalized_returns(
        &mut self,
        values: &[f64],
        mut consume: impl FnMut(f64) -> MetricResult<()>,
    ) -> MetricResult<()> {
        self.ensure_bound()?;
        debug_assert_eq!(self.kind, MetricInputKind::Returns);
        for &value in values {
            self.position += 1;
            self.len += 1;
            consume(value)?;
        }
        Ok(())
    }

    #[inline]
    fn accept(
        &mut self,
        value: f64,
        consume: &mut impl FnMut(f64) -> MetricResult<()>,
    ) -> MetricResult<()> {
        self.position += 1;
        self.len += 1;
        consume(value)
    }

    #[inline]
    fn handle_missing(&mut self) -> MetricResult<()> {
        if self.nan_policy == NanPolicy::Raise {
            return Err(self.invalid(
                f64::NAN,
                self.position,
                "NaN is forbidden by nan_policy='raise'",
            ));
        }
        self.position += 1;
        Ok(())
    }

    /// Restore fresh converter behavior while preserving mode and configuration.
    pub fn reset(&mut self) {
        self.previous_equity = None;
        self.current_pnl_equity = match self.kind {
            MetricInputKind::PeriodPnl { initial_capital } => Some(initial_capital),
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

    /// Return whether a semantic `from_*` lifecycle call selected the domain.
    pub fn is_bound(&self) -> bool {
        self.bound
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

    fn ensure_bound(&self) -> MetricResult<()> {
        if self.bound {
            Ok(())
        } else {
            Err(MetricError::InvalidParameter {
                name: "input_kind",
                value: "unbound".to_owned(),
                reason: "call a semantic from_* method before append or extend",
            })
        }
    }
}

/// Add the standard return-domain ingestion lifecycle to a metric whose input
/// converter is stored in an `input` field and whose numerical bulk path is
/// its inherent `extend` method.
#[macro_export]
macro_rules! impl_return_metric_lifecycle {
    ($state:ty) => {
        impl $state {
            pub fn from_returns(&mut self, values: &[f64]) -> $crate::MetricResult<&mut Self> {
                self.input.bind($crate::MetricInputKind::Returns)?;
                self.extend(values)?;
                Ok(self)
            }

            pub fn from_log_returns(&mut self, values: &[f64]) -> $crate::MetricResult<&mut Self> {
                self.input.bind($crate::MetricInputKind::LogReturns)?;
                self.extend(values)?;
                Ok(self)
            }

            pub fn from_equity(&mut self, values: &[f64]) -> $crate::MetricResult<&mut Self> {
                self.input.bind($crate::MetricInputKind::Equity)?;
                self.extend(values)?;
                Ok(self)
            }

            pub fn from_pnl(
                &mut self,
                values: &[f64],
                initial_capital: f64,
            ) -> $crate::MetricResult<&mut Self> {
                self.input.bind($crate::MetricInputKind::PeriodPnl {
                    initial_capital: initial_capital,
                })?;
                self.extend(values)?;
                Ok(self)
            }
        }
    };
}

/// Add raw observation ingestion for metrics that accept returns, period P&L,
/// and closed-trade P&L without capital conversion.
#[macro_export]
macro_rules! impl_observation_metric_lifecycle {
    ($state:ty) => {
        impl $state {
            pub fn from_returns(&mut self, values: &[f64]) -> $crate::MetricResult<&mut Self> {
                self.input.bind($crate::MetricInputKind::Returns)?;
                self.extend(values)?;
                Ok(self)
            }

            pub fn from_pnl(&mut self, values: &[f64]) -> $crate::MetricResult<&mut Self> {
                self.input.bind($crate::MetricInputKind::RawPnl)?;
                self.extend(values)?;
                Ok(self)
            }

            pub fn from_trades(&mut self, values: &[f64]) -> $crate::MetricResult<&mut Self> {
                self.input.bind($crate::MetricInputKind::Trades)?;
                self.extend(values)?;
                Ok(self)
            }
        }
    };
}

/// Add raw P&L and closed-trade ingestion for monetary observation metrics.
#[macro_export]
macro_rules! impl_pnl_trade_metric_lifecycle {
    ($state:ty) => {
        impl $state {
            pub fn from_pnl(&mut self, values: &[f64]) -> $crate::MetricResult<&mut Self> {
                self.input.bind($crate::MetricInputKind::RawPnl)?;
                self.extend(values)?;
                Ok(self)
            }

            pub fn from_trades(&mut self, values: &[f64]) -> $crate::MetricResult<&mut Self> {
                self.input.bind($crate::MetricInputKind::Trades)?;
                self.extend(values)?;
                Ok(self)
            }
        }
    };
}

/// Add return and closed-trade ingestion for metrics defined on either scale.
#[macro_export]
macro_rules! impl_return_trade_metric_lifecycle {
    ($state:ty) => {
        impl $state {
            pub fn from_returns(&mut self, values: &[f64]) -> $crate::MetricResult<&mut Self> {
                self.input.bind($crate::MetricInputKind::Returns)?;
                self.extend(values)?;
                Ok(self)
            }

            pub fn from_trades(&mut self, values: &[f64]) -> $crate::MetricResult<&mut Self> {
                self.input.bind($crate::MetricInputKind::Trades)?;
                self.extend(values)?;
                Ok(self)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_returns_only_metric_lifecycle {
    ($state:ty) => {
        impl $state {
            pub fn from_returns(&mut self, values: &[f64]) -> $crate::MetricResult<&mut Self> {
                self.input.bind($crate::MetricInputKind::Returns)?;
                self.extend(values)?;
                Ok(self)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_trades_only_metric_lifecycle {
    ($state:ty) => {
        impl $state {
            pub fn from_trades(&mut self, values: &[f64]) -> $crate::MetricResult<&mut Self> {
                self.input.bind($crate::MetricInputKind::Trades)?;
                self.extend(values)?;
                Ok(self)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_paired_return_metric_lifecycle {
    ($state:ty) => {
        impl $state {
            pub fn from_returns(
                &mut self,
                primary: &[f64],
                benchmark: &[f64],
            ) -> $crate::MetricResult<&mut Self> {
                self.input.bind(
                    $crate::MetricInputKind::Returns,
                    $crate::MetricInputKind::Returns,
                )?;
                self.extend(primary, benchmark)?;
                Ok(self)
            }
            pub fn from_log_returns(
                &mut self,
                primary: &[f64],
                benchmark: &[f64],
            ) -> $crate::MetricResult<&mut Self> {
                self.input.bind(
                    $crate::MetricInputKind::LogReturns,
                    $crate::MetricInputKind::LogReturns,
                )?;
                self.extend(primary, benchmark)?;
                Ok(self)
            }
            pub fn from_equity(
                &mut self,
                primary: &[f64],
                benchmark: &[f64],
            ) -> $crate::MetricResult<&mut Self> {
                self.input.bind(
                    $crate::MetricInputKind::Equity,
                    $crate::MetricInputKind::Equity,
                )?;
                self.extend(primary, benchmark)?;
                Ok(self)
            }
            pub fn from_pnl(
                &mut self,
                primary: &[f64],
                benchmark: &[f64],
                initial_capital: f64,
                benchmark_initial_capital: f64,
            ) -> $crate::MetricResult<&mut Self> {
                self.input.bind(
                    $crate::MetricInputKind::PeriodPnl {
                        initial_capital: initial_capital,
                    },
                    $crate::MetricInputKind::PeriodPnl {
                        initial_capital: benchmark_initial_capital,
                    },
                )?;
                self.extend(primary, benchmark)?;
                Ok(self)
            }
        }
    };
}
