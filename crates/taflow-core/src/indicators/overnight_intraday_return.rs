use crate::error::TaResult;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OvernightIntradayReturnValue {
    pub overnight: f64,
    pub intraday: f64,
}
#[derive(Debug, Clone)]
pub struct OvernightIntradayReturn {
    previous_close: Option<f64>,
    value: Option<OvernightIntradayReturnValue>,
}
impl OvernightIntradayReturn {
    pub fn new(_utc_offset_minutes: i32) -> TaResult<Self> {
        Ok(Self {
            previous_close: None,
            value: None,
        })
    }
    pub fn append(
        &mut self,
        open: f64,
        _high: f64,
        _low: f64,
        close: f64,
        _volume: f64,
        _timestamp: i64,
    ) -> Option<OvernightIntradayReturnValue> {
        self.value = self.previous_close.filter(|x| *x != 0.0).map(|previous| {
            OvernightIntradayReturnValue {
                overnight: open / previous - 1.0,
                intraday: if open == 0.0 {
                    f64::NAN
                } else {
                    close / open - 1.0
                },
            }
        });
        self.previous_close = Some(close);
        self.value
    }
    pub fn value(&self) -> Option<OvernightIntradayReturnValue> {
        self.value
    }
    pub fn reset(&mut self) {
        self.previous_close = None;
        self.value = None;
    }
}
