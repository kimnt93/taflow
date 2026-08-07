use std::collections::VecDeque;

use crate::error::{TaError, TaResult};

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
}
