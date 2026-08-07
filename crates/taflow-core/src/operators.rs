use std::collections::VecDeque;

use crate::error::{TaError, TaResult};
use crate::stream::Atr;

fn validate_period(timeperiod: usize) -> TaResult<()> {
    if timeperiod == 0 {
        return Err(TaError::InvalidParameter {
            name: "timeperiod",
            value: timeperiod.to_string(),
            reason: "must be >= 1",
        });
    }
    Ok(())
}

fn validate_quantile(quantile: f64) -> TaResult<()> {
    if !(0.0..=1.0).contains(&quantile) {
        return Err(TaError::InvalidParameter {
            name: "quantile",
            value: quantile.to_string(),
            reason: "must be between 0 and 1",
        });
    }
    Ok(())
}

/// Delay a series by `timeperiod` bars. Warm-up values are `NaN`.
pub fn lag(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    validate_period(timeperiod)?;
    let mut output = vec![f64::NAN; input.len()];
    for index in timeperiod..input.len() {
        output[index] = input[index - timeperiod];
    }
    Ok(output)
}

/// Natural-log return over `timeperiod` bars. Warm-up values are `NaN`.
pub fn log_return(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    validate_period(timeperiod)?;
    let mut output = vec![f64::NAN; input.len()];
    for index in timeperiod..input.len() {
        output[index] = (input[index] / input[index - timeperiod]).ln();
    }
    Ok(output)
}

/// Cumulative sum of a series.
pub fn cumsum(input: &[f64]) -> Vec<f64> {
    let mut total = 0.0;
    input.iter().map(|&value| { total += value; total }).collect()
}

/// Cumulative product of a series.
pub fn cumprod(input: &[f64]) -> Vec<f64> {
    let mut total = 1.0;
    input.iter().map(|&value| { total *= value; total }).collect()
}

pub fn cummax(input: &[f64]) -> Vec<f64> {
    let mut maximum = f64::NEG_INFINITY;
    input.iter().map(|&value| { maximum = maximum.max(value); maximum }).collect()
}

pub fn cummin(input: &[f64]) -> Vec<f64> {
    let mut minimum = f64::INFINITY;
    input.iter().map(|&value| { minimum = minimum.min(value); minimum }).collect()
}

pub fn drawdown(input: &[f64]) -> Vec<f64> {
    let mut maximum = f64::NEG_INFINITY;
    input.iter().map(|&value| {
        maximum = maximum.max(value);
        if maximum != 0.0 { value / maximum - 1.0 } else { 0.0 }
    }).collect()
}

pub fn rolling_sharpe(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    let mut state = RollingSharpe::new(timeperiod)?;
    Ok(input.iter().map(|&value| state.append(value).unwrap_or(f64::NAN)).collect())
}

pub fn rolling_sortino(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    let mut state = RollingSortino::new(timeperiod)?;
    Ok(input.iter().map(|&value| state.append(value).unwrap_or(f64::NAN)).collect())
}

pub fn rolling_calmar(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    let mut state = RollingCalmar::new(timeperiod)?;
    Ok(input.iter().map(|&value| state.append(value).unwrap_or(f64::NAN)).collect())
}

pub fn hma(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> { let mut state = Hma::new(timeperiod)?; Ok(input.iter().map(|&v| state.append(v).unwrap_or(f64::NAN)).collect()) }
pub fn vwma(price: &[f64], volume: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> { if price.len()!=volume.len(){return Err(TaError::LengthMismatch{expected:price.len(),got:volume.len()});} let mut state=Vwma::new(timeperiod)?;Ok(price.iter().zip(volume).map(|(&p,&v)|state.append(p,v).unwrap_or(f64::NAN)).collect()) }
pub fn zlema(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> { let mut state=Zlema::new(timeperiod)?;Ok(input.iter().map(|&v|state.append(v).unwrap_or(f64::NAN)).collect()) }
pub fn alma(input: &[f64], timeperiod: usize, offset: f64, sigma: f64) -> TaResult<Vec<f64>> { let mut state=Alma::new(timeperiod,offset,sigma)?;Ok(input.iter().map(|&v|state.append(v).unwrap_or(f64::NAN)).collect()) }

pub fn tsi(input: &[f64], fast: usize, slow: usize) -> TaResult<Vec<f64>> { let mut state=Tsi::new(fast,slow)?;Ok(input.iter().map(|&v|state.append(v).unwrap_or(f64::NAN)).collect()) }
pub fn awesome_oscillator(high: &[f64], low: &[f64], fast: usize, slow: usize) -> TaResult<Vec<f64>> { if high.len()!=low.len(){return Err(TaError::LengthMismatch{expected:high.len(),got:low.len()});}let mut state=AwesomeOscillator::new(fast,slow)?;Ok(high.iter().zip(low).map(|(&h,&l)|state.append(h,l).unwrap_or(f64::NAN)).collect()) }
pub fn fisher_transform(high: &[f64], low: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> { if high.len()!=low.len(){return Err(TaError::LengthMismatch{expected:high.len(),got:low.len()});}let mut state=FisherTransform::new(timeperiod)?;Ok(high.iter().zip(low).map(|(&h,&l)|state.append(h,l).unwrap_or(f64::NAN)).collect()) }
pub fn ulcer_index(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> { let mut state=UlcerIndex::new(timeperiod)?;Ok(input.iter().map(|&v|state.append(v).unwrap_or(f64::NAN)).collect()) }
pub fn chaikin_volatility(high: &[f64], low: &[f64], timeperiod: usize, roc_period: usize) -> TaResult<Vec<f64>> { if high.len()!=low.len(){return Err(TaError::LengthMismatch{expected:high.len(),got:low.len()});}let mut state=ChaikinVolatility::new(timeperiod,roc_period)?;Ok(high.iter().zip(low).map(|(&h,&l)|state.append(h,l).unwrap_or(f64::NAN)).collect()) }
pub fn vwap(high: &[f64], low: &[f64], close: &[f64], volume: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> { if high.len()!=low.len()||high.len()!=close.len()||high.len()!=volume.len(){return Err(TaError::LengthMismatch{expected:high.len(),got:low.len()});}let mut state=Vwap::new(timeperiod)?;Ok(high.iter().zip(low).zip(close).zip(volume).map(|(((&h,&l),&c),&v)|state.append(h,l,c,v).unwrap_or(f64::NAN)).collect()) }
pub fn force_index(close: &[f64], volume: &[f64]) -> TaResult<Vec<f64>> { if close.len()!=volume.len(){return Err(TaError::LengthMismatch{expected:close.len(),got:volume.len()});}let mut state=ForceIndex::new();Ok(close.iter().zip(volume).map(|(&c,&v)|state.append(c,v).unwrap_or(f64::NAN)).collect()) }
pub fn ease_of_movement(high: &[f64], low: &[f64], volume: &[f64]) -> TaResult<Vec<f64>> { if high.len()!=low.len()||high.len()!=volume.len(){return Err(TaError::LengthMismatch{expected:high.len(),got:low.len()});}let mut state=EaseOfMovement::new();Ok(high.iter().zip(low).zip(volume).map(|((&h,&l),&v)|state.append(h,l,v).unwrap_or(f64::NAN)).collect()) }

macro_rules! bar_relation_operator { ($name:ident,$predicate:expr)=>{#[derive(Debug,Clone)]pub struct $name{previous:Option<(f64,f64)>,value:Option<f64>}impl $name{pub fn new()->Self{Self{previous:None,value:None}}pub fn append(&mut self,high:f64,low:f64)->Option<f64>{self.value=self.previous.map(|(ph,pl)|if $predicate(high,low,ph,pl){1.0}else{0.0});self.previous=Some((high,low));self.value}pub fn value(&self)->Option<f64>{self.value}pub fn reset(&mut self){self.previous=None;self.value=None;}}impl Default for $name{fn default()->Self{Self::new()}}};}
bar_relation_operator!(HigherHigh, |h:f64,_l:f64,ph:f64,_pl:f64| h>ph);
bar_relation_operator!(LowerLow, |_h:f64,l:f64,_ph:f64,pl:f64| l<pl);
bar_relation_operator!(InsideBar, |h:f64,l:f64,ph:f64,pl:f64| h<ph&&l>pl);
bar_relation_operator!(OutsideBar, |h:f64,l:f64,ph:f64,pl:f64| h>ph&&l<pl);
bar_relation_operator!(GapUp, |_h:f64,l:f64,ph:f64,_pl:f64| l>ph);
bar_relation_operator!(GapDown, |h:f64,_l:f64,_ph:f64,pl:f64| h<pl);

#[derive(Debug, Clone)] pub struct BarsSince { count: Option<usize>, value: Option<f64> }
impl BarsSince { pub fn new()->Self{Self{count:None,value:None}}pub fn append(&mut self,condition:bool)->Option<f64>{self.count=Some(if condition{0}else{self.count.map_or(0,|v|v+1)});self.value=self.count.map(|v|v as f64);self.value}pub fn value(&self)->Option<f64>{self.value}pub fn reset(&mut self){self.count=None;self.value=None;}}
impl Default for BarsSince{fn default()->Self{Self::new()}}
#[derive(Debug, Clone)] pub struct ValueWhen { latest: Option<f64>, value: Option<f64> }
impl ValueWhen { pub fn new()->Self{Self{latest:None,value:None}}pub fn append(&mut self,condition:bool,input:f64)->Option<f64>{if condition{self.latest=Some(input);}self.value=self.latest;self.value}pub fn value(&self)->Option<f64>{self.value}pub fn reset(&mut self){self.latest=None;self.value=None;}}
impl Default for ValueWhen{fn default()->Self{Self::new()}}
macro_rules! since_extreme {($name:ident,$operation:expr)=>{#[derive(Debug,Clone)]pub struct $name{extreme:Option<f64>,value:Option<f64>}impl $name{pub fn new()->Self{Self{extreme:None,value:None}}pub fn append(&mut self,condition:bool,input:f64)->Option<f64>{self.extreme=Some(if condition{input}else{self.extreme.map_or(input,|v|$operation(v,input))});self.value=self.extreme;self.value}pub fn value(&self)->Option<f64>{self.value}pub fn reset(&mut self){self.extreme=None;self.value=None;}}impl Default for $name{fn default()->Self{Self::new()}}};}
since_extreme!(HighestSince,f64::max); since_extreme!(LowestSince,f64::min);
pub fn rising(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> { let mut state=Rising::new(timeperiod)?;Ok(input.iter().map(|&v|state.append(v).unwrap_or(f64::NAN)).collect()) }
pub fn falling(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> { let mut state=Falling::new(timeperiod)?;Ok(input.iter().map(|&v|state.append(v).unwrap_or(f64::NAN)).collect()) }

pub fn rolling_entropy(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    let mut state = RollingEntropy::new(timeperiod)?;
    Ok(input.iter().map(|&value| state.append(value).unwrap_or(f64::NAN)).collect())
}

pub fn rolling_autocorr(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    let mut state = RollingAutocorr::new(timeperiod)?;
    Ok(input.iter().map(|&value| state.append(value).unwrap_or(f64::NAN)).collect())
}

pub fn hurst(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    let mut state = Hurst::new(timeperiod)?;
    Ok(input.iter().map(|&value| state.append(value).unwrap_or(f64::NAN)).collect())
}

pub fn fractal_dimension(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    let mut state = Hurst::new(timeperiod)?;
    Ok(input.iter().map(|&value| state.append(value).map(|hurst| 2.0 - hurst).unwrap_or(f64::NAN)).collect())
}

pub fn rolling_alpha(input: &[f64], benchmark: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    if input.len() != benchmark.len() { return Err(TaError::LengthMismatch { expected: input.len(), got: benchmark.len() }); }
    let mut state = RollingAlpha::new(timeperiod)?;
    Ok(input.iter().zip(benchmark).map(|(&input, &benchmark)| state.append(input, benchmark).unwrap_or(f64::NAN)).collect())
}

pub fn rolling_information_ratio(input: &[f64], benchmark: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    if input.len() != benchmark.len() { return Err(TaError::LengthMismatch { expected: input.len(), got: benchmark.len() }); }
    let mut state = RollingInformationRatio::new(timeperiod)?;
    Ok(input.iter().zip(benchmark).map(|(&input, &benchmark)| state.append(input, benchmark).unwrap_or(f64::NAN)).collect())
}

#[derive(Debug, Clone)]
pub struct RollingAlpha { values: VecDeque<(f64, f64)>, period: usize, value: Option<f64> }
impl RollingAlpha {
    pub fn new(period: usize) -> TaResult<Self> { validate_period(period)?; Ok(Self { values: VecDeque::with_capacity(period), period, value: None }) }
    pub fn append(&mut self, input: f64, benchmark: f64) -> Option<f64> {
        if self.values.len() == self.period { self.values.pop_front(); }
        self.values.push_back((input, benchmark));
        self.value = (self.values.len() == self.period).then(|| {
            let n = self.period as f64;
            let mean_input = self.values.iter().map(|&(input, _)| input).sum::<f64>() / n;
            let mean_benchmark = self.values.iter().map(|&(_, benchmark)| benchmark).sum::<f64>() / n;
            let covariance = self.values.iter().map(|&(input, benchmark)| (input - mean_input) * (benchmark - mean_benchmark)).sum::<f64>();
            let variance = self.values.iter().map(|&(_, benchmark)| (benchmark - mean_benchmark).powi(2)).sum::<f64>();
            let beta = if variance > 0.0 { covariance / variance } else { 0.0 };
            mean_input - beta * mean_benchmark
        });
        self.value
    }
    pub fn value(&self) -> Option<f64> { self.value }
    pub fn reset(&mut self) { self.values.clear(); self.value = None; }
}

#[derive(Debug, Clone)]
pub struct RollingInformationRatio { values: VecDeque<f64>, period: usize, value: Option<f64> }
impl RollingInformationRatio {
    pub fn new(period: usize) -> TaResult<Self> { validate_period(period)?; Ok(Self { values: VecDeque::with_capacity(period), period, value: None }) }
    pub fn append(&mut self, input: f64, benchmark: f64) -> Option<f64> {
        if self.values.len() == self.period { self.values.pop_front(); }
        self.values.push_back(input - benchmark);
        self.value = (self.values.len() == self.period).then(|| {
            let n = self.period as f64;
            let mean = self.values.iter().sum::<f64>() / n;
            let variance = self.values.iter().map(|&value| (value - mean).powi(2)).sum::<f64>() / n;
            if variance > 0.0 { mean / variance.sqrt() } else { 0.0 }
        });
        self.value
    }
    pub fn value(&self) -> Option<f64> { self.value }
    pub fn reset(&mut self) { self.values.clear(); self.value = None; }
}

#[derive(Debug, Clone)]
pub struct Hurst { values: VecDeque<f64>, period: usize, value: Option<f64> }

impl Hurst {
    pub fn new(period: usize) -> TaResult<Self> {
        if period < 2 { return Err(TaError::InvalidParameter { name: "timeperiod", value: period.to_string(), reason: "must be >= 2" }); }
        Ok(Self { values: VecDeque::with_capacity(period), period, value: None })
    }
    pub fn append(&mut self, input: f64) -> Option<f64> {
        if self.values.len() == self.period { self.values.pop_front(); }
        self.values.push_back(input);
        self.value = (self.values.len() == self.period).then(|| {
            let n = self.period as f64;
            let mean = self.values.iter().sum::<f64>() / n;
            let mut cumulative = 0.0;
            let mut minimum = f64::INFINITY;
            let mut maximum = f64::NEG_INFINITY;
            for &value in &self.values { cumulative += value - mean; minimum = minimum.min(cumulative); maximum = maximum.max(cumulative); }
            let standard_deviation = (self.values.iter().map(|&value| (value - mean).powi(2)).sum::<f64>() / n).sqrt();
            let rescaled_range = (maximum - minimum) / standard_deviation;
            if rescaled_range > 0.0 { (rescaled_range.ln() / n.ln()).clamp(0.0, 1.0) } else { 0.5 }
        });
        self.value
    }
    pub fn value(&self) -> Option<f64> { self.value }
    pub fn reset(&mut self) { self.values.clear(); self.value = None; }
}

#[derive(Debug, Clone)]
pub struct RollingEntropy {
    values: VecDeque<f64>,
    period: usize,
    value: Option<f64>,
}

impl RollingEntropy {
    pub fn new(period: usize) -> TaResult<Self> {
        validate_period(period)?;
        Ok(Self { values: VecDeque::with_capacity(period), period, value: None })
    }

    /// Shannon entropy of exact-value frequencies in the rolling window.
    pub fn append(&mut self, input: f64) -> Option<f64> {
        if self.values.len() == self.period { self.values.pop_front(); }
        self.values.push_back(input);
        self.value = (self.values.len() == self.period).then(|| {
            let n = self.period as f64;
            let mut entropy = 0.0;
            let mut seen = Vec::new();
            for &candidate in &self.values {
                if seen.contains(&candidate) { continue; }
                seen.push(candidate);
                let count = self.values.iter().filter(|&&value| value == candidate).count();
                let probability = count as f64 / n;
                entropy -= probability * probability.ln();
            }
            entropy
        });
        self.value
    }

    pub fn value(&self) -> Option<f64> { self.value }
    pub fn reset(&mut self) { self.values.clear(); self.value = None; }
}

#[derive(Debug, Clone)]
pub struct RollingAutocorr {
    values: VecDeque<f64>,
    period: usize,
    value: Option<f64>,
}

impl RollingAutocorr {
    pub fn new(period: usize) -> TaResult<Self> {
        if period < 2 {
            return Err(TaError::InvalidParameter { name: "timeperiod", value: period.to_string(), reason: "must be >= 2" });
        }
        Ok(Self { values: VecDeque::with_capacity(period), period, value: None })
    }

    /// Lag-one Pearson autocorrelation over the rolling window.
    pub fn append(&mut self, input: f64) -> Option<f64> {
        if self.values.len() == self.period { self.values.pop_front(); }
        self.values.push_back(input);
        self.value = (self.values.len() == self.period).then(|| {
            let n = self.period as f64;
            let left_n = (self.period - 1) as f64;
            let left_mean = self.values.iter().take(self.period - 1).sum::<f64>() / left_n;
            let right_mean = self.values.iter().skip(1).sum::<f64>() / left_n;
            let left_variance = self.values.iter().take(self.period - 1)
                .map(|&value| (value - left_mean).powi(2)).sum::<f64>();
            let right_variance = self.values.iter().skip(1)
                .map(|&value| (value - right_mean).powi(2)).sum::<f64>();
            if left_variance == 0.0 || right_variance == 0.0 { return 0.0; }
            let covariance = self.values.iter().take(self.period - 1)
                .zip(self.values.iter().skip(1))
                .map(|(&left, &right)| (left - left_mean) * (right - right_mean)).sum::<f64>();
            covariance / (left_variance * right_variance).sqrt()
        });
        self.value
    }

    pub fn value(&self) -> Option<f64> { self.value }
    pub fn reset(&mut self) { self.values.clear(); self.value = None; }
}

/// Rolling OLS slope of price-level `y` on price-level `x`.
pub fn hedge_ratio(x: &[f64], y: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    if x.len() != y.len() {
        return Err(TaError::LengthMismatch { expected: x.len(), got: y.len() });
    }
    let mut state = HedgeRatio::new(timeperiod)?;
    Ok(x.iter().zip(y).map(|(&x, &y)| state.append(x, y).unwrap_or(f64::NAN)).collect())
}

#[derive(Debug, Clone)]
pub struct HedgeRatio {
    values: VecDeque<(f64, f64)>,
    period: usize,
    value: Option<f64>,
}

impl HedgeRatio {
    pub fn new(period: usize) -> TaResult<Self> {
        validate_period(period)?;
        Ok(Self { values: VecDeque::with_capacity(period), period, value: None })
    }

    pub fn append(&mut self, x: f64, y: f64) -> Option<f64> {
        if self.values.len() == self.period {
            self.values.pop_front();
        }
        self.values.push_back((x, y));
        self.value = if self.values.len() == self.period {
            let n = self.period as f64;
            let mean_x = self.values.iter().map(|&(x, _)| x).sum::<f64>() / n;
            let mean_y = self.values.iter().map(|&(_, y)| y).sum::<f64>() / n;
            let covariance = self.values.iter().map(|&(x, y)| (x - mean_x) * (y - mean_y)).sum::<f64>();
            let variance = self.values.iter().map(|&(x, _)| (x - mean_x).powi(2)).sum::<f64>();
            Some(if variance > 0.0 { covariance / variance } else { 0.0 })
        } else {
            None
        };
        self.value
    }

    pub fn value(&self) -> Option<f64> { self.value }

    pub fn reset(&mut self) {
        self.values.clear();
        self.value = None;
    }
}

/// Running high and low values reset by an explicit session boundary.
///
/// The boundary is supplied as an aligned boolean input. The first bar is
/// treated as the beginning of a session when `new_session` is false.
pub fn session_extrema(
    new_session: &[bool],
    high: &[f64],
    low: &[f64],
) -> TaResult<(Vec<f64>, Vec<f64>)> {
    if new_session.len() != high.len() || high.len() != low.len() {
        return Err(TaError::LengthMismatch {
            expected: new_session.len(),
            got: high.len().max(low.len()),
        });
    }
    let mut state = SessionExtrema::new();
    let mut session_high = Vec::with_capacity(high.len());
    let mut session_low = Vec::with_capacity(low.len());
    for ((&new_session, &high), &low) in new_session.iter().zip(high).zip(low) {
        let value = state.append(new_session, high, low);
        session_high.push(value.high);
        session_low.push(value.low);
    }
    Ok((session_high, session_low))
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SessionExtremaValue {
    pub high: f64,
    pub low: f64,
}

#[derive(Debug, Clone, Default)]
pub struct SessionExtrema {
    high: Option<f64>,
    low: Option<f64>,
    value: Option<SessionExtremaValue>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Zone {
    pub top: f64,
    pub bottom: f64,
    pub birth: usize,
    pub flags: u32,
}

/// Bounded active-zone storage for causal zone-based indicators.
#[derive(Debug, Clone)]
pub struct ActiveZoneList {
    zones: Vec<Zone>,
    capacity: usize,
    index: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FvgValue {
    pub signal: f64,
    pub top: f64,
    pub bottom: f64,
    pub mitigated: f64,
}

#[derive(Debug, Clone, Copy)]
struct FvgZone {
    direction: f64,
    top: f64,
    bottom: f64,
}

/// Causal fair-value-gap detection with directional mitigation events.
#[derive(Debug, Clone, Default)]
pub struct Fvg {
    bars: VecDeque<(f64, f64, f64, f64)>,
    zones: Vec<FvgZone>,
    value: Option<FvgValue>,
}

impl Fvg {
    pub fn new() -> Self { Self::default() }

    pub fn append(&mut self, open: f64, high: f64, low: f64, close: f64) -> Option<FvgValue> {
        let previous = self.bars.back().copied();
        let two_back = self.bars.front().copied();
        let mut signal = f64::NAN;
        let mut top = f64::NAN;
        let mut bottom = f64::NAN;
        if let (Some((middle_open, _, _, middle_close)), Some((_, old_high, old_low, _))) =
            (previous, two_back)
        {
            if old_high < low && middle_close > middle_open {
                signal = 1.0;
                top = low;
                bottom = old_high;
                self.zones.push(FvgZone { direction: signal, top, bottom });
            } else if old_low > high && middle_close < middle_open {
                signal = -1.0;
                top = old_low;
                bottom = high;
                self.zones.push(FvgZone { direction: signal, top, bottom });
            }
        }
        let mut mitigated = f64::NAN;
        self.zones.retain(|zone| {
            let filled = (zone.direction > 0.0 && low <= zone.bottom)
                || (zone.direction < 0.0 && high >= zone.top);
            if filled {
                mitigated = zone.direction;
            }
            !filled
        });
        if signal.is_nan() && !mitigated.is_nan() {
            signal = f64::NAN;
        }
        if self.bars.len() == 2 {
            self.bars.pop_front();
        }
        self.bars.push_back((open, high, low, close));
        let value = FvgValue { signal, top, bottom, mitigated };
        self.value = Some(value);
        Some(value)
    }

    pub fn value(&self) -> Option<FvgValue> { self.value }

    pub fn reset(&mut self) {
        self.bars.clear();
        self.zones.clear();
        self.value = None;
    }
}

pub fn fvg(
    open: &[f64],
    high: &[f64],
    low: &[f64],
    close: &[f64],
) -> TaResult<(Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>)> {
    if open.len() != high.len() || high.len() != low.len() || low.len() != close.len() {
        return Err(TaError::LengthMismatch { expected: open.len(), got: high.len().max(low.len()).max(close.len()) });
    }
    let mut state = Fvg::new();
    let mut signal = Vec::with_capacity(open.len());
    let mut top = Vec::with_capacity(open.len());
    let mut bottom = Vec::with_capacity(open.len());
    let mut mitigated = Vec::with_capacity(open.len());
    for (((&open, &high), &low), &close) in open.iter().zip(high).zip(low).zip(close) {
        let value = state.append(open, high, low, close).expect("FVG always emits an aligned value");
        signal.push(value.signal);
        top.push(value.top);
        bottom.push(value.bottom);
        mitigated.push(value.mitigated);
    }
    Ok((signal, top, bottom, mitigated))
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BosChochValue {
    pub bos: f64,
    pub choch: f64,
    pub level: f64,
    pub broken: f64,
}

/// Causal break-of-structure and change-of-character events.
#[derive(Debug, Clone)]
pub struct BosChoch {
    swing: Swing,
    swings: VecDeque<(f64, f64)>,
    pending: Option<(f64, f64)>,
    trend: Option<f64>,
    value: Option<BosChochValue>,
}

impl BosChoch {
    pub fn new(swing_length: usize) -> TaResult<Self> {
        Ok(Self {
            swing: Swing::new(swing_length)?,
            swings: VecDeque::with_capacity(4),
            pending: None,
            trend: None,
            value: None,
        })
    }

    pub fn append(&mut self, high: f64, low: f64, close: f64) -> BosChochValue {
        let mut bos = f64::NAN;
        let mut choch = f64::NAN;
        let mut level = f64::NAN;
        let mut broken = f64::NAN;

        if let Some((direction, pending_level)) = self.pending {
            let crossed = (direction > 0.0 && close > pending_level)
                || (direction < 0.0 && close < pending_level);
            if crossed {
                broken = direction;
                level = pending_level;
                self.pending = None;
                self.trend = Some(direction);
            }
        }

        if let Some(swing) = self.swing.append(high, low) {
            self.swings.push_back((swing.signal, swing.level));
            if self.swings.len() > 4 {
                self.swings.pop_front();
            }
            if self.swings.len() == 4 {
                let items: Vec<_> = self.swings.iter().copied().collect();
                let bullish = items[0].0 < 0.0
                    && items[1].0 > 0.0
                    && items[2].0 < 0.0
                    && items[3].0 > 0.0
                    && items[0].1 < items[2].1
                    && items[1].1 < items[3].1;
                let bearish = items[0].0 > 0.0
                    && items[1].0 < 0.0
                    && items[2].0 > 0.0
                    && items[3].0 < 0.0
                    && items[0].1 > items[2].1
                    && items[1].1 > items[3].1;
                let direction = if bullish {
                    Some(1.0)
                } else if bearish {
                    Some(-1.0)
                } else {
                    None
                };
                if let Some(direction) = direction {
                    bos = direction;
                    choch = if self.trend.is_some_and(|trend| trend != direction) {
                        direction
                    } else {
                        f64::NAN
                    };
                    level = items[1].1;
                    self.pending = Some((direction, level));
                }
            }
        }

        let value = BosChochValue { bos, choch, level, broken };
        self.value = Some(value);
        value
    }

    pub fn value(&self) -> Option<BosChochValue> { self.value }

    pub fn reset(&mut self) {
        self.swing.reset();
        self.swings.clear();
        self.pending = None;
        self.trend = None;
        self.value = None;
    }
}

pub fn bos_choch(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    swing_length: usize,
) -> TaResult<(Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>)> {
    if high.len() != low.len() || low.len() != close.len() {
        return Err(TaError::LengthMismatch {
            expected: high.len(),
            got: low.len().max(close.len()),
        });
    }
    let mut state = BosChoch::new(swing_length)?;
    let mut bos = Vec::with_capacity(high.len());
    let mut choch = Vec::with_capacity(high.len());
    let mut level = Vec::with_capacity(high.len());
    let mut broken = Vec::with_capacity(high.len());
    for ((&high, &low), &close) in high.iter().zip(low).zip(close) {
        let value = state.append(high, low, close);
        bos.push(value.bos);
        choch.push(value.choch);
        level.push(value.level);
        broken.push(value.broken);
    }
    Ok((bos, choch, level, broken))
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ObValue {
    pub ob: f64,
    pub top: f64,
    pub bottom: f64,
    pub ob_volume: f64,
    pub mitigated: f64,
}

#[derive(Debug, Clone, Copy)]
struct ObZone {
    direction: f64,
    top: f64,
    bottom: f64,
}

/// Causal order-block detection with volatile-bar exclusion and directional
/// mitigation. Dual pivot scales: `swing_length` locates the structure
/// interval, `internal_length` locates the extreme block within it. Bars
/// whose range is at least `threshold * ATR(atr_period)` are excluded from
/// being order blocks.
#[derive(Debug, Clone)]
pub struct Ob {
    atr: Atr,
    internal: Swing,
    structure: Swing,
    internal_low: Option<(f64, f64, bool)>,
    internal_high: Option<(f64, f64, bool)>,
    structure_low: Option<f64>,
    structure_high: Option<f64>,
    threshold: f64,
    zones: Vec<ObZone>,
    value: Option<ObValue>,
}

impl Ob {
    pub fn new(
        swing_length: usize,
        internal_length: usize,
        atr_period: usize,
        threshold: f64,
    ) -> TaResult<Self> {
        validate_period(swing_length)?;
        validate_period(internal_length)?;
        if atr_period == 0 {
            return Err(TaError::InvalidParameter {
                name: "atr_period",
                value: atr_period.to_string(),
                reason: "must be >= 1",
            });
        }
        if threshold < 0.0 {
            return Err(TaError::InvalidParameter {
                name: "threshold",
                value: threshold.to_string(),
                reason: "must be >= 0",
            });
        }
        Ok(Self {
            atr: Atr::new(atr_period)?,
            internal: Swing::new(internal_length)?,
            structure: Swing::new(swing_length)?,
            internal_low: None,
            internal_high: None,
            structure_low: None,
            structure_high: None,
            threshold,
            zones: Vec::new(),
            value: None,
        })
    }

    pub fn append(&mut self, high: f64, low: f64, close: f64, volume: f64) -> ObValue {
        let atr = self.atr.append(high, low, close);
        let volatile = atr.is_some_and(|atr| high - low >= self.threshold * atr);

        let mut ob = f64::NAN;
        let mut top = f64::NAN;
        let mut bottom = f64::NAN;
        let mut ob_volume = f64::NAN;

        if let Some(internal_swing) = self.internal.append(high, low) {
            match internal_swing.signal {
                signal if signal > 0.0 => {
                    self.internal_high = Some((internal_swing.level, volume, volatile));
                    if let Some(structure_high) = self.structure_high {
                        if internal_swing.level > structure_high
                            && self.internal_low.is_some_and(|(_, _, volatile)| !volatile)
                        {
                            let (low_level, low_volume, _) =
                                self.internal_low.expect("internal low is set");
                            ob = 1.0;
                            top = internal_swing.level;
                            bottom = low_level;
                            ob_volume = low_volume;
                            self.zones.push(ObZone {
                                direction: ob,
                                top,
                                bottom,
                            });
                            self.structure_high = Some(internal_swing.level);
                        }
                    }
                }
                signal if signal < 0.0 => {
                    self.internal_low = Some((internal_swing.level, volume, volatile));
                    if let Some(structure_low) = self.structure_low {
                        if internal_swing.level < structure_low
                            && self.internal_high.is_some_and(|(_, _, volatile)| !volatile)
                        {
                            let (high_level, high_volume, _) =
                                self.internal_high.expect("internal high is set");
                            ob = -1.0;
                            top = high_level;
                            bottom = internal_swing.level;
                            ob_volume = high_volume;
                            self.zones.push(ObZone {
                                direction: ob,
                                top,
                                bottom,
                            });
                            self.structure_low = Some(internal_swing.level);
                        }
                    }
                }
                _ => {}
            }
        }

        if let Some(structure_swing) = self.structure.append(high, low) {
            match structure_swing.signal {
                signal if signal > 0.0 => self.structure_high = Some(structure_swing.level),
                signal if signal < 0.0 => self.structure_low = Some(structure_swing.level),
                _ => {}
            }
        }

        let mut mitigated = f64::NAN;
        self.zones.retain(|zone| {
            let filled = (zone.direction > 0.0 && low <= zone.bottom)
                || (zone.direction < 0.0 && high >= zone.top);
            if filled {
                mitigated = zone.direction;
            }
            !filled
        });

        let value = ObValue { ob, top, bottom, ob_volume, mitigated };
        self.value = Some(value);
        value
    }

    pub fn value(&self) -> Option<ObValue> { self.value }

    pub fn reset(&mut self) {
        self.atr.reset();
        self.internal.reset();
        self.structure.reset();
        self.internal_low = None;
        self.internal_high = None;
        self.structure_low = None;
        self.structure_high = None;
        self.zones.clear();
        self.value = None;
    }
}

pub fn ob(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    volume: &[f64],
    swing_length: usize,
    internal_length: usize,
    atr_period: usize,
    threshold: f64,
) -> TaResult<(Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>)> {
    if high.len() != low.len() || low.len() != close.len() || close.len() != volume.len() {
        return Err(TaError::LengthMismatch {
            expected: high.len(),
            got: low.len().max(close.len()).max(volume.len()),
        });
    }
    let mut state = Ob::new(swing_length, internal_length, atr_period, threshold)?;
    let mut ob_out = Vec::with_capacity(high.len());
    let mut top = Vec::with_capacity(high.len());
    let mut bottom = Vec::with_capacity(high.len());
    let mut ob_volume = Vec::with_capacity(high.len());
    let mut mitigated = Vec::with_capacity(high.len());
    for ((((&high, &low), &close), &volume), _) in high
        .iter()
        .zip(low)
        .zip(close)
        .zip(volume)
        .zip(std::iter::repeat(()))
    {
        let value = state.append(high, low, close, volume);
        ob_out.push(value.ob);
        top.push(value.top);
        bottom.push(value.bottom);
        ob_volume.push(value.ob_volume);
        mitigated.push(value.mitigated);
    }
    Ok((ob_out, top, bottom, ob_volume, mitigated))
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LiquidityValue {
    pub liquidity: f64,
    pub level: f64,
    pub swept: f64,
}

#[derive(Debug, Clone, Copy)]
struct LiquidityPool {
    level: f64,
    count: usize,
}

/// Causal liquidity-pool clustering with sweep detection. Swing highs and
/// lows are clustered into pools when they fall within a `range_percent`
/// price tolerance; a pool emits a signal once a second swing confirms it.
/// A pool is swept and removed when price trades beyond its level.
#[derive(Debug, Clone)]
pub struct Liquidity {
    swing: Swing,
    high_pools: Vec<LiquidityPool>,
    low_pools: Vec<LiquidityPool>,
    range_percent: f64,
    value: Option<LiquidityValue>,
}

impl Liquidity {
    pub fn new(swing_length: usize, range_percent: f64) -> TaResult<Self> {
        validate_period(swing_length)?;
        if !(0.0..=1.0).contains(&range_percent) {
            return Err(TaError::InvalidParameter {
                name: "range_percent",
                value: range_percent.to_string(),
                reason: "must be between 0 and 1",
            });
        }
        Ok(Self {
            swing: Swing::new(swing_length)?,
            high_pools: Vec::new(),
            low_pools: Vec::new(),
            range_percent,
            value: None,
        })
    }

    fn nearest_pool(pools: &mut Vec<LiquidityPool>, level: f64, range_percent: f64) -> Option<usize> {
        let mut best: Option<(usize, f64)> = None;
        for (index, pool) in pools.iter().enumerate() {
            let distance = (pool.level - level).abs();
            if distance <= range_percent * pool.level
                && best.map_or(true, |(_, best_distance)| distance < best_distance)
            {
                best = Some((index, distance));
            }
        }
        best.map(|(index, _)| index)
    }

    pub fn append(&mut self, high: f64, low: f64, _close: f64) -> LiquidityValue {
        let mut liquidity = f64::NAN;
        let mut level = f64::NAN;
        let mut swept = f64::NAN;

        if let Some(swing) = self.swing.append(high, low) {
            if swing.signal > 0.0 {
                if let Some(index) = Self::nearest_pool(&mut self.high_pools, swing.level, self.range_percent) {
                    let pool = &mut self.high_pools[index];
                    pool.level = pool.level.max(swing.level);
                    pool.count += 1;
                    if pool.count >= 2 {
                        liquidity = 1.0;
                        level = pool.level;
                    }
                } else {
                    self.high_pools.push(LiquidityPool { level: swing.level, count: 1 });
                }
            } else if swing.signal < 0.0 {
                if let Some(index) = Self::nearest_pool(&mut self.low_pools, swing.level, self.range_percent) {
                    let pool = &mut self.low_pools[index];
                    pool.level = pool.level.min(swing.level);
                    pool.count += 1;
                    if pool.count >= 2 {
                        liquidity = -1.0;
                        level = pool.level;
                    }
                } else {
                    self.low_pools.push(LiquidityPool { level: swing.level, count: 1 });
                }
            }
        }

        self.high_pools.retain(|pool| {
            let swept_pool = pool.count >= 2 && high >= pool.level;
            if swept_pool {
                swept = 1.0;
                level = pool.level;
            }
            !swept_pool
        });
        self.low_pools.retain(|pool| {
            let swept_pool = pool.count >= 2 && low <= pool.level;
            if swept_pool {
                swept = -1.0;
                level = pool.level;
            }
            !swept_pool
        });

        let value = LiquidityValue { liquidity, level, swept };
        self.value = Some(value);
        value
    }

    pub fn value(&self) -> Option<LiquidityValue> { self.value }

    pub fn reset(&mut self) {
        self.swing.reset();
        self.high_pools.clear();
        self.low_pools.clear();
        self.value = None;
    }
}

pub fn liquidity(
    high: &[f64],
    low: &[f64],
    swing_length: usize,
    range_percent: f64,
) -> TaResult<(Vec<f64>, Vec<f64>, Vec<f64>)> {
    if high.len() != low.len() {
        return Err(TaError::LengthMismatch { expected: high.len(), got: low.len() });
    }
    let mut state = Liquidity::new(swing_length, range_percent)?;
    let mut liquidity_out = Vec::with_capacity(high.len());
    let mut level = Vec::with_capacity(high.len());
    let mut swept = Vec::with_capacity(high.len());
    for (&high, &low) in high.iter().zip(low) {
        let value = state.append(high, low, f64::NAN);
        liquidity_out.push(value.liquidity);
        level.push(value.level);
        swept.push(value.swept);
    }
    Ok((liquidity_out, level, swept))
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EqualHighsLowsValue {
    pub eqh: f64,
    pub eql: f64,
    pub level: f64,
}

/// Causal equal-high/equal-low detection. Two consecutive confirmed pivots
/// of the same kind are "equal" when their levels differ by less than
/// `eq_threshold * ATR(atr_period)`, matching the LuxAlgo Pine variant.
#[derive(Debug, Clone)]
pub struct EqualHighsLows {
    atr: Atr,
    swing: Swing,
    previous_high: Option<f64>,
    previous_low: Option<f64>,
    eq_threshold: f64,
    value: Option<EqualHighsLowsValue>,
}

impl EqualHighsLows {
    pub fn new(eq_len: usize, atr_period: usize, eq_threshold: f64) -> TaResult<Self> {
        validate_period(eq_len)?;
        if atr_period == 0 {
            return Err(TaError::InvalidParameter {
                name: "atr_period",
                value: atr_period.to_string(),
                reason: "must be >= 1",
            });
        }
        if !(0.0..=1.0).contains(&eq_threshold) {
            return Err(TaError::InvalidParameter {
                name: "eq_threshold",
                value: eq_threshold.to_string(),
                reason: "must be between 0 and 1",
            });
        }
        Ok(Self {
            atr: Atr::new(atr_period)?,
            swing: Swing::new(eq_len)?,
            previous_high: None,
            previous_low: None,
            eq_threshold,
            value: None,
        })
    }

    pub fn append(&mut self, high: f64, low: f64, close: f64) -> EqualHighsLowsValue {
        let atr = self.atr.append(high, low, close);
        let mut eqh = f64::NAN;
        let mut eql = f64::NAN;
        let mut level = f64::NAN;

        if let Some(swing) = self.swing.append(high, low) {
            if swing.signal > 0.0 {
                if let (Some(previous), Some(atr)) = (self.previous_high, atr) {
                    if (swing.level - previous).abs() < atr * self.eq_threshold {
                        eqh = 1.0;
                        level = swing.level;
                    }
                }
                self.previous_high = Some(swing.level);
            } else if swing.signal < 0.0 {
                if let (Some(previous), Some(atr)) = (self.previous_low, atr) {
                    if (swing.level - previous).abs() < atr * self.eq_threshold {
                        eql = 1.0;
                        level = swing.level;
                    }
                }
                self.previous_low = Some(swing.level);
            }
        }

        let value = EqualHighsLowsValue { eqh, eql, level };
        self.value = Some(value);
        value
    }

    pub fn value(&self) -> Option<EqualHighsLowsValue> { self.value }

    pub fn reset(&mut self) {
        self.atr.reset();
        self.swing.reset();
        self.previous_high = None;
        self.previous_low = None;
        self.value = None;
    }
}

pub fn equal_highs_lows(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    eq_len: usize,
    atr_period: usize,
    eq_threshold: f64,
) -> TaResult<(Vec<f64>, Vec<f64>, Vec<f64>)> {
    if high.len() != low.len() || low.len() != close.len() {
        return Err(TaError::LengthMismatch {
            expected: high.len(),
            got: low.len().max(close.len()),
        });
    }
    let mut state = EqualHighsLows::new(eq_len, atr_period, eq_threshold)?;
    let mut eqh = Vec::with_capacity(high.len());
    let mut eql = Vec::with_capacity(high.len());
    let mut level = Vec::with_capacity(high.len());
    for ((&high, &low), &close) in high.iter().zip(low).zip(close) {
        let value = state.append(high, low, close);
        eqh.push(value.eqh);
        eql.push(value.eql);
        level.push(value.level);
    }
    Ok((eqh, eql, level))
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PreviousHighLowValue {
    pub prev_high: f64,
    pub prev_low: f64,
    pub broken_high: f64,
    pub broken_low: f64,
}

/// Causal prior-higher-timeframe high/low tracking with break flags. Given a
/// HTF boundary flag series, running extrema are snapshotted into
/// `prev_high`/`prev_low` at each boundary; breaks are flagged when the
/// current bar trades beyond the previous HTF bar's extrema.
#[derive(Debug, Clone)]
pub struct PreviousHighLow {
    running_high: Option<f64>,
    running_low: Option<f64>,
    previous_high: Option<f64>,
    previous_low: Option<f64>,
    value: Option<PreviousHighLowValue>,
}

impl PreviousHighLow {
    pub fn new() -> Self {
        Self {
            running_high: None,
            running_low: None,
            previous_high: None,
            previous_low: None,
            value: None,
        }
    }

    pub fn append(&mut self, new_session: bool, high: f64, low: f64) -> PreviousHighLowValue {
        if new_session {
            if self.running_high.is_some() {
                self.previous_high = self.running_high;
                self.previous_low = self.running_low;
            }
            self.running_high = Some(high);
            self.running_low = Some(low);
        } else {
            self.running_high = Some(self.running_high.map_or(high, |running| running.max(high)));
            self.running_low = Some(self.running_low.map_or(low, |running| running.min(low)));
        }

        let broken_high = self
            .previous_high
            .map_or(f64::NAN, |previous| if high > previous { 1.0 } else { f64::NAN });
        let broken_low = self
            .previous_low
            .map_or(f64::NAN, |previous| if low < previous { 1.0 } else { f64::NAN });

        let value = PreviousHighLowValue {
            prev_high: self.previous_high.unwrap_or(f64::NAN),
            prev_low: self.previous_low.unwrap_or(f64::NAN),
            broken_high,
            broken_low,
        };
        self.value = Some(value);
        value
    }

    pub fn value(&self) -> Option<PreviousHighLowValue> { self.value }

    pub fn reset(&mut self) {
        self.running_high = None;
        self.running_low = None;
        self.previous_high = None;
        self.previous_low = None;
        self.value = None;
    }
}

impl Default for PreviousHighLow {
    fn default() -> Self {
        Self::new()
    }
}

pub fn previous_high_low(
    new_session: &[bool],
    high: &[f64],
    low: &[f64],
) -> TaResult<(Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>)> {
    if new_session.len() != high.len() || high.len() != low.len() {
        return Err(TaError::LengthMismatch {
            expected: new_session.len(),
            got: high.len().max(low.len()),
        });
    }
    let mut state = PreviousHighLow::new();
    let mut prev_high = Vec::with_capacity(high.len());
    let mut prev_low = Vec::with_capacity(high.len());
    let mut broken_high = Vec::with_capacity(high.len());
    let mut broken_low = Vec::with_capacity(high.len());
    for ((&new_session, &high), &low) in new_session.iter().zip(high).zip(low) {
        let value = state.append(new_session, high, low);
        prev_high.push(value.prev_high);
        prev_low.push(value.prev_low);
        broken_high.push(value.broken_high);
        broken_low.push(value.broken_low);
    }
    Ok((prev_high, prev_low, broken_high, broken_low))
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SessionsValue {
    pub active: f64,
    pub session_high: f64,
    pub session_low: f64,
}

/// Causal session-scoped extrema. Given a session-boundary flag series,
/// emits a constant `active` marker and the running high/low since the last
/// boundary — matching the package's causal running extrema.
#[derive(Debug, Clone)]
pub struct Sessions {
    session_high: Option<f64>,
    session_low: Option<f64>,
    started: bool,
    value: Option<SessionsValue>,
}

impl Sessions {
    pub fn new() -> Self {
        Self { session_high: None, session_low: None, started: false, value: None }
    }

    pub fn append(&mut self, new_session: bool, high: f64, low: f64) -> SessionsValue {
        if new_session || !self.started {
            self.session_high = Some(high);
            self.session_low = Some(low);
            self.started = true;
        } else {
            self.session_high = Some(self.session_high.map_or(high, |running| running.max(high)));
            self.session_low = Some(self.session_low.map_or(low, |running| running.min(low)));
        }
        let value = SessionsValue {
            active: 1.0,
            session_high: self.session_high.unwrap_or(f64::NAN),
            session_low: self.session_low.unwrap_or(f64::NAN),
        };
        self.value = Some(value);
        value
    }

    pub fn value(&self) -> Option<SessionsValue> { self.value }

    pub fn reset(&mut self) {
        self.session_high = None;
        self.session_low = None;
        self.started = false;
        self.value = None;
    }
}

impl Default for Sessions {
    fn default() -> Self {
        Self::new()
    }
}

pub fn sessions(
    new_session: &[bool],
    high: &[f64],
    low: &[f64],
) -> TaResult<(Vec<f64>, Vec<f64>, Vec<f64>)> {
    if new_session.len() != high.len() || high.len() != low.len() {
        return Err(TaError::LengthMismatch {
            expected: new_session.len(),
            got: high.len().max(low.len()),
        });
    }
    let mut state = Sessions::new();
    let mut active = Vec::with_capacity(high.len());
    let mut session_high = Vec::with_capacity(high.len());
    let mut session_low = Vec::with_capacity(high.len());
    for ((&new_session, &high), &low) in new_session.iter().zip(high).zip(low) {
        let value = state.append(new_session, high, low);
        active.push(value.active);
        session_high.push(value.session_high);
        session_low.push(value.session_low);
    }
    Ok((active, session_high, session_low))
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RetracementsValue {
    pub direction: f64,
    pub current_retracement_pct: f64,
    pub deepest_retracement_pct: f64,
}

/// Causal swing-leg retracement tracking. On each confirmed swing a leg is
/// established from the opposite prior pivot; the retracement percentage is
/// the fraction of that leg already given back by the current close, with
/// the deepest value tracked since the leg began.
#[derive(Debug, Clone)]
pub struct Retracements {
    swing: Swing,
    last_high: Option<f64>,
    last_low: Option<f64>,
    leg_high: Option<f64>,
    leg_low: Option<f64>,
    direction: Option<f64>,
    deepest: f64,
    value: Option<RetracementsValue>,
}

impl Retracements {
    pub fn new(swing_length: usize) -> TaResult<Self> {
        Ok(Self {
            swing: Swing::new(swing_length)?,
            last_high: None,
            last_low: None,
            leg_high: None,
            leg_low: None,
            direction: None,
            deepest: 0.0,
            value: None,
        })
    }

    pub fn append(&mut self, high: f64, low: f64, close: f64) -> RetracementsValue {
        if let Some(swing) = self.swing.append(high, low) {
            if swing.signal > 0.0 {
                self.last_high = Some(swing.level);
                if let Some(last_low) = self.last_low {
                    self.leg_high = Some(swing.level);
                    self.leg_low = Some(last_low);
                    self.direction = Some(1.0);
                    self.deepest = 0.0;
                }
            } else if swing.signal < 0.0 {
                self.last_low = Some(swing.level);
                if let Some(last_high) = self.last_high {
                    self.leg_high = Some(last_high);
                    self.leg_low = Some(swing.level);
                    self.direction = Some(-1.0);
                    self.deepest = 0.0;
                }
            }
        }

        let mut current_retracement_pct = f64::NAN;
        let mut deepest_retracement_pct = f64::NAN;
        if let (Some(leg_high), Some(leg_low), Some(direction)) = (self.leg_high, self.leg_low, self.direction) {
            let range = leg_high - leg_low;
            if range > 0.0 {
                let pct = if direction > 0.0 {
                    (leg_high - close) / range * 100.0
                } else {
                    (close - leg_low) / range * 100.0
                };
                current_retracement_pct = pct.max(0.0);
                self.deepest = self.deepest.max(current_retracement_pct);
                deepest_retracement_pct = self.deepest;
            }
        }

        let value = RetracementsValue {
            direction: self.direction.unwrap_or(f64::NAN),
            current_retracement_pct,
            deepest_retracement_pct,
        };
        self.value = Some(value);
        value
    }

    pub fn value(&self) -> Option<RetracementsValue> { self.value }

    pub fn reset(&mut self) {
        self.swing.reset();
        self.last_high = None;
        self.last_low = None;
        self.leg_high = None;
        self.leg_low = None;
        self.direction = None;
        self.deepest = 0.0;
        self.value = None;
    }
}

pub fn retracements(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    swing_length: usize,
) -> TaResult<(Vec<f64>, Vec<f64>, Vec<f64>)> {
    if high.len() != low.len() || low.len() != close.len() {
        return Err(TaError::LengthMismatch {
            expected: high.len(),
            got: low.len().max(close.len()),
        });
    }
    let mut state = Retracements::new(swing_length)?;
    let mut direction = Vec::with_capacity(high.len());
    let mut current_retracement_pct = Vec::with_capacity(high.len());
    let mut deepest_retracement_pct = Vec::with_capacity(high.len());
    for ((&high, &low), &close) in high.iter().zip(low).zip(close) {
        let value = state.append(high, low, close);
        direction.push(value.direction);
        current_retracement_pct.push(value.current_retracement_pct);
        deepest_retracement_pct.push(value.deepest_retracement_pct);
    }
    Ok((direction, current_retracement_pct, deepest_retracement_pct))
}

/// Rolling standard deviation of log returns (close-to-close volatility).
/// Warm-up values are `NaN`.
#[derive(Debug, Clone)]
pub struct CloseToCloseSigma {
    mean: RollingMean,
    squares: RollingMean,
    previous_close: Option<f64>,
    value: Option<f64>,
}

impl CloseToCloseSigma {
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        Ok(Self {
            mean: RollingMean::new(timeperiod)?,
            squares: RollingMean::new(timeperiod)?,
            previous_close: None,
            value: None,
        })
    }

    pub fn append(&mut self, close: f64) -> Option<f64> {
        if let Some(previous_close) = self.previous_close.replace(close) {
            if close > 0.0 && previous_close > 0.0 {
                let log_return = (close / previous_close).ln();
                let _ = self.mean.append(log_return);
                let _ = self.squares.append(log_return * log_return);
                self.value = match (self.mean.value(), self.squares.value()) {
                    (Some(mean), Some(squares)) => Some((squares - mean * mean).max(0.0).sqrt()),
                    _ => None,
                };
            }
        }
        self.value
    }

    pub fn value(&self) -> Option<f64> { self.value }

    pub fn reset(&mut self) {
        self.mean.reset();
        self.squares.reset();
        self.previous_close = None;
        self.value = None;
    }
}

pub fn close_to_close_sigma(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    let mut state = CloseToCloseSigma::new(timeperiod)?;
    Ok(input.iter().map(|&value| state.append(value).unwrap_or(f64::NAN)).collect())
}

/// Rolling mean of `ln(H/L)² / (4 ln 2)` (Parkinson volatility).
#[derive(Debug, Clone)]
pub struct Parkinson {
    mean: RollingMean,
    value: Option<f64>,
}

impl Parkinson {
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        Ok(Self { mean: RollingMean::new(timeperiod)?, value: None })
    }

    pub fn append(&mut self, high: f64, low: f64) -> Option<f64> {
        let term = if high > low && high > 0.0 && low > 0.0 {
            (high / low).ln().powi(2) / (4.0 * 2.0f64.ln())
        } else {
            0.0
        };
        self.value = self.mean.append(term).map(|mean| mean.sqrt());
        self.value
    }

    pub fn value(&self) -> Option<f64> { self.value }

    pub fn reset(&mut self) {
        self.mean.reset();
        self.value = None;
    }
}

pub fn parkinson(high: &[f64], low: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    if high.len() != low.len() {
        return Err(TaError::LengthMismatch { expected: high.len(), got: low.len() });
    }
    let mut state = Parkinson::new(timeperiod)?;
    Ok(high.iter().zip(low).map(|(&high, &low)| state.append(high, low).unwrap_or(f64::NAN)).collect())
}

/// Rolling mean of `0.5·ln(H/L)² − (2ln2−1)·ln(C/O)²` (Garman-Klass).
#[derive(Debug, Clone)]
pub struct GarmanKlass {
    mean: RollingMean,
    value: Option<f64>,
}

impl GarmanKlass {
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        Ok(Self { mean: RollingMean::new(timeperiod)?, value: None })
    }

    pub fn append(&mut self, open: f64, high: f64, low: f64, close: f64) -> Option<f64> {
        let term = if high > 0.0 && low > 0.0 && open > 0.0 && close > 0.0 {
            0.5 * (high / low).ln().powi(2) - (2.0 * 2.0f64.ln() - 1.0) * (close / open).ln().powi(2)
        } else {
            0.0
        };
        self.value = self.mean.append(term).map(|mean| mean.sqrt());
        self.value
    }

    pub fn value(&self) -> Option<f64> { self.value }

    pub fn reset(&mut self) {
        self.mean.reset();
        self.value = None;
    }
}

pub fn garman_klass(
    open: &[f64],
    high: &[f64],
    low: &[f64],
    close: &[f64],
    timeperiod: usize,
) -> TaResult<Vec<f64>> {
    if open.len() != high.len() || high.len() != low.len() || low.len() != close.len() {
        return Err(TaError::LengthMismatch {
            expected: open.len(),
            got: high.len().max(low.len()).max(close.len()),
        });
    }
    let mut state = GarmanKlass::new(timeperiod)?;
    Ok(open
        .iter()
        .zip(high)
        .zip(low)
        .zip(close)
        .map(|((( &open, &high), &low), &close)| state.append(open, high, low, close).unwrap_or(f64::NAN))
        .collect())
}

/// Rolling mean of `ln(H/C)ln(H/O) + ln(L/C)ln(L/O)` (Rogers-Satchell).
#[derive(Debug, Clone)]
pub struct RogersSatchell {
    mean: RollingMean,
    value: Option<f64>,
}

impl RogersSatchell {
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        Ok(Self { mean: RollingMean::new(timeperiod)?, value: None })
    }

    pub fn append(&mut self, open: f64, high: f64, low: f64, close: f64) -> Option<f64> {
        let term = if open > 0.0 && high > 0.0 && low > 0.0 && close > 0.0 {
            (high / close).ln() * (high / open).ln() + (low / close).ln() * (low / open).ln()
        } else {
            0.0
        };
        self.value = self.mean.append(term).map(|mean| mean.sqrt());
        self.value
    }

    pub fn value(&self) -> Option<f64> { self.value }

    pub fn reset(&mut self) {
        self.mean.reset();
        self.value = None;
    }
}

pub fn rogers_satchell(
    open: &[f64],
    high: &[f64],
    low: &[f64],
    close: &[f64],
    timeperiod: usize,
) -> TaResult<Vec<f64>> {
    if open.len() != high.len() || high.len() != low.len() || low.len() != close.len() {
        return Err(TaError::LengthMismatch {
            expected: open.len(),
            got: high.len().max(low.len()).max(close.len()),
        });
    }
    let mut state = RogersSatchell::new(timeperiod)?;
    Ok(open
        .iter()
        .zip(high)
        .zip(low)
        .zip(close)
        .map(|((( &open, &high), &low), &close)| state.append(open, high, low, close).unwrap_or(f64::NAN))
        .collect())
}

/// Garman-Klass with the overnight term `ln(O/C_prev)²` added (GK-Yang-Zhang).
#[derive(Debug, Clone)]
pub struct GkYangZhang {
    mean: RollingMean,
    previous_close: Option<f64>,
    value: Option<f64>,
}

impl GkYangZhang {
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        Ok(Self { mean: RollingMean::new(timeperiod)?, previous_close: None, value: None })
    }

    pub fn append(&mut self, open: f64, high: f64, low: f64, close: f64) -> Option<f64> {
        if let Some(previous_close) = self.previous_close.replace(close) {
            let term = if open > 0.0 && high > 0.0 && low > 0.0 && close > 0.0 && previous_close > 0.0 {
                let gk = 0.5 * (high / low).ln().powi(2)
                    - (2.0 * 2.0f64.ln() - 1.0) * (close / open).ln().powi(2);
                let overnight = (open / previous_close).ln().powi(2);
                gk + overnight
            } else {
                0.0
            };
            self.value = self.mean.append(term).map(|mean| mean.sqrt());
        }
        self.value
    }

    pub fn value(&self) -> Option<f64> { self.value }

    pub fn reset(&mut self) {
        self.mean.reset();
        self.previous_close = None;
        self.value = None;
    }
}

pub fn gk_yang_zhang(
    open: &[f64],
    high: &[f64],
    low: &[f64],
    close: &[f64],
    timeperiod: usize,
) -> TaResult<Vec<f64>> {
    if open.len() != high.len() || high.len() != low.len() || low.len() != close.len() {
        return Err(TaError::LengthMismatch {
            expected: open.len(),
            got: high.len().max(low.len()).max(close.len()),
        });
    }
    let mut state = GkYangZhang::new(timeperiod)?;
    Ok(open
        .iter()
        .zip(high)
        .zip(low)
        .zip(close)
        .map(|((( &open, &high), &low), &close)| state.append(open, high, low, close).unwrap_or(f64::NAN))
        .collect())
}

/// Yang-Zhang volatility: `σ² = σ²_on + k·σ²_oc + (1−k)·σ²_RS` with
/// `k = 0.34/(1.34 + (n+1)/(n−1))`. Highest-efficiency estimator.
#[derive(Debug, Clone)]
pub struct YangZhang {
    overnight: RollingMean,
    open_close: RollingMean,
    rs: RollingMean,
    timeperiod: usize,
    previous_close: Option<f64>,
    value: Option<f64>,
}

impl YangZhang {
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        validate_period(timeperiod)?;
        if timeperiod < 2 {
            return Err(TaError::InvalidParameter {
                name: "timeperiod",
                value: timeperiod.to_string(),
                reason: "must be >= 2 for Yang-Zhang",
            });
        }
        Ok(Self {
            overnight: RollingMean::new(timeperiod)?,
            open_close: RollingMean::new(timeperiod)?,
            rs: RollingMean::new(timeperiod)?,
            timeperiod,
            previous_close: None,
            value: None,
        })
    }

    pub fn append(&mut self, open: f64, high: f64, low: f64, close: f64) -> Option<f64> {
        let previous_close = self.previous_close.replace(close);
        if open > 0.0 && high > 0.0 && low > 0.0 && close > 0.0 {
            if let Some(previous_close) = previous_close {
                if previous_close > 0.0 {
                    let overnight = (open / previous_close).ln().powi(2);
                    let open_close = (close / open).ln().powi(2);
                    let rs = (high / close).ln() * (high / open).ln()
                        + (low / close).ln() * (low / open).ln();
                    let _ = self.overnight.append(overnight);
                    let _ = self.open_close.append(open_close);
                    let _ = self.rs.append(rs);
                }
            }
        }
        self.value = match (self.overnight.value(), self.open_close.value(), self.rs.value()) {
            (Some(on), Some(oc), Some(rs)) => {
                let n = self.timeperiod as f64;
                let k = 0.34 / (1.34 + (n + 1.0) / (n - 1.0));
                Some((on + k * oc + (1.0 - k) * rs).max(0.0).sqrt())
            }
            _ => None,
        };
        self.value
    }

    pub fn value(&self) -> Option<f64> { self.value }

    pub fn reset(&mut self) {
        self.overnight.reset();
        self.open_close.reset();
        self.rs.reset();
        self.previous_close = None;
        self.value = None;
    }
}

pub fn yang_zhang(
    open: &[f64],
    high: &[f64],
    low: &[f64],
    close: &[f64],
    timeperiod: usize,
) -> TaResult<Vec<f64>> {
    if open.len() != high.len() || high.len() != low.len() || low.len() != close.len() {
        return Err(TaError::LengthMismatch {
            expected: open.len(),
            got: high.len().max(low.len()).max(close.len()),
        });
    }
    let mut state = YangZhang::new(timeperiod)?;
    Ok(open
        .iter()
        .zip(high)
        .zip(low)
        .zip(close)
        .map(|((( &open, &high), &low), &close)| state.append(open, high, low, close).unwrap_or(f64::NAN))
        .collect())
}

/// WorldQuant Alpha101 `ts_rank(x, d)`: the rank of the current value within
/// the trailing `d`-bar window as a fraction in `(0, 1]`. Shares the rolling
/// rank kernel. Warm-up values are `NaN`.
pub fn ts_rank(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    rolling_rank(input, timeperiod)
}

/// WorldQuant Alpha101 `signedpower(x, a)`: pointwise `sign(x)·|x|^a`.
/// `a == 2` is special-cased to `x·|x|` to avoid `powf`.
pub fn signedpower(input: &[f64], exponent: f64) -> Vec<f64> {
    input
        .iter()
        .map(|&value| {
            if exponent == 2.0 {
                value * value.abs()
            } else {
                value.signum() * value.abs().powf(exponent)
            }
        })
        .collect()
}

/// WorldQuant Alpha101 `decay_linear(x, d)`: verified alias of the weighted
/// moving average with weights `d..=1` — re-exported, zero additional code.
pub fn decay_linear(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    crate::overlap::wma(input, timeperiod)
}

/// Average daily dollar value traded: SMA of `close × volume`.
#[derive(Debug, Clone)]
pub struct Adv {
    sum: f64,
    window: VecDeque<f64>,
    timeperiod: usize,
    value: Option<f64>,
}

impl Adv {
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        validate_period(timeperiod)?;
        Ok(Self { sum: 0.0, window: VecDeque::with_capacity(timeperiod), timeperiod, value: None })
    }

    pub fn append(&mut self, close: f64, volume: f64) -> Option<f64> {
        let term = close * volume;
        if self.window.len() == self.timeperiod {
            self.sum -= self.window.pop_front().expect("ring is full");
        }
        self.window.push_back(term);
        self.sum += term;
        self.value = if self.window.len() == self.timeperiod {
            Some(self.sum / self.timeperiod as f64)
        } else {
            None
        };
        self.value
    }

    pub fn value(&self) -> Option<f64> { self.value }

    pub fn reset(&mut self) {
        self.sum = 0.0;
        self.window.clear();
        self.value = None;
    }
}

pub fn adv(close: &[f64], volume: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    if close.len() != volume.len() {
        return Err(TaError::LengthMismatch { expected: close.len(), got: volume.len() });
    }
    let mut state = Adv::new(timeperiod)?;
    Ok(close
        .iter()
        .zip(volume)
        .map(|(&close, &volume)| state.append(close, volume).unwrap_or(f64::NAN))
        .collect())
}

/// Amihud illiquidity: rolling mean of `|ret| / (close × volume)`.
#[derive(Debug, Clone)]
pub struct Amihud {
    mean: RollingMean,
    previous_close: Option<f64>,
    value: Option<f64>,
}

impl Amihud {
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        Ok(Self { mean: RollingMean::new(timeperiod)?, previous_close: None, value: None })
    }

    pub fn append(&mut self, close: f64, volume: f64) -> Option<f64> {
        if let Some(previous_close) = self.previous_close.replace(close) {
            let term = if close > 0.0 && previous_close > 0.0 && volume > 0.0 {
                ((close - previous_close) / previous_close).abs() / (close * volume)
            } else {
                0.0
            };
            self.value = self.mean.append(term);
        }
        self.value
    }

    pub fn value(&self) -> Option<f64> { self.value }

    pub fn reset(&mut self) {
        self.mean.reset();
        self.previous_close = None;
        self.value = None;
    }
}

pub fn amihud(close: &[f64], volume: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    if close.len() != volume.len() {
        return Err(TaError::LengthMismatch { expected: close.len(), got: volume.len() });
    }
    let mut state = Amihud::new(timeperiod)?;
    Ok(close
        .iter()
        .zip(volume)
        .map(|(&close, &volume)| state.append(close, volume).unwrap_or(f64::NAN))
        .collect())
}

/// Roll spread estimate: `2√max(0, −cov(Δp_t, Δp_{t−1}))`.
#[derive(Debug, Clone)]
pub struct RollSpread {
    previous_price: Option<f64>,
    delta_previous: Option<f64>,
    moments: RollingPairMoments,
    value: Option<f64>,
}

#[derive(Debug, Clone)]
struct RollingPairMoments {
    x: VecDeque<f64>,
    y: VecDeque<f64>,
    timeperiod: usize,
    value: Option<f64>,
}

impl RollingPairMoments {
    fn new(timeperiod: usize) -> TaResult<Self> {
        validate_period(timeperiod)?;
        Ok(Self {
            x: VecDeque::with_capacity(timeperiod),
            y: VecDeque::with_capacity(timeperiod),
            timeperiod,
            value: None,
        })
    }

    fn append(&mut self, x: f64, y: f64) -> Option<f64> {
        if self.x.len() == self.timeperiod {
            self.x.pop_front();
            self.y.pop_front();
        }
        self.x.push_back(x);
        self.y.push_back(y);
        self.value = if self.x.len() == self.timeperiod {
            let n = self.timeperiod as f64;
            let mean_x = self.x.iter().sum::<f64>() / n;
            let mean_y = self.y.iter().sum::<f64>() / n;
            let mut cov = 0.0;
            for (x, y) in self.x.iter().zip(self.y.iter()) {
                cov += (x - mean_x) * (y - mean_y);
            }
            Some(cov / (n - 1.0))
        } else {
            None
        };
        self.value
    }

    fn value(&self) -> Option<f64> { self.value }

    fn reset(&mut self) {
        self.x.clear();
        self.y.clear();
        self.value = None;
    }
}

impl RollSpread {
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        Ok(Self {
            previous_price: None,
            delta_previous: None,
            moments: RollingPairMoments::new(timeperiod)?,
            value: None,
        })
    }

    pub fn append(&mut self, price: f64) -> Option<f64> {
        let delta = if let Some(previous_price) = self.previous_price.replace(price) {
            price - previous_price
        } else {
            0.0
        };
        if let Some(delta_previous) = self.delta_previous {
            let _ = self.moments.append(delta, delta_previous);
        }
        self.delta_previous = Some(delta);
        self.value = self.moments.value().map(|cov| 2.0 * (0.0f64 - cov).max(0.0).sqrt());
        self.value
    }

    pub fn value(&self) -> Option<f64> { self.value }

    pub fn reset(&mut self) {
        self.previous_price = None;
        self.delta_previous = None;
        self.moments.reset();
        self.value = None;
    }
}

pub fn roll_spread(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    let mut state = RollSpread::new(timeperiod)?;
    Ok(input.iter().map(|&price| state.append(price).unwrap_or(f64::NAN)).collect())
}

/// OU half-life: `−ln(2)/λ` where `λ` is the slope of `Δp` on lagged `p`.
/// `λ ≥ 0` yields `NaN`.
#[derive(Debug, Clone)]
pub struct OuHalfLife {
    moments: RollingPairMoments,
    previous_price: Option<f64>,
    value: Option<f64>,
}

impl OuHalfLife {
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        Ok(Self {
            moments: RollingPairMoments::new(timeperiod)?,
            previous_price: None,
            value: None,
        })
    }

    pub fn append(&mut self, price: f64) -> Option<f64> {
        if let Some(previous_price) = self.previous_price.replace(price) {
            let delta = price - previous_price;
            let _ = self.moments.append(delta, previous_price);
        }
        self.value = if let Some(cov) = self.moments.value() {
            let n = self.moments.timeperiod as f64;
            let mean_y = self.moments.y.iter().sum::<f64>() / n;
            let var_y = self
                .moments
                .y
                .iter()
                .map(|&y| (y - mean_y) * (y - mean_y))
                .sum::<f64>()
                / (n - 1.0);
            if var_y > 0.0 {
                let lambda = -cov / var_y;
                (lambda > 0.0).then_some(2.0f64.ln() / lambda)
            } else {
                None
            }
        } else {
            None
        };
        self.value
    }

    pub fn value(&self) -> Option<f64> { self.value }

    pub fn reset(&mut self) {
        self.moments.reset();
        self.previous_price = None;
        self.value = None;
    }
}

pub fn ou_half_life(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    let mut state = OuHalfLife::new(timeperiod)?;
    Ok(input.iter().map(|&price| state.append(price).unwrap_or(f64::NAN)).collect())
}

/// CUSUM event flags (AFML §2.5.2): `+1` when the cumulative deviation from
/// `threshold` (daily volatility) exceeds it, `-1` on the downside, else `0`.
#[derive(Debug, Clone)]
pub struct Cusum {
    threshold: f64,
    s_positive: f64,
    s_negative: f64,
    value: Option<f64>,
}

impl Cusum {
    pub fn new(threshold: f64) -> TaResult<Self> {
        if threshold < 0.0 {
            return Err(TaError::InvalidParameter {
                name: "threshold",
                value: threshold.to_string(),
                reason: "must be >= 0",
            });
        }
        Ok(Self { threshold, s_positive: 0.0, s_negative: 0.0, value: None })
    }

    pub fn append(&mut self, change: f64) -> f64 {
        self.s_positive = (self.s_positive + change).max(0.0);
        self.s_negative = (self.s_negative - change).max(0.0);
        let flag = if self.s_positive > self.threshold {
            self.s_positive = 0.0;
            1.0
        } else if self.s_negative > self.threshold {
            self.s_negative = 0.0;
            -1.0
        } else {
            0.0
        };
        self.value = Some(flag);
        flag
    }

    pub fn value(&self) -> Option<f64> { self.value }

    pub fn reset(&mut self) {
        self.s_positive = 0.0;
        self.s_negative = 0.0;
        self.value = None;
    }
}

pub fn cusum(input: &[f64], threshold: f64) -> TaResult<Vec<f64>> {
    let mut state = Cusum::new(threshold)?;
    Ok(input.iter().map(|&change| state.append(change)).collect())
}

/// Pairs-trading z-score: rolling OLS hedge ratio `β` of `y` on `x`, spread
/// `s = y − β·x`, then `(s − mean(s)) / std(s)` over the same window —
/// composition of the `HedgeRatio` and `RollingZscore` definitions.
#[derive(Debug, Clone)]
pub struct SpreadZscore {
    values: VecDeque<(f64, f64)>,
    timeperiod: usize,
    value: Option<f64>,
}

impl SpreadZscore {
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        validate_period(timeperiod)?;
        Ok(Self { values: VecDeque::with_capacity(timeperiod), timeperiod, value: None })
    }

    pub fn append(&mut self, x: f64, y: f64) -> Option<f64> {
        if self.values.len() == self.timeperiod {
            self.values.pop_front();
        }
        self.values.push_back((x, y));
        self.value = if self.values.len() == self.timeperiod {
            let n = self.timeperiod as f64;
            let mean_x = self.values.iter().map(|&(x, _)| x).sum::<f64>() / n;
            let mean_y = self.values.iter().map(|&(_, y)| y).sum::<f64>() / n;
            let covariance = self.values.iter().map(|&(x, y)| (x - mean_x) * (y - mean_y)).sum::<f64>();
            let variance = self.values.iter().map(|&(x, _)| (x - mean_x).powi(2)).sum::<f64>();
            let beta = if variance > 0.0 { covariance / variance } else { 0.0 };
            let spread = (y - beta * x);
            let mean_spread = self.values.iter().map(|&(x, y)| y - beta * x).sum::<f64>() / n;
            let std_spread = (self
                .values
                .iter()
                .map(|&(x, y)| (y - beta * x - mean_spread).powi(2))
                .sum::<f64>()
                / n)
                .sqrt();
            Some(if std_spread > 0.0 { (spread - mean_spread) / std_spread } else { 0.0 })
        } else {
            None
        };
        self.value
    }

    pub fn value(&self) -> Option<f64> { self.value }

    pub fn reset(&mut self) {
        self.values.clear();
        self.value = None;
    }
}

pub fn spread_zscore(x: &[f64], y: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    if x.len() != y.len() {
        return Err(TaError::LengthMismatch { expected: x.len(), got: y.len() });
    }
    let mut state = SpreadZscore::new(timeperiod)?;
    Ok(x.iter().zip(y).map(|(&x, &y)| state.append(x, y).unwrap_or(f64::NAN)).collect())
}

/// Fractionally-differentiated series (AFML §5.4, fixed-width window).
///
/// Weights `w_0 = 1`, `w_k = −w_{k−1}·(d−k+1)/k` truncated once
/// `|w_k| < threshold`; each output is the dot product of the weights with the
/// last `len(weights)` inputs — O(w) per bar over a ring buffer.
#[derive(Debug, Clone)]
pub struct FracDiff {
    weights: Vec<f64>,
    window: VecDeque<f64>,
    value: Option<f64>,
}

impl FracDiff {
    pub fn new(d: f64, threshold: f64) -> TaResult<Self> {
        if !(d > 0.0) {
            return Err(TaError::InvalidParameter {
                name: "d",
                value: d.to_string(),
                reason: "must be > 0",
            });
        }
        if !(threshold > 0.0) {
            return Err(TaError::InvalidParameter {
                name: "threshold",
                value: threshold.to_string(),
                reason: "must be > 0",
            });
        }
        let mut weights = vec![1.0];
        let mut k = 1usize;
        loop {
            let wk = -weights[k - 1] * (d - k as f64 + 1.0) / k as f64;
            if wk.abs() < threshold {
                break;
            }
            weights.push(wk);
            k += 1;
        }
        let capacity = weights.len();
        Ok(Self { weights, window: VecDeque::with_capacity(capacity), value: None })
    }

    pub fn append(&mut self, input: f64) -> Option<f64> {
        if self.window.len() == self.weights.len() {
            self.window.pop_front();
        }
        self.window.push_back(input);
        self.value = if self.window.len() == self.weights.len() {
            let mut acc = 0.0;
            for (i, &w) in self.weights.iter().enumerate() {
                acc += w * self.window[self.window.len() - 1 - i];
            }
            Some(acc)
        } else {
            None
        };
        self.value
    }

    pub fn value(&self) -> Option<f64> { self.value }

    pub fn reset(&mut self) {
        self.window.clear();
        self.value = None;
    }
}

pub fn frac_diff(input: &[f64], d: f64, threshold: f64) -> TaResult<Vec<f64>> {
    let mut state = FracDiff::new(d, threshold)?;
    Ok(input.iter().map(|&value| state.append(value).unwrap_or(f64::NAN)).collect())
}

/// Online Kalman estimate of the hedge ratio `β` in `y = α + β·x + v`.
///
/// Two-state filter with random-walk transition (`Q = δ·I`) and observation
/// noise `R` (QuantStart "Dynamic Hedge Ratio"; pykalman `filter_update`).
/// The primary output is `β`; `α`, the innovation, and `√S` are also exposed.
/// O(1) per bar — no linear-algebra dependency.
#[derive(Debug, Clone)]
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

    pub fn value(&self) -> Option<f64> { self.value }

    pub fn alpha(&self) -> Option<f64> { self.alpha_value }

    pub fn innovation(&self) -> Option<f64> { self.innovation }

    pub fn std(&self) -> Option<f64> { self.std_value }

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

pub fn kalman_hedge_ratio(x: &[f64], y: &[f64], delta: f64, observation_variance: f64) -> TaResult<Vec<f64>> {
    if x.len() != y.len() {
        return Err(TaError::LengthMismatch { expected: x.len(), got: y.len() });
    }
    let mut state = KalmanHedgeRatio::new(delta, observation_variance)?;
    Ok(x.iter().zip(y).map(|(&x, &y)| state.append(x, y).unwrap_or(f64::NAN)).collect())
}

impl ActiveZoneList {
    pub fn new(capacity: usize) -> TaResult<Self> {
        if capacity == 0 {
            return Err(TaError::InvalidParameter {
                name: "capacity",
                value: capacity.to_string(),
                reason: "must be >= 1",
            });
        }
        Ok(Self { zones: Vec::with_capacity(capacity), capacity, index: 0 })
    }

    pub fn add(&mut self, top: f64, bottom: f64, flags: u32) -> usize {
        if self.zones.len() == self.capacity {
            self.zones.remove(0);
        }
        let (top, bottom) = if top >= bottom { (top, bottom) } else { (bottom, top) };
        self.zones.push(Zone { top, bottom, birth: self.index, flags });
        self.zones.len() - 1
    }

    pub fn advance(&mut self, price: f64, max_age: Option<usize>) -> Vec<bool> {
        self.index = self.index.saturating_add(1);
        let mut mitigated = vec![false; self.zones.len()];
        for (index, zone) in self.zones.iter_mut().enumerate() {
            let expired = max_age.is_some_and(|age| self.index.saturating_sub(zone.birth) > age);
            if !expired && price >= zone.bottom && price <= zone.top {
                zone.flags |= 1;
                mitigated[index] = true;
            }
        }
        self.zones.retain(|zone| {
            let expired = max_age.is_some_and(|age| self.index.saturating_sub(zone.birth) > age);
            !expired && zone.flags & 1 == 0
        });
        mitigated.truncate(self.zones.len());
        mitigated
    }

    pub fn zones(&self) -> &[Zone] { &self.zones }

    pub fn reset(&mut self) {
        self.zones.clear();
        self.index = 0;
    }
}

impl SessionExtrema {
    pub fn new() -> Self { Self::default() }

    pub fn append(&mut self, new_session: bool, high: f64, low: f64) -> SessionExtremaValue {
        if new_session || self.high.is_none() {
            self.high = Some(high);
            self.low = Some(low);
        } else {
            self.high = Some(self.high.expect("session high is initialized").max(high));
            self.low = Some(self.low.expect("session low is initialized").min(low));
        }
        let value = SessionExtremaValue {
            high: self.high.expect("session high is initialized"),
            low: self.low.expect("session low is initialized"),
        };
        self.value = Some(value);
        value
    }

    pub fn value(&self) -> Option<SessionExtremaValue> { self.value }

    pub fn reset(&mut self) {
        self.high = None;
        self.low = None;
        self.value = None;
    }
}

/// Causal swing-point confirmation.
///
/// The center bar of a `2 * swing_length + 1` window is confirmed at the
/// current bar. A signal is emitted only after the required future bars have
/// arrived, so no output uses lookahead when it is observed.
pub fn swing_highs_lows(
    high: &[f64],
    low: &[f64],
    swing_length: usize,
) -> TaResult<(Vec<f64>, Vec<f64>, Vec<f64>)> {
    if high.len() != low.len() {
        return Err(TaError::LengthMismatch {
            expected: high.len(),
            got: low.len(),
        });
    }
    let mut state = Swing::new(swing_length)?;
    let mut signal = Vec::with_capacity(high.len());
    let mut level = Vec::with_capacity(high.len());
    let mut bars_since = Vec::with_capacity(high.len());
    for (&high, &low) in high.iter().zip(low) {
        let value = state.append(high, low);
        signal.push(value.map_or(f64::NAN, |value| value.signal));
        level.push(value.map_or(f64::NAN, |value| value.level));
        bars_since.push(value.map_or(f64::NAN, |value| value.bars_since));
    }
    Ok((signal, level, bars_since))
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SwingValue {
    pub signal: f64,
    pub level: f64,
    pub bars_since: f64,
}

#[derive(Debug, Clone)]
pub struct Swing {
    highs: VecDeque<f64>,
    lows: VecDeque<f64>,
    length: usize,
    bars_since: Option<usize>,
    value: Option<SwingValue>,
}

impl Swing {
    pub fn new(length: usize) -> TaResult<Self> {
        validate_period(length)?;
        let capacity = length.saturating_mul(2).saturating_add(1);
        Ok(Self {
            highs: VecDeque::with_capacity(capacity),
            lows: VecDeque::with_capacity(capacity),
            length,
            bars_since: None,
            value: None,
        })
    }

    pub fn append(&mut self, high: f64, low: f64) -> Option<SwingValue> {
        let capacity = self.length * 2 + 1;
        if self.highs.len() == capacity {
            self.highs.pop_front();
            self.lows.pop_front();
        }
        self.highs.push_back(high);
        self.lows.push_back(low);

        if self.highs.len() < capacity {
            self.value = None;
            return None;
        }
        let center_high = self.highs[self.length];
        let center_low = self.lows[self.length];
        let is_high = center_high >= self.highs.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        let is_low = center_low <= self.lows.iter().copied().fold(f64::INFINITY, f64::min);
        let (signal, level) = match (is_high, is_low) {
            (true, false) => (1.0, center_high),
            (false, true) => (-1.0, center_low),
            _ => (f64::NAN, f64::NAN),
        };
        self.bars_since = if signal.is_nan() {
            self.bars_since.map(|bars| bars + 1)
        } else {
            Some(0)
        };
        let value = SwingValue {
            signal,
            level,
            bars_since: self.bars_since.map_or(f64::NAN, |bars| bars as f64),
        };
        self.value = Some(value);
        Some(value)
    }

    pub fn value(&self) -> Option<SwingValue> { self.value }

    pub fn bars_since(&self) -> Option<f64> { self.bars_since.map(|bars| bars as f64) }

    pub fn reset(&mut self) {
        self.highs.clear();
        self.lows.clear();
        self.bars_since = None;
        self.value = None;
    }
}

/// Rolling median. Warm-up values are `NaN`; even windows average the two
/// central values.
pub fn rolling_median(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    validate_period(timeperiod)?;
    let mut state = RollingMedian::new(timeperiod)?;
    Ok(input.iter().map(|&value| state.append(value).unwrap_or(f64::NAN)).collect())
}

/// Rolling mode. Warm-up values are `NaN`; exact-value ties keep the earliest
/// value in the current window.
pub fn rolling_mode(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    validate_period(timeperiod)?;
    let mut state = RollingMode::new(timeperiod)?;
    Ok(input.iter().map(|&value| state.append(value).unwrap_or(f64::NAN)).collect())
}

pub fn rolling_quantile(input: &[f64], timeperiod: usize, quantile: f64) -> TaResult<Vec<f64>> {
    validate_quantile(quantile)?;
    let mut state = RollingQuantile::new(timeperiod, quantile)?;
    Ok(input.iter().map(|&value| state.append(value).unwrap_or(f64::NAN)).collect())
}

pub fn rolling_percentile(input: &[f64], timeperiod: usize, percentile: f64) -> TaResult<Vec<f64>> {
    if !(0.0..=100.0).contains(&percentile) {
        return Err(TaError::InvalidParameter { name: "percentile", value: percentile.to_string(), reason: "must be between 0 and 100" });
    }
    rolling_quantile(input, timeperiod, percentile / 100.0)
}

pub fn rolling_rank(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    let mut state = RollingRank::new(timeperiod)?;
    Ok(input.iter().map(|&value| state.append(value).unwrap_or(f64::NAN)).collect())
}

pub fn rolling_zscore(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    let mut state = RollingZscore::new(timeperiod)?;
    Ok(input.iter().map(|&value| state.append(value).unwrap_or(f64::NAN)).collect())
}

pub fn rolling_skew(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    let mut state = RollingSkew::new(timeperiod)?;
    Ok(input.iter().map(|&value| state.append(value).unwrap_or(f64::NAN)).collect())
}

pub fn rolling_kurtosis(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    let mut state = RollingKurtosis::new(timeperiod)?;
    Ok(input.iter().map(|&value| state.append(value).unwrap_or(f64::NAN)).collect())
}

pub fn rolling_iqr(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    let mut state = RollingIqr::new(timeperiod)?;
    Ok(input.iter().map(|&value| state.append(value).unwrap_or(f64::NAN)).collect())
}

pub fn rolling_cov(input0: &[f64], input1: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    if input0.len() != input1.len() { return Err(TaError::LengthMismatch { expected: input0.len(), got: input1.len() }); }
    let mut state = RollingCov::new(timeperiod)?;
    Ok(input0.iter().zip(input1).map(|(&left, &right)| state.append(left, right).unwrap_or(f64::NAN)).collect())
}

pub fn rolling_winsorize(input: &[f64], timeperiod: usize, lower: f64, upper: f64) -> TaResult<Vec<f64>> {
    let mut state = RollingWinsorize::new(timeperiod, lower, upper)?;
    Ok(input.iter().map(|&value| state.append(value).unwrap_or(f64::NAN)).collect())
}

pub fn ewm_var(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    let mut state = EwmVar::new(timeperiod)?;
    Ok(input.iter().map(|&value| state.append(value)).collect())
}

pub fn ewm_std(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    let mut state = EwmStd::new(timeperiod)?;
    Ok(input.iter().map(|&value| state.append(value)).collect())
}

pub fn ewm_cov(input0: &[f64], input1: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    if input0.len() != input1.len() { return Err(TaError::LengthMismatch { expected: input0.len(), got: input1.len() }); }
    let mut state = EwmCov::new(timeperiod)?;
    Ok(input0.iter().zip(input1).map(|(&left, &right)| state.append(left, right)).collect())
}

pub fn ewm_corr(input0: &[f64], input1: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    if input0.len() != input1.len() { return Err(TaError::LengthMismatch { expected: input0.len(), got: input1.len() }); }
    let mut state = EwmCorr::new(timeperiod)?;
    Ok(input0.iter().zip(input1).map(|(&left, &right)| state.append(left, right)).collect())
}

#[derive(Debug, Clone)]
pub struct Lag {
    values: VecDeque<f64>,
    timeperiod: usize,
    value: Option<f64>,
}

impl Lag {
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        validate_period(timeperiod)?;
        Ok(Self { values: VecDeque::with_capacity(timeperiod), timeperiod, value: None })
    }
    pub fn append(&mut self, input: f64) -> Option<f64> {
        self.value = if self.values.len() == self.timeperiod {
            let value = self.values.pop_front().expect("lag window is full");
            self.values.push_back(input);
            Some(value)
        } else {
            self.values.push_back(input);
            None
        };
        self.value
    }
    pub fn value(&self) -> Option<f64> { self.value }
    pub fn reset(&mut self) { self.values.clear(); self.value = None; }
}

#[derive(Debug, Clone)]
pub struct LogReturn { lag: Lag, value: Option<f64> }

impl LogReturn {
    pub fn new(timeperiod: usize) -> TaResult<Self> { Ok(Self { lag: Lag::new(timeperiod)?, value: None }) }
    pub fn append(&mut self, input: f64) -> Option<f64> {
        self.value = self.lag.append(input).map(|previous| (input / previous).ln());
        self.value
    }
    pub fn value(&self) -> Option<f64> { self.value }
    pub fn reset(&mut self) { self.lag.reset(); self.value = None; }
}

#[derive(Debug, Clone)]
pub struct RollingMean {
    values: VecDeque<f64>,
    timeperiod: usize,
    sum: f64,
    value: Option<f64>,
}

impl RollingMean {
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        validate_period(timeperiod)?;
        Ok(Self { values: VecDeque::with_capacity(timeperiod), timeperiod, sum: 0.0, value: None })
    }

    pub fn append(&mut self, input: f64) -> Option<f64> {
        if self.values.len() == self.timeperiod {
            self.sum -= self.values.pop_front().expect("ring is full");
        }
        self.values.push_back(input);
        self.sum += input;
        self.value = if self.values.len() == self.timeperiod {
            Some(self.sum / self.timeperiod as f64)
        } else {
            None
        };
        self.value
    }

    pub fn value(&self) -> Option<f64> { self.value }

    pub fn reset(&mut self) {
        self.values.clear();
        self.sum = 0.0;
        self.value = None;
    }
}

#[derive(Debug, Clone)]
pub struct RollingMedian {
    values: VecDeque<f64>,
    timeperiod: usize,
    value: Option<f64>,
}

impl RollingMedian {
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        validate_period(timeperiod)?;
        Ok(Self { values: VecDeque::with_capacity(timeperiod), timeperiod, value: None })
    }

    pub fn append(&mut self, input: f64) -> Option<f64> {
        if self.values.len() == self.timeperiod { self.values.pop_front(); }
        self.values.push_back(input);
        self.value = if self.values.len() == self.timeperiod {
            let mut sorted: Vec<f64> = self.values.iter().copied().collect();
            sorted.sort_by(f64::total_cmp);
            let middle = self.timeperiod / 2;
            Some(if self.timeperiod % 2 == 1 {
                sorted[middle]
            } else {
                (sorted[middle - 1] + sorted[middle]) * 0.5
            })
        } else { None };
        self.value
    }

    pub fn value(&self) -> Option<f64> { self.value }
    pub fn reset(&mut self) { self.values.clear(); self.value = None; }
}

#[derive(Debug, Clone)]
pub struct RollingMode {
    values: VecDeque<f64>,
    timeperiod: usize,
    value: Option<f64>,
}

impl RollingMode {
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        validate_period(timeperiod)?;
        Ok(Self { values: VecDeque::with_capacity(timeperiod), timeperiod, value: None })
    }

    pub fn append(&mut self, input: f64) -> Option<f64> {
        if self.values.len() == self.timeperiod { self.values.pop_front(); }
        self.values.push_back(input);
        self.value = if self.values.len() == self.timeperiod {
            let mut best = self.values[0];
            let mut best_count = 0;
            for &candidate in &self.values {
                let count = self.values.iter().filter(|&&value| value == candidate).count();
                if count > best_count { best = candidate; best_count = count; }
            }
            Some(best)
        } else { None };
        self.value
    }

    pub fn value(&self) -> Option<f64> { self.value }
    pub fn reset(&mut self) { self.values.clear(); self.value = None; }
}

#[derive(Debug, Clone)]
pub struct RollingQuantile {
    values: VecDeque<f64>,
    timeperiod: usize,
    quantile: f64,
    value: Option<f64>,
}

impl RollingQuantile {
    pub fn new(timeperiod: usize, quantile: f64) -> TaResult<Self> {
        validate_period(timeperiod)?;
        validate_quantile(quantile)?;
        Ok(Self { values: VecDeque::with_capacity(timeperiod), timeperiod, quantile, value: None })
    }
    pub fn append(&mut self, input: f64) -> Option<f64> {
        if self.values.len() == self.timeperiod { self.values.pop_front(); }
        self.values.push_back(input);
        self.value = if self.values.len() == self.timeperiod {
            let mut sorted: Vec<f64> = self.values.iter().copied().collect();
            sorted.sort_by(f64::total_cmp);
            let position = self.quantile * (self.timeperiod - 1) as f64;
            let lower = position.floor() as usize;
            let upper = position.ceil() as usize;
            Some(sorted[lower] + (sorted[upper] - sorted[lower]) * (position - lower as f64))
        } else { None };
        self.value
    }
    pub fn value(&self) -> Option<f64> { self.value }
    pub fn reset(&mut self) { self.values.clear(); self.value = None; }
}

#[derive(Debug, Clone)]
pub struct RollingRank {
    values: VecDeque<f64>,
    timeperiod: usize,
    value: Option<f64>,
}

impl RollingRank {
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        validate_period(timeperiod)?;
        Ok(Self { values: VecDeque::with_capacity(timeperiod), timeperiod, value: None })
    }
    pub fn append(&mut self, input: f64) -> Option<f64> {
        if self.values.len() == self.timeperiod { self.values.pop_front(); }
        self.values.push_back(input);
        self.value = if self.values.len() == self.timeperiod {
            let less = self.values.iter().filter(|&&value| value < input).count();
            let equal = self.values.iter().filter(|&&value| value == input).count();
            Some((less as f64 + equal as f64) / self.timeperiod as f64)
        } else { None };
        self.value
    }
    pub fn value(&self) -> Option<f64> { self.value }
    pub fn reset(&mut self) { self.values.clear(); self.value = None; }
}

#[derive(Debug, Clone)]
pub struct RollingZscore {
    values: VecDeque<f64>,
    timeperiod: usize,
    value: Option<f64>,
}

impl RollingZscore {
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        validate_period(timeperiod)?;
        Ok(Self { values: VecDeque::with_capacity(timeperiod), timeperiod, value: None })
    }
    pub fn append(&mut self, input: f64) -> Option<f64> {
        if self.values.len() == self.timeperiod { self.values.pop_front(); }
        self.values.push_back(input);
        self.value = if self.values.len() == self.timeperiod {
            let mean = self.values.iter().sum::<f64>() / self.timeperiod as f64;
            let variance = self.values.iter().map(|&value| (value - mean).powi(2)).sum::<f64>() / self.timeperiod as f64;
            Some(if variance > 0.0 { (input - mean) / variance.sqrt() } else { 0.0 })
        } else { None };
        self.value
    }
    pub fn value(&self) -> Option<f64> { self.value }
    pub fn reset(&mut self) { self.values.clear(); self.value = None; }
}

macro_rules! rolling_moment_operator {
    ($name:ident, $formula:expr) => {
        #[derive(Debug, Clone)]
        pub struct $name { values: VecDeque<f64>, timeperiod: usize, value: Option<f64> }
        impl $name {
            pub fn new(timeperiod: usize) -> TaResult<Self> {
                validate_period(timeperiod)?;
                Ok(Self { values: VecDeque::with_capacity(timeperiod), timeperiod, value: None })
            }
            pub fn append(&mut self, input: f64) -> Option<f64> {
                if self.values.len() == self.timeperiod { self.values.pop_front(); }
                self.values.push_back(input);
                self.value = if self.values.len() == self.timeperiod {
                    let mean = self.values.iter().sum::<f64>() / self.timeperiod as f64;
                    let result = $formula(&self.values, mean);
                    Some(result)
                } else { None };
                self.value
            }
            pub fn value(&self) -> Option<f64> { self.value }
            pub fn reset(&mut self) { self.values.clear(); self.value = None; }
        }
    };
}

rolling_moment_operator!(RollingSkew, |values: &VecDeque<f64>, mean: f64| {
    let m2 = values.iter().map(|&value| (value - mean).powi(2)).sum::<f64>() / values.len() as f64;
    let m3 = values.iter().map(|&value| (value - mean).powi(3)).sum::<f64>() / values.len() as f64;
    if m2 > 0.0 { m3 / m2.powf(1.5) } else { 0.0 }
});

rolling_moment_operator!(RollingKurtosis, |values: &VecDeque<f64>, mean: f64| {
    let m2 = values.iter().map(|&value| (value - mean).powi(2)).sum::<f64>() / values.len() as f64;
    let m4 = values.iter().map(|&value| (value - mean).powi(4)).sum::<f64>() / values.len() as f64;
    if m2 > 0.0 { m4 / m2.powi(2) - 3.0 } else { 0.0 }
});

#[derive(Debug, Clone)]
pub struct RollingIqr { quantile: RollingQuantile, value: Option<f64> }

impl RollingIqr {
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        Ok(Self { quantile: RollingQuantile::new(timeperiod, 0.25)?, value: None })
    }
    pub fn append(&mut self, input: f64) -> Option<f64> {
        self.quantile.append(input);
        self.value = if self.quantile.values.len() == self.quantile.timeperiod {
            let mut sorted: Vec<f64> = self.quantile.values.iter().copied().collect();
            sorted.sort_by(f64::total_cmp);
            let quantile = |q: f64| {
                let position = q * (sorted.len() - 1) as f64;
                let lower = position.floor() as usize;
                let upper = position.ceil() as usize;
                sorted[lower] + (sorted[upper] - sorted[lower]) * (position - lower as f64)
            };
            Some(quantile(0.75) - quantile(0.25))
        } else { None };
        self.value
    }
    pub fn value(&self) -> Option<f64> { self.value }
    pub fn reset(&mut self) { self.quantile.reset(); self.value = None; }
}

#[derive(Debug, Clone)]
pub struct RollingCov { values: VecDeque<(f64, f64)>, timeperiod: usize, value: Option<f64> }

impl RollingCov {
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        validate_period(timeperiod)?;
        Ok(Self { values: VecDeque::with_capacity(timeperiod), timeperiod, value: None })
    }
    pub fn append(&mut self, left: f64, right: f64) -> Option<f64> {
        if self.values.len() == self.timeperiod { self.values.pop_front(); }
        self.values.push_back((left, right));
        self.value = if self.values.len() == self.timeperiod {
            let n = self.timeperiod as f64;
            let left_mean = self.values.iter().map(|&(left, _)| left).sum::<f64>() / n;
            let right_mean = self.values.iter().map(|&(_, right)| right).sum::<f64>() / n;
            Some(self.values.iter().map(|&(left, right)| (left - left_mean) * (right - right_mean)).sum::<f64>() / n)
        } else { None };
        self.value
    }
    pub fn value(&self) -> Option<f64> { self.value }
    pub fn reset(&mut self) { self.values.clear(); self.value = None; }
}

#[derive(Debug, Clone)]
pub struct RollingWinsorize { values: VecDeque<f64>, timeperiod: usize, lower: f64, upper: f64, value: Option<f64> }

impl RollingWinsorize {
    pub fn new(timeperiod: usize, lower: f64, upper: f64) -> TaResult<Self> {
        validate_period(timeperiod)?;
        validate_quantile(lower)?;
        validate_quantile(upper)?;
        if lower > upper { return Err(TaError::InvalidParameter { name: "lower/upper", value: format!("{lower}/{upper}"), reason: "lower must be <= upper" }); }
        Ok(Self { values: VecDeque::with_capacity(timeperiod), timeperiod, lower, upper, value: None })
    }
    pub fn append(&mut self, input: f64) -> Option<f64> {
        if self.values.len() == self.timeperiod { self.values.pop_front(); }
        self.values.push_back(input);
        self.value = if self.values.len() == self.timeperiod {
            let mut sorted: Vec<f64> = self.values.iter().copied().collect();
            sorted.sort_by(f64::total_cmp);
            let quantile = |q: f64| {
                let position = q * (sorted.len() - 1) as f64;
                let lower = position.floor() as usize;
                let upper = position.ceil() as usize;
                sorted[lower] + (sorted[upper] - sorted[lower]) * (position - lower as f64)
            };
            Some(input.max(quantile(self.lower)).min(quantile(self.upper)))
        } else { None };
        self.value
    }
    pub fn value(&self) -> Option<f64> { self.value }
    pub fn reset(&mut self) { self.values.clear(); self.value = None; }
}

fn ewm_alpha(timeperiod: usize) -> TaResult<f64> {
    validate_period(timeperiod)?;
    Ok(2.0 / (timeperiod as f64 + 1.0))
}

#[derive(Debug, Clone)]
pub struct EwmVar { alpha: f64, mean: Option<f64>, variance: f64, value: Option<f64> }

impl EwmVar {
    pub fn new(timeperiod: usize) -> TaResult<Self> { Ok(Self { alpha: ewm_alpha(timeperiod)?, mean: None, variance: 0.0, value: None }) }
    pub fn append(&mut self, input: f64) -> f64 {
        let variance = match self.mean {
            None => { self.mean = Some(input); 0.0 }
            Some(previous) => {
                let delta = input - previous;
                self.mean = Some(previous + self.alpha * delta);
                (1.0 - self.alpha) * (self.variance + self.alpha * delta * delta)
            }
        };
        self.variance = variance;
        self.value = Some(variance);
        variance
    }
    pub fn value(&self) -> Option<f64> { self.value }
    pub fn reset(&mut self) { self.mean = None; self.variance = 0.0; self.value = None; }
}

#[derive(Debug, Clone)]
pub struct EwmStd { variance: EwmVar, value: Option<f64> }

impl EwmStd {
    pub fn new(timeperiod: usize) -> TaResult<Self> { Ok(Self { variance: EwmVar::new(timeperiod)?, value: None }) }
    pub fn append(&mut self, input: f64) -> f64 { let value = self.variance.append(input).sqrt(); self.value = Some(value); value }
    pub fn value(&self) -> Option<f64> { self.value }
    pub fn reset(&mut self) { self.variance.reset(); self.value = None; }
}

#[derive(Debug, Clone)]
pub struct EwmCov { alpha: f64, mean0: Option<f64>, mean1: Option<f64>, var0: f64, var1: f64, covariance: f64, value: Option<f64> }

impl EwmCov {
    pub fn new(timeperiod: usize) -> TaResult<Self> { Ok(Self { alpha: ewm_alpha(timeperiod)?, mean0: None, mean1: None, var0: 0.0, var1: 0.0, covariance: 0.0, value: None }) }
    pub fn append(&mut self, left: f64, right: f64) -> f64 {
        let covariance = match (self.mean0, self.mean1) {
            (Some(previous0), Some(previous1)) => {
                let delta0 = left - previous0;
                let delta1 = right - previous1;
                self.mean0 = Some(previous0 + self.alpha * delta0);
                self.mean1 = Some(previous1 + self.alpha * delta1);
                self.var0 = (1.0 - self.alpha) * (self.var0 + self.alpha * delta0 * delta0);
                self.var1 = (1.0 - self.alpha) * (self.var1 + self.alpha * delta1 * delta1);
                (1.0 - self.alpha) * (self.covariance + self.alpha * delta0 * delta1)
            }
            _ => { self.mean0 = Some(left); self.mean1 = Some(right); 0.0 }
        };
        self.covariance = covariance;
        self.value = Some(covariance);
        covariance
    }
    pub fn value(&self) -> Option<f64> { self.value }
    pub fn reset(&mut self) { self.mean0 = None; self.mean1 = None; self.var0 = 0.0; self.var1 = 0.0; self.covariance = 0.0; self.value = None; }
}

#[derive(Debug, Clone)]
pub struct EwmCorr { covariance: EwmCov, value: Option<f64> }

impl EwmCorr {
    pub fn new(timeperiod: usize) -> TaResult<Self> { Ok(Self { covariance: EwmCov::new(timeperiod)?, value: None }) }
    pub fn append(&mut self, left: f64, right: f64) -> f64 {
        self.covariance.append(left, right);
        let denominator = (self.covariance.var0 * self.covariance.var1).sqrt();
        let value = if denominator > 0.0 { self.covariance.covariance / denominator } else { 0.0 };
        self.value = Some(value);
        value
    }
    pub fn value(&self) -> Option<f64> { self.value }
    pub fn reset(&mut self) { self.covariance.reset(); self.value = None; }
}

macro_rules! cumulative_operator {
    ($name:ident, $initial:expr, $operation:expr) => {
        #[derive(Debug, Clone)]
        pub struct $name { total: f64, value: Option<f64> }
        impl $name {
            pub fn new() -> Self { Self { total: $initial, value: None } }
            pub fn append(&mut self, input: f64) -> f64 {
                self.total = $operation(self.total, input);
                self.value = Some(self.total);
                self.total
            }
            pub fn value(&self) -> Option<f64> { self.value }
            pub fn reset(&mut self) { self.total = $initial; self.value = None; }
        }
        impl Default for $name { fn default() -> Self { Self::new() } }
    };
}

cumulative_operator!(Cumsum, 0.0, |total: f64, input: f64| total + input);
cumulative_operator!(Cumprod, 1.0, |total: f64, input: f64| total * input);

macro_rules! cumulative_extrema_operator {
    ($name:ident, $initial:expr, $operation:expr) => {
        #[derive(Debug, Clone)]
        pub struct $name { extreme: f64, value: Option<f64> }
        impl $name {
            pub fn new() -> Self { Self { extreme: $initial, value: None } }
            pub fn append(&mut self, input: f64) -> f64 { self.extreme = $operation(self.extreme, input); self.value = Some(self.extreme); self.extreme }
            pub fn value(&self) -> Option<f64> { self.value }
            pub fn reset(&mut self) { self.extreme = $initial; self.value = None; }
        }
        impl Default for $name { fn default() -> Self { Self::new() } }
    };
}

cumulative_extrema_operator!(Cummax, f64::NEG_INFINITY, f64::max);
cumulative_extrema_operator!(Cummin, f64::INFINITY, f64::min);

#[derive(Debug, Clone)]
pub struct Drawdown { maximum: Cummax, value: Option<f64> }
impl Drawdown {
    pub fn new() -> Self { Self { maximum: Cummax::new(), value: None } }
    pub fn append(&mut self, input: f64) -> f64 { let maximum = self.maximum.append(input); let value = if maximum != 0.0 { input / maximum - 1.0 } else { 0.0 }; self.value = Some(value); value }
    pub fn value(&self) -> Option<f64> { self.value }
    pub fn reset(&mut self) { self.maximum.reset(); self.value = None; }
}
impl Default for Drawdown { fn default() -> Self { Self::new() } }

macro_rules! rolling_risk_operator {
    ($name:ident, $formula:expr) => {
        #[derive(Debug, Clone)]
        pub struct $name { values: VecDeque<f64>, timeperiod: usize, value: Option<f64> }
        impl $name {
            pub fn new(timeperiod: usize) -> TaResult<Self> { validate_period(timeperiod)?; Ok(Self { values: VecDeque::with_capacity(timeperiod), timeperiod, value: None }) }
            pub fn append(&mut self, input: f64) -> Option<f64> {
                if self.values.len() == self.timeperiod { self.values.pop_front(); }
                self.values.push_back(input);
                self.value = if self.values.len() == self.timeperiod { Some($formula(&self.values)) } else { None };
                self.value
            }
            pub fn value(&self) -> Option<f64> { self.value }
            pub fn reset(&mut self) { self.values.clear(); self.value = None; }
        }
    };
}

fn mean(values: &VecDeque<f64>) -> f64 { values.iter().sum::<f64>() / values.len() as f64 }

fn weighted_mean(values: &VecDeque<f64>) -> f64 { let denominator = (values.len() * (values.len() + 1) / 2) as f64; values.iter().enumerate().map(|(i,&v)| v * (i + 1) as f64).sum::<f64>() / denominator }

#[derive(Debug, Clone)]
pub struct Hma { raw: VecDeque<f64>, intermediate: VecDeque<f64>, period: usize, half: usize, smooth: usize, value: Option<f64> }
impl Hma { pub fn new(period:usize)->TaResult<Self>{validate_period(period)?;let half=(period/2).max(1);let smooth=(period as f64).sqrt().floor() as usize;Ok(Self{raw:VecDeque::with_capacity(period),intermediate:VecDeque::with_capacity(smooth.max(1)),period,half,smooth:smooth.max(1),value:None})} pub fn append(&mut self,input:f64)->Option<f64>{if self.raw.len()==self.period{self.raw.pop_front();}self.raw.push_back(input);if self.raw.len()>=self.half&&self.raw.len()>=self.period{let half=weighted_mean(&self.raw.iter().skip(self.period-self.half).copied().collect());let full=weighted_mean(&self.raw);if self.intermediate.len()==self.smooth{self.intermediate.pop_front();}self.intermediate.push_back(2.0*half-full);self.value=(self.intermediate.len()==self.smooth).then(||weighted_mean(&self.intermediate));}else{self.value=None}self.value}pub fn value(&self)->Option<f64>{self.value}pub fn reset(&mut self){self.raw.clear();self.intermediate.clear();self.value=None;}}

#[derive(Debug, Clone)]
pub struct Vwma { prices: VecDeque<f64>, volumes: VecDeque<f64>, period: usize, value: Option<f64> }
impl Vwma { pub fn new(period:usize)->TaResult<Self>{validate_period(period)?;Ok(Self{prices:VecDeque::with_capacity(period),volumes:VecDeque::with_capacity(period),period,value:None})} pub fn append(&mut self,price:f64,volume:f64)->Option<f64>{if self.prices.len()==self.period{self.prices.pop_front();self.volumes.pop_front();}self.prices.push_back(price);self.volumes.push_back(volume);self.value=(self.prices.len()==self.period).then(||{let volume=self.volumes.iter().sum::<f64>();if volume!=0.0{self.prices.iter().zip(&self.volumes).map(|(&p,&v)|p*v).sum::<f64>()/volume}else{0.0}});self.value}pub fn value(&self)->Option<f64>{self.value}pub fn reset(&mut self){self.prices.clear();self.volumes.clear();self.value=None;}}

#[derive(Debug, Clone)]
pub struct Zlema { values: VecDeque<f64>, period: usize, lag: usize, alpha: f64, ema: Option<f64>, value: Option<f64> }
impl Zlema { pub fn new(period:usize)->TaResult<Self>{validate_period(period)?;Ok(Self{values:VecDeque::with_capacity((period/2).max(1)),period,lag:(period-1)/2,alpha:2.0/(period as f64+1.0),ema:None,value:None})}pub fn append(&mut self,input:f64)->Option<f64>{if self.values.len()==self.lag.max(1){self.values.pop_front();}self.values.push_back(input);if self.values.len()<=self.lag{self.value=None}else{let lagged=self.values.front().copied().unwrap_or(input);let adjusted=2.0*input-lagged;self.ema=Some(match self.ema{Some(previous)=>previous+self.alpha*(adjusted-previous),None=>adjusted});self.value=self.ema;}self.value}pub fn value(&self)->Option<f64>{self.value}pub fn reset(&mut self){self.values.clear();self.ema=None;self.value=None;}}

#[derive(Debug, Clone)]
pub struct Alma { values: VecDeque<f64>, period: usize, weights: Vec<f64>, value: Option<f64> }
impl Alma { pub fn new(period:usize,offset:f64,sigma:f64)->TaResult<Self>{validate_period(period)?;if !(0.0..=1.0).contains(&offset)||sigma<=0.0{return Err(TaError::InvalidParameter{name:"offset/sigma",value:format!("{offset}/{sigma}"),reason:"offset must be 0..1 and sigma must be positive"});}let m=offset*(period-1)as f64;let weights=(0..period).map(|i|((-(i as f64-m).powi(2)/(2.0*sigma.powi(2)*(period as f64).powi(2))).exp())).collect();Ok(Self{values:VecDeque::with_capacity(period),period,weights,value:None})}pub fn append(&mut self,input:f64)->Option<f64>{if self.values.len()==self.period{self.values.pop_front();}self.values.push_back(input);self.value=(self.values.len()==self.period).then(||{let total=self.weights.iter().sum::<f64>();self.values.iter().zip(&self.weights).map(|(&v,&w)|v*w).sum::<f64>()/total});self.value}pub fn value(&self)->Option<f64>{self.value}pub fn reset(&mut self){self.values.clear();self.value=None;}}

#[derive(Debug, Clone)]
pub struct Tsi { previous: Option<f64>, fast: usize, slow: usize, alpha_fast: f64, alpha_slow: f64, momentum: Option<f64>, absolute: Option<f64>, value: Option<f64> }
impl Tsi { pub fn new(fast:usize,slow:usize)->TaResult<Self>{validate_period(fast)?;validate_period(slow)?;Ok(Self{previous:None,fast,slow,alpha_fast:2.0/(fast as f64+1.0),alpha_slow:2.0/(slow as f64+1.0),momentum:None,absolute:None,value:None})}pub fn append(&mut self,input:f64)->Option<f64>{let previous=self.previous.replace(input)?;let change=input-previous;let abs=change.abs();let m1=self.momentum.map_or(change,|v|v+self.alpha_fast*(change-v));let a1=self.absolute.map_or(abs,|v|v+self.alpha_fast*(abs-v));self.momentum=Some(m1);self.absolute=Some(a1);let m2=self.momentum.map_or(m1,|v|v+self.alpha_slow*(m1-v));let a2=self.absolute.map_or(a1,|v|v+self.alpha_slow*(a1-v));let value=if a2!=0.0{Some(100.0*m2/a2)}else{Some(0.0)};self.value=value;value}pub fn value(&self)->Option<f64>{self.value}pub fn reset(&mut self){self.previous=None;self.momentum=None;self.absolute=None;self.value=None;}}

#[derive(Debug, Clone)]
pub struct AwesomeOscillator { fast: usize, slow: usize, values: VecDeque<f64>, value: Option<f64> }
impl AwesomeOscillator { pub fn new(fast:usize,slow:usize)->TaResult<Self>{validate_period(fast)?;validate_period(slow)?;if fast>slow{return Err(TaError::InvalidParameter{name:"fast/slow",value:format!("{fast}/{slow}"),reason:"fast must be <= slow"});}Ok(Self{fast,slow,values:VecDeque::with_capacity(slow),value:None})}pub fn append(&mut self,high:f64,low:f64)->Option<f64>{if self.values.len()==self.slow{self.values.pop_front();}self.values.push_back((high+low)*0.5);self.value=(self.values.len()==self.slow).then(||{let fast=self.values.iter().rev().take(self.fast).sum::<f64>()/self.fast as f64;let slow=self.values.iter().sum::<f64>()/self.slow as f64;fast-slow});self.value}pub fn value(&self)->Option<f64>{self.value}pub fn reset(&mut self){self.values.clear();self.value=None;}}

#[derive(Debug, Clone)]
pub struct FisherTransform { period: usize, values: VecDeque<f64>, previous: f64, value: Option<f64> }
impl FisherTransform { pub fn new(period:usize)->TaResult<Self>{validate_period(period)?;Ok(Self{period,values:VecDeque::with_capacity(period),previous:0.0,value:None})}pub fn append(&mut self,high:f64,low:f64)->Option<f64>{if self.values.len()==self.period{self.values.pop_front();}self.values.push_back((high+low)*0.5);self.value=(self.values.len()==self.period).then(||{let high=self.values.iter().copied().fold(f64::NEG_INFINITY,f64::max);let low=self.values.iter().copied().fold(f64::INFINITY,f64::min);let normalized=if high!=low{2.0*((self.values.back().copied().unwrap()-low)/(high-low)-0.5)}else{0.0};let x=(0.66*normalized+0.67*self.previous).clamp(-0.999, 0.999);self.previous=x;0.5*((1.0+x)/(1.0-x)).ln()});self.value}pub fn value(&self)->Option<f64>{self.value}pub fn reset(&mut self){self.values.clear();self.previous=0.0;self.value=None;}}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DonchianValue { pub upper: f64, pub lower: f64, pub middle: f64 }
#[derive(Debug, Clone)]
pub struct Donchian { highs: VecDeque<f64>, lows: VecDeque<f64>, period: usize, value: Option<DonchianValue> }
impl Donchian { pub fn new(period:usize)->TaResult<Self>{validate_period(period)?;Ok(Self{highs:VecDeque::with_capacity(period),lows:VecDeque::with_capacity(period),period,value:None})}pub fn append(&mut self,high:f64,low:f64)->Option<DonchianValue>{if self.highs.len()==self.period{self.highs.pop_front();self.lows.pop_front();}self.highs.push_back(high);self.lows.push_back(low);self.value=(self.highs.len()==self.period).then(||{let upper=self.highs.iter().copied().fold(f64::NEG_INFINITY,f64::max);let lower=self.lows.iter().copied().fold(f64::INFINITY,f64::min);DonchianValue{upper,lower,middle:(upper+lower)*0.5}});self.value}pub fn value(&self)->Option<DonchianValue>{self.value}pub fn reset(&mut self){self.highs.clear();self.lows.clear();self.value=None;}}

#[derive(Debug, Clone)]
pub struct UlcerIndex { values: VecDeque<f64>, period: usize, value: Option<f64> }
impl UlcerIndex { pub fn new(period:usize)->TaResult<Self>{validate_period(period)?;Ok(Self{values:VecDeque::with_capacity(period),period,value:None})}pub fn append(&mut self,input:f64)->Option<f64>{if self.values.len()==self.period{self.values.pop_front();}self.values.push_back(input);self.value=(self.values.len()==self.period).then(||{let mut peak=f64::NEG_INFINITY;let sum=self.values.iter().map(|&v|{peak=peak.max(v);let drawdown=if peak!=0.0{100.0*(v-peak)/peak}else{0.0};drawdown*drawdown}).sum::<f64>();(sum/self.period as f64).sqrt()});self.value}pub fn value(&self)->Option<f64>{self.value}pub fn reset(&mut self){self.values.clear();self.value=None;}}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KeltnerValue { pub upper: f64, pub middle: f64, pub lower: f64 }
#[derive(Debug, Clone)]
pub struct KeltnerChannels { period: usize, multiplier: f64, ema: Option<f64>, range_ema: Option<f64>, alpha: f64, value: Option<KeltnerValue> }
impl KeltnerChannels { pub fn new(period:usize,multiplier:f64)->TaResult<Self>{validate_period(period)?;Ok(Self{period,multiplier,ema:None,range_ema:None,alpha:2.0/(period as f64+1.0),value:None})}pub fn append(&mut self,high:f64,low:f64,close:f64)->Option<KeltnerValue>{let typical=(high+low+close)/3.0;let range=high-low;let ema=self.ema.map_or(typical,|v|v+self.alpha*(typical-v));let re=self.range_ema.map_or(range,|v|v+self.alpha*(range-v));self.ema=Some(ema);self.range_ema=Some(re);self.value=Some(KeltnerValue{upper:ema+self.multiplier*re,middle:ema,lower:ema-self.multiplier*re});self.value}pub fn value(&self)->Option<KeltnerValue>{self.value}pub fn reset(&mut self){self.ema=None;self.range_ema=None;self.value=None;}}

#[derive(Debug, Clone)]
pub struct ChaikinVolatility { period: usize, roc_period: usize, alpha: f64, ema: Option<f64>, history: VecDeque<f64>, value: Option<f64> }
impl ChaikinVolatility { pub fn new(period:usize,roc_period:usize)->TaResult<Self>{validate_period(period)?;validate_period(roc_period)?;Ok(Self{period,roc_period,alpha:2.0/(period as f64+1.0),ema:None,history:VecDeque::with_capacity(roc_period+1),value:None})}pub fn append(&mut self,high:f64,low:f64)->Option<f64>{let range=high-low;let ema=self.ema.map_or(range,|v|v+self.alpha*(range-v));self.ema=Some(ema);if self.history.len()==self.roc_period+1{self.history.pop_front();}self.history.push_back(ema);self.value=(self.history.len()==self.roc_period+1).then(||{let old=self.history.front().copied().unwrap();if old!=0.0{(ema-old)/old*100.0}else{0.0}});self.value}pub fn value(&self)->Option<f64>{self.value}pub fn reset(&mut self){self.ema=None;self.history.clear();self.value=None;}}

#[derive(Debug, Clone)]
pub struct Vwap { prices: VecDeque<f64>, volumes: VecDeque<f64>, period: usize, value: Option<f64> }
impl Vwap { pub fn new(period:usize)->TaResult<Self>{validate_period(period)?;Ok(Self{prices:VecDeque::with_capacity(period),volumes:VecDeque::with_capacity(period),period,value:None})}pub fn append(&mut self,high:f64,low:f64,close:f64,volume:f64)->Option<f64>{if self.prices.len()==self.period{self.prices.pop_front();self.volumes.pop_front();}self.prices.push_back((high+low+close)/3.0);self.volumes.push_back(volume);self.value=(self.prices.len()==self.period).then(||{let total=self.volumes.iter().sum::<f64>();if total!=0.0{self.prices.iter().zip(&self.volumes).map(|(&p,&v)|p*v).sum::<f64>()/total}else{0.0}});self.value}pub fn value(&self)->Option<f64>{self.value}pub fn reset(&mut self){self.prices.clear();self.volumes.clear();self.value=None;}}
#[derive(Debug, Clone)] pub struct ForceIndex { previous: Option<f64>, value: Option<f64> }
impl ForceIndex { pub fn new()->Self{Self{previous:None,value:None}}pub fn append(&mut self,close:f64,volume:f64)->Option<f64>{let previous=self.previous.replace(close)?;self.value=Some((close-previous)*volume);self.value}pub fn value(&self)->Option<f64>{self.value}pub fn reset(&mut self){self.previous=None;self.value=None;}}
impl Default for ForceIndex{fn default()->Self{Self::new()}}
#[derive(Debug, Clone)] pub struct EaseOfMovement { previous_midpoint: Option<f64>, value: Option<f64> }
impl EaseOfMovement { pub fn new()->Self{Self{previous_midpoint:None,value:None}}pub fn append(&mut self,high:f64,low:f64,volume:f64)->Option<f64>{let midpoint=(high+low)*0.5;let previous=self.previous_midpoint.replace(midpoint)?;self.value=Some(if volume!=0.0{(midpoint-previous)*(high-low)/volume}else{0.0});self.value}pub fn value(&self)->Option<f64>{self.value}pub fn reset(&mut self){self.previous_midpoint=None;self.value=None;}}
impl Default for EaseOfMovement{fn default()->Self{Self::new()}}

#[derive(Debug, Clone)]
pub struct SignalDelay { values: VecDeque<f64>, period: usize, value: Option<f64> }
impl SignalDelay {
    pub fn new(period: usize) -> TaResult<Self> { validate_period(period)?; Ok(Self { values: VecDeque::with_capacity(period), period, value: None }) }
    pub fn append(&mut self, input: f64) -> Option<f64> {
        self.value = if self.values.len() == self.period { let value = self.values.pop_front(); self.values.push_back(input); value } else { self.values.push_back(input); None };
        self.value
    }
    pub fn value(&self) -> Option<f64> { self.value }
    pub fn reset(&mut self) { self.values.clear(); self.value = None; }
}

#[derive(Debug, Clone)]
pub struct PositionHold { position: f64, value: Option<f64> }
impl PositionHold { pub fn new()->Self{Self{position:0.0,value:None}}pub fn append(&mut self,input:f64)->f64{if input!=0.0{self.position=input;}self.value=Some(self.position);self.position}pub fn value(&self)->Option<f64>{self.value}pub fn reset(&mut self){self.position=0.0;self.value=None;}}
impl Default for PositionHold{fn default()->Self{Self::new()}}
#[derive(Debug, Clone)] pub struct EntryExit { position:f64, value:Option<f64> }
impl EntryExit { pub fn new()->Self{Self{position:0.0,value:None}}pub fn append(&mut self,entry:bool,exit:bool)->f64{if entry&&!exit{self.position=1.0}else if exit&&!entry{self.position=-1.0}self.value=Some(self.position);self.position}pub fn value(&self)->Option<f64>{self.value}pub fn reset(&mut self){self.position=0.0;self.value=None;}}
impl Default for EntryExit{fn default()->Self{Self::new()}}

#[derive(Debug, Clone)]
pub struct Crossover { previous_left: Option<f64>, previous_right: Option<f64>, value: Option<f64> }
impl Crossover { pub fn new()->Self{Self{previous_left:None,previous_right:None,value:None}}pub fn append(&mut self,left:f64,right:f64)->f64{let value=match(self.previous_left,self.previous_right){(Some(pl),Some(pr)) if pl<=pr&&left>right=>1.0,_=>0.0};self.previous_left=Some(left);self.previous_right=Some(right);self.value=Some(value);value}pub fn value(&self)->Option<f64>{self.value}pub fn reset(&mut self){self.previous_left=None;self.previous_right=None;self.value=None;}}
impl Default for Crossover{fn default()->Self{Self::new()}}
#[derive(Debug, Clone)]
pub struct Crossunder { previous_left: Option<f64>, previous_right: Option<f64>, value: Option<f64> }
impl Crossunder { pub fn new()->Self{Self{previous_left:None,previous_right:None,value:None}}pub fn append(&mut self,left:f64,right:f64)->f64{let value=match(self.previous_left,self.previous_right){(Some(pl),Some(pr)) if pl>=pr&&left<right=>1.0,_=>0.0};self.previous_left=Some(left);self.previous_right=Some(right);self.value=Some(value);value}pub fn value(&self)->Option<f64>{self.value}pub fn reset(&mut self){self.previous_left=None;self.previous_right=None;self.value=None;}}
impl Default for Crossunder{fn default()->Self{Self::new()}}
#[derive(Debug, Clone)]
pub struct Cross { crossover:Crossover, crossunder:Crossunder, value:Option<f64> }
impl Cross { pub fn new()->Self{Self{crossover:Crossover::new(),crossunder:Crossunder::new(),value:None}}pub fn append(&mut self,left:f64,right:f64)->f64{let value=(self.crossover.append(left,right)+self.crossunder.append(left,right)).min(1.0);self.value=Some(value);value}pub fn value(&self)->Option<f64>{self.value}pub fn reset(&mut self){self.crossover.reset();self.crossunder.reset();self.value=None;}}
impl Default for Cross{fn default()->Self{Self::new()}}

macro_rules! direction_operator { ($name:ident,$predicate:expr)=>{#[derive(Debug,Clone)]pub struct $name{values:VecDeque<f64>,period:usize,value:Option<f64>}impl $name{pub fn new(period:usize)->TaResult<Self>{validate_period(period)?;Ok(Self{values:VecDeque::with_capacity(period+1),period,value:None})}pub fn append(&mut self,input:f64)->Option<f64>{if self.values.len()==self.period+1{self.values.pop_front();}self.values.push_back(input);self.value=(self.values.len()==self.period+1).then(||if $predicate(input,self.values.front().copied().unwrap()){1.0}else{0.0});self.value}pub fn value(&self)->Option<f64>{self.value}pub fn reset(&mut self){self.values.clear();self.value=None;}}};}
direction_operator!(Rising, |current:f64,previous:f64| current>previous);
direction_operator!(Falling, |current:f64,previous:f64| current<previous);

rolling_risk_operator!(RollingSharpe, |values: &VecDeque<f64>| {
    let average = mean(values);
    let variance = values.iter().map(|&value| (value - average).powi(2)).sum::<f64>() / values.len() as f64;
    if variance > 0.0 { average / variance.sqrt() } else { 0.0 }
});

rolling_risk_operator!(RollingSortino, |values: &VecDeque<f64>| {
    let average = mean(values);
    let downside = values.iter().map(|&value| value.min(0.0).powi(2)).sum::<f64>() / values.len() as f64;
    if downside > 0.0 { average / downside.sqrt() } else { 0.0 }
});

rolling_risk_operator!(RollingCalmar, |values: &VecDeque<f64>| {
    let average = mean(values);
    let mut peak = values[0];
    let mut drawdown: f64 = 0.0;
    for &value in values {
        peak = peak.max(value);
        drawdown = drawdown.min(if peak != 0.0 { value / peak - 1.0 } else { 0.0 });
    }
    if drawdown < 0.0 { average / -drawdown } else { 0.0 }
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn batch_and_stream_match() {
        let input = vec![2.0, 4.0, 1.0, 8.0, 2.0];
        assert_eq!(lag(&input, 2).unwrap()[2..], [2.0, 4.0, 1.0]);
        assert_eq!(cumsum(&input), vec![2.0, 6.0, 7.0, 15.0, 17.0]);
        assert_eq!(cumprod(&input), vec![2.0, 8.0, 8.0, 64.0, 128.0]);
        assert_eq!(cummax(&input), vec![2.0, 4.0, 4.0, 8.0, 8.0]);
        assert_eq!(cummin(&input), vec![2.0, 2.0, 1.0, 1.0, 1.0]);
        assert_eq!(drawdown(&input), vec![0.0, 0.0, -0.75, 0.0, -0.75]);
        let expected = log_return(&input, 2).unwrap();
        let mut state = LogReturn::new(2).unwrap();
        for (input, expected) in input.iter().zip(expected) {
            assert_eq!(state.append(*input).map(f64::to_bits), (!expected.is_nan()).then_some(expected.to_bits()));
        }
    }

    #[test]
    fn cumulative_states_reset() {
        let mut sum = Cumsum::new();
        let mut product = Cumprod::new();
        assert_eq!(sum.append(2.0), 2.0);
        assert_eq!(product.append(2.0), 2.0);
        sum.reset(); product.reset();
        assert_eq!(sum.append(3.0), 3.0);
        assert_eq!(product.append(3.0), 3.0);
    }

    #[test]
    fn rolling_statistics_match_batch_and_reset() {
        let input = vec![1.0, 4.0, 2.0, 2.0, 9.0, 4.0];
        let median = rolling_median(&input, 3).unwrap();
        let mode = rolling_mode(&input, 3).unwrap();
        assert!(median[0].is_nan() && median[1].is_nan());
        assert_eq!(&median[2..], &[2.0, 2.0, 2.0, 4.0]);
        assert!(mode[0].is_nan() && mode[1].is_nan());
        assert_eq!(&mode[2..], &[1.0, 2.0, 2.0, 2.0]);

        let mut state = RollingMedian::new(3).unwrap();
        for &value in &input { state.append(value); }
        state.reset();
        assert!(state.append(7.0).is_none());
    }

    #[test]
    fn rolling_distribution_operators_match_definitions() {
        let input = vec![1.0, 4.0, 2.0, 8.0];
        assert_eq!(rolling_quantile(&input, 3, 0.5).unwrap()[2..], [2.0, 4.0]);
        assert_eq!(rolling_percentile(&input, 3, 50.0).unwrap()[2..], [2.0, 4.0]);
        assert_eq!(rolling_rank(&input, 3).unwrap()[2..], [2.0 / 3.0, 1.0]);
        assert!((rolling_zscore(&input, 3).unwrap()[2] - (-0.2672612419)).abs() < 1e-9);
        assert_eq!(rolling_iqr(&input, 3).unwrap()[2], 1.5);
        assert!((rolling_cov(&input, &[2.0, 8.0, 4.0, 16.0], 3).unwrap()[2] - 28.0 / 9.0).abs() < 1e-12);
        assert_eq!(rolling_winsorize(&input, 3, 0.0, 0.5).unwrap()[2], 2.0);
        assert_eq!(ewm_var(&input, 2).unwrap()[0], 0.0);
        assert_eq!(ewm_std(&input, 2).unwrap()[0], 0.0);
    }

    #[test]
    fn quant_family_batch_and_stream_match() {
        let close = vec![100.0, 102.0, 101.0, 105.0, 107.0, 106.0];
        let volume = vec![1000.0, 1100.0, 900.0, 1200.0, 1300.0, 950.0];

        assert_eq!(
            ts_rank(&close, 3).unwrap().iter().map(|&x| x.to_bits()).collect::<Vec<_>>(),
            rolling_rank(&close, 3).unwrap().iter().map(|&x| x.to_bits()).collect::<Vec<_>>()
        );
        assert_eq!(
            decay_linear(&close, 3).unwrap().iter().map(|&x| x.to_bits()).collect::<Vec<_>>(),
            crate::overlap::wma(&close, 3).unwrap().iter().map(|&x| x.to_bits()).collect::<Vec<_>>()
        );
        assert_eq!(signedpower(&[2.0, -3.0, 0.5], 2.0), vec![4.0, -9.0, 0.25]);

        let adv_batch = adv(&close, &volume, 3).unwrap();
        let mut adv_state = Adv::new(3).unwrap();
        for ((close, volume), expected) in close.iter().zip(&volume).zip(&adv_batch) {
            assert_eq!(adv_state.append(*close, *volume).map(f64::to_bits), (!expected.is_nan()).then_some(expected.to_bits()));
        }

        let amihud_batch = amihud(&close, &volume, 3).unwrap();
        let mut amihud_state = Amihud::new(3).unwrap();
        for ((close, volume), expected) in close.iter().zip(&volume).zip(&amihud_batch) {
            assert_eq!(amihud_state.append(*close, *volume).map(f64::to_bits), (!expected.is_nan()).then_some(expected.to_bits()));
        }

        let spread_batch = roll_spread(&close, 3).unwrap();
        let mut spread_state = RollSpread::new(3).unwrap();
        for (price, expected) in close.iter().zip(&spread_batch) {
            assert_eq!(spread_state.append(*price).map(f64::to_bits), (!expected.is_nan()).then_some(expected.to_bits()));
        }

        let hl_batch = ou_half_life(&close, 3).unwrap();
        let mut hl_state = OuHalfLife::new(3).unwrap();
        for (price, expected) in close.iter().zip(&hl_batch) {
            assert_eq!(hl_state.append(*price).map(f64::to_bits), (!expected.is_nan()).then_some(expected.to_bits()));
        }

        let cusum_batch = cusum(&[0.5, -0.5, 2.0, -1.0], 1.0).unwrap();
        assert_eq!(cusum_batch, vec![0.0, 0.0, 1.0, 0.0]);

        assert_eq!(adv(&close, &volume[..5], 3), Err(TaError::LengthMismatch { expected: 6, got: 5 }));
    }

    #[test]
    fn spread_zscore_matches_hedge_ratio_composition() {
        let x = vec![10.0, 11.0, 9.0, 12.0, 13.0, 11.5];
        let y = vec![20.0, 22.0, 18.5, 23.0, 25.0, 22.0];
        let period = 4;

        let z = spread_zscore(&x, &y, period).unwrap();
        assert!(z[..period - 1].iter().all(|&value| value.is_nan()));

        let beta = hedge_ratio(&x, &y, period).unwrap();
        for i in period - 1..x.len() {
            let window_x = &x[i + 1 - period..=i];
            let window_y = &y[i + 1 - period..=i];
            let spreads: Vec<f64> = window_x.iter().zip(window_y).map(|(&x, &y)| y - beta[i] * x).collect();
            let mean = spreads.iter().sum::<f64>() / period as f64;
            let variance = spreads.iter().map(|&s| (s - mean).powi(2)).sum::<f64>() / period as f64;
            let expected = if variance > 0.0 { (spreads[period - 1] - mean) / variance.sqrt() } else { 0.0 };
            assert!((z[i] - expected).abs() < 1e-9, "index {i}");
        }

        let mut state = SpreadZscore::new(period).unwrap();
        let mut replayed = Vec::new();
        for (&x, &y) in x.iter().zip(&y) {
            replayed.push(state.append(x, y).unwrap_or(f64::NAN));
        }
        assert_eq!(
            replayed.iter().map(|&v| v.to_bits()).collect::<Vec<_>>(),
            z.iter().map(|&v| v.to_bits()).collect::<Vec<_>>()
        );
    }

    #[test]
    fn frac_diff_matches_reference_weights() {
        let d = 0.5;
        let threshold = 1e-3;
        let mut weights = vec![1.0];
        let mut k = 1usize;
        loop {
            let wk = -weights[k - 1] * (d - k as f64 + 1.0) / k as f64;
            if wk.abs() < threshold {
                break;
            }
            weights.push(wk);
            k += 1;
        }
        assert!(weights.len() > 2, "truncation should retain several weights");

        let input: Vec<f64> = (1..=200).map(|x| x as f64).collect();
        let output = frac_diff(&input, d, threshold).unwrap();
        let w = weights.len();
        assert!(output[..w - 1].iter().all(|&v| v.is_nan()));
        for i in w - 1..input.len() {
            let mut expected = 0.0;
            for (j, &weight) in weights.iter().enumerate() {
                expected += weight * input[i - j];
            }
            assert!((output[i] - expected).abs() < 1e-9, "index {i}");
        }

        let mut state = FracDiff::new(d, threshold).unwrap();
        let replayed: Vec<f64> = input.iter().map(|&v| state.append(v).unwrap_or(f64::NAN)).collect();
        assert_eq!(
            replayed.iter().map(|&v| v.to_bits()).collect::<Vec<_>>(),
            output.iter().map(|&v| v.to_bits()).collect::<Vec<_>>()
        );
    }

    #[test]
    fn frac_diff_rejects_bad_params() {
        assert!(FracDiff::new(0.0, 1e-5).is_err());
        assert!(FracDiff::new(0.5, 0.0).is_err());
        assert!(FracDiff::new(-1.0, 1e-5).is_err());
    }

    #[test]
    fn kalman_hedge_ratio_tracks_synthetic_beta() {
        let true_beta = 2.0;
        let x: Vec<f64> = (0..200).map(|i| i as f64 / 10.0).collect();
        let y: Vec<f64> = x.iter().map(|&x| 1.0 + true_beta * x).collect();

        let delta = 1e-4;
        let observation_variance = 1e-3;
        let beta = kalman_hedge_ratio(&x, &y, delta, observation_variance).unwrap();
        assert_eq!(beta.len(), x.len());
        assert!((beta[0] - 1.0).abs() < 1e-9);
        assert!((beta[beta.len() - 1] - true_beta).abs() < 0.1, "final beta {}", beta[beta.len() - 1]);

        let mut state = KalmanHedgeRatio::new(delta, observation_variance).unwrap();
        let replayed: Vec<f64> = x.iter().zip(&y).map(|(&x, &y)| state.append(x, y).unwrap_or(f64::NAN)).collect();
        assert_eq!(
            replayed.iter().map(|&v| v.to_bits()).collect::<Vec<_>>(),
            beta.iter().map(|&v| v.to_bits()).collect::<Vec<_>>()
        );
        assert!(state.alpha().unwrap().abs() < 2.0);
        assert!(state.innovation().is_some());
        assert!(state.std().unwrap() > 0.0);

        state.reset();
        assert!(state.append(1.0, 3.0).is_some());
        assert!(state.value().unwrap() > 1.0);
    }

    #[test]
    fn kalman_hedge_ratio_rejects_bad_params() {
        assert!(KalmanHedgeRatio::new(-0.1, 1.0).is_err());
        assert!(KalmanHedgeRatio::new(0.0, 0.0).is_err());
        assert_eq!(
            kalman_hedge_ratio(&[1.0, 2.0], &[1.0], 1e-4, 1e-3),
            Err(TaError::LengthMismatch { expected: 2, got: 1 })
        );
    }

    #[test]
    fn quant_family_rejects_bad_periods() {
        assert!(Adv::new(0).is_err());
        assert!(Amihud::new(0).is_err());
        assert!(RollSpread::new(0).is_err());
        assert!(OuHalfLife::new(0).is_err());
        assert!(Cusum::new(-1.0).is_err());
    }
}
