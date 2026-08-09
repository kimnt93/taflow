use crate::error::TaResult;
use crate::stream::operator_states::*;
use crate::stream::*;

#[derive(Debug, Clone, Copy, PartialEq)]
/// Persistent Rust state or aligned output type for `KnowSureThingValue`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct KnowSureThingValue {
    pub kst: f64,
    pub signal: f64,
}

/// Stateful Know Sure Thing (bukosabino `ta` `trend.KSTIndicator`, theory:
/// Martin Pring). `kst = 100·(rocma1 + 2·rocma2 + 3·rocma3 + 4·rocma4)` where
/// each `rocma` is an SMA of the raw ROC ratio over its window; the signal is
/// an `nsig`-period mean of KST (pandas `min_periods=0` warm-up).
///
/// The package fills the ROC shift warm-up with the global close mean; taflow
/// instead leaves those bars NaN, so outputs match the reference exactly from
/// bar `roc4 + sma4 − 1` (KST) and `roc4 + sma4 + nsig − 2` (signal).
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `KnowSureThing`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct KnowSureThing {
    rocs: [KstRocSma; 4],
    nsig: usize,
    signal_state: RollingMeanMin0,
    value: Option<KnowSureThingValue>,
}

impl KnowSureThing {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(
        roc1: usize,
        roc2: usize,
        roc3: usize,
        roc4: usize,
        sma1: usize,
        sma2: usize,
        sma3: usize,
        sma4: usize,
        nsig: usize,
    ) -> TaResult<Self> {
        validate_period(roc1)?;
        validate_period(roc2)?;
        validate_period(roc3)?;
        validate_period(roc4)?;
        validate_period(sma1)?;
        validate_period(sma2)?;
        validate_period(sma3)?;
        validate_period(sma4)?;
        validate_period(nsig)?;
        Ok(Self {
            rocs: [
                KstRocSma::new(roc1, sma1)?,
                KstRocSma::new(roc2, sma2)?,
                KstRocSma::new(roc3, sma3)?,
                KstRocSma::new(roc4, sma4)?,
            ],
            nsig,
            signal_state: RollingMeanMin0::new(nsig)?,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, close: f64) -> KnowSureThingValue {
        let rocma1 = self.rocs[0].append(close).unwrap_or(f64::NAN);
        let rocma2 = self.rocs[1].append(close).unwrap_or(f64::NAN);
        let rocma3 = self.rocs[2].append(close).unwrap_or(f64::NAN);
        let rocma4 = self.rocs[3].append(close).unwrap_or(f64::NAN);
        let kst = 100.0 * (rocma1 + 2.0 * rocma2 + 3.0 * rocma3 + 4.0 * rocma4);
        let signal = self.signal_state.append(kst).unwrap_or(f64::NAN);
        let value = KnowSureThingValue { kst, signal };
        self.value = Some(value);
        value
    }

    /// Computes or updates `value` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn value(&self) -> Option<KnowSureThingValue> {
        self.value
    }

    /// Computes or updates `reset` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn reset(&mut self) {
        for roc in &mut self.rocs {
            roc.reset();
        }
        self.signal_state.reset();
        self.value = None;
    }

    /// Bulk kernel: once every ROC/SMA chain is warm, advances the four
    /// sliding-sum recurrences in one loop with the running sums held in
    /// locals while the rings advance in place. Bit-identical to per-bar
    /// [`Self::append`] in outputs and post-run streaming state.
    pub fn extend_slices_into(
        &mut self,
        close: &[f64],
        kst_out: &mut Vec<f64>,
        signal_out: &mut Vec<f64>,
    ) {
        kst_out.reserve(close.len());
        signal_out.reserve(close.len());
        let mut index = 0;
        // Warm-up prologue: per-bar appends until KST is non-NaN, which
        // implies every ROC window and every SMA window is full.
        while index < close.len() && self.value.map_or(true, |value| value.kst.is_nan()) {
            let value = self.append(close[index]);
            kst_out.push(value.kst);
            signal_out.push(value.signal);
            index += 1;
        }
        if index == close.len() {
            return;
        }

        let [chain1, chain2, chain3, chain4] = &mut self.rocs;
        let period1 = chain1.sma.period() as f64;
        let period2 = chain2.sma.period() as f64;
        let period3 = chain3.sma.period() as f64;
        let period4 = chain4.sma.period() as f64;
        let mut sum1 = chain1.sma.raw_sum();
        let mut sum2 = chain2.sma.raw_sum();
        let mut sum3 = chain3.sma.raw_sum();
        let mut sum4 = chain4.sma.raw_sum();
        let (mut rocma1, mut rocma2, mut rocma3, mut rocma4) =
            (f64::NAN, f64::NAN, f64::NAN, f64::NAN);
        let (mut kst, mut signal) = (f64::NAN, f64::NAN);
        for &close_value in &close[index..] {
            let previous1 = chain1
                .close_window
                .push(close_value)
                .expect("full ROC window");
            let ratio1 = (close_value - previous1) / previous1;
            let evicted1 = chain1
                .sma
                .window_mut()
                .push(ratio1)
                .expect("full SMA window");
            sum1 -= evicted1;
            sum1 += ratio1;
            rocma1 = sum1 / period1;

            let previous2 = chain2
                .close_window
                .push(close_value)
                .expect("full ROC window");
            let ratio2 = (close_value - previous2) / previous2;
            let evicted2 = chain2
                .sma
                .window_mut()
                .push(ratio2)
                .expect("full SMA window");
            sum2 -= evicted2;
            sum2 += ratio2;
            rocma2 = sum2 / period2;

            let previous3 = chain3
                .close_window
                .push(close_value)
                .expect("full ROC window");
            let ratio3 = (close_value - previous3) / previous3;
            let evicted3 = chain3
                .sma
                .window_mut()
                .push(ratio3)
                .expect("full SMA window");
            sum3 -= evicted3;
            sum3 += ratio3;
            rocma3 = sum3 / period3;

            let previous4 = chain4
                .close_window
                .push(close_value)
                .expect("full ROC window");
            let ratio4 = (close_value - previous4) / previous4;
            let evicted4 = chain4
                .sma
                .window_mut()
                .push(ratio4)
                .expect("full SMA window");
            sum4 -= evicted4;
            sum4 += ratio4;
            rocma4 = sum4 / period4;

            kst = 100.0 * (rocma1 + 2.0 * rocma2 + 3.0 * rocma3 + 4.0 * rocma4);
            signal = self.signal_state.append(kst).unwrap_or(f64::NAN);
            kst_out.push(kst);
            signal_out.push(signal);
        }

        chain1.sma.store_bulk_state(sum1, Some(rocma1));
        chain2.sma.store_bulk_state(sum2, Some(rocma2));
        chain3.sma.store_bulk_state(sum3, Some(rocma3));
        chain4.sma.store_bulk_state(sum4, Some(rocma4));
        self.value = Some(KnowSureThingValue { kst, signal });
    }
}
