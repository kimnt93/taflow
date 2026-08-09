use std::collections::VecDeque;

#[derive(Debug, Clone, Copy)]
struct FvgZone {
    direction: f64,
    top: f64,
    bottom: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FairValueGapValue {
    pub signal: f64,
    pub top: f64,
    pub bottom: f64,
    pub mitigated: f64,
}

#[derive(Debug, Clone, Default)]
/// Causal fair-value-gap detection with directional mitigation events.
pub struct FairValueGap {
    bars: VecDeque<(f64, f64, f64, f64)>,
    zones: Vec<FvgZone>,
    value: Option<FairValueGapValue>,
}

impl FairValueGap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn append(
        &mut self,
        open: f64,
        high: f64,
        low: f64,
        close: f64,
    ) -> Option<FairValueGapValue> {
        let mut mitigated = f64::NAN;
        self.zones.retain(|zone| {
            let filled = (zone.direction > 0.0 && low <= zone.top)
                || (zone.direction < 0.0 && high >= zone.bottom);
            if filled {
                mitigated = zone.direction;
            }
            !filled
        });
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
                self.zones.push(FvgZone {
                    direction: signal,
                    top,
                    bottom,
                });
            } else if old_low > high && middle_close < middle_open {
                signal = -1.0;
                top = old_low;
                bottom = high;
                self.zones.push(FvgZone {
                    direction: signal,
                    top,
                    bottom,
                });
            }
        }
        if self.bars.len() == 2 {
            self.bars.pop_front();
        }
        self.bars.push_back((open, high, low, close));
        let value = FairValueGapValue {
            signal,
            top,
            bottom,
            mitigated,
        };
        self.value = Some(value);
        Some(value)
    }

    pub fn value(&self) -> Option<FairValueGapValue> {
        self.value
    }

    pub fn reset(&mut self) {
        self.bars.clear();
        self.zones.clear();
        self.value = None;
    }
}
