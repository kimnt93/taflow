use crate::error::TaResult;
use super::pattern_swing::{approximately_equal, ratios_in, SwingTracker, SWING_THRESHOLD};

/// Four-point AB=CD harmonic completion signal.
#[derive(Debug, Clone)]
pub struct FourPointHarmonicPattern { swing: SwingTracker, count: usize, value: Option<f64> }
impl FourPointHarmonicPattern {
    /// Create a detector retaining four confirmed pivots.
    pub fn new()->TaResult<Self>{Ok(Self{swing:SwingTracker::new(SWING_THRESHOLD,4),count:0,value:None})}
    /// Append one OHLC bar and return the latest harmonic signal.
    pub fn append(&mut self,_open:f64,high:f64,low:f64,_close:f64)->Option<f64>{
        self.count+=1;self.value=Some(0.0);
        if !self.swing.append(high,low)||self.swing.pivots().len()<4{return self.value;}
        let p=self.swing.pivots();let n=p.len();let ab=(p[n-3].price-p[n-4].price).abs();let bc=(p[n-2].price-p[n-3].price).abs();let cd=(p[n-1].price-p[n-2].price).abs();
        if ratios_in(&[(bc/ab,.382,.886),(cd/bc,1.13,2.618)])&&approximately_equal(ab,cd,.10){self.value=Some(if p[n-1].direction<0.0{1.0}else{-1.0});}
        self.value
    }
    /// Return the latest signal.
    pub fn value(&self)->Option<f64>{self.value}
    /// Return the processed-bar count.
    pub fn len(&self)->usize{self.count}
    /// Return whether no bars were processed.
    pub fn is_empty(&self)->bool{self.count==0}
    /// Clear pivots and output.
    pub fn reset(&mut self){self.swing.reset();self.count=0;self.value=None;}
}
