use crate::error::{TaError, TaResult};

#[derive(Debug, Clone, Copy, PartialEq)]
struct Zone {
    top: f64,
    bottom: f64,
    birth: usize,
    flags: u32,
}

/// Bounded active-zone storage for causal zone-based indicators.
#[derive(Debug, Clone)]
pub struct ActiveZoneList {
    zones: Vec<Zone>,
    capacity: usize,
    index: usize,
}

impl ActiveZoneList {
    /// Create a bounded, empty zone list.
    pub fn new(capacity: usize) -> TaResult<Self> {
        if capacity == 0 {
            return Err(TaError::InvalidParameter {
                name: "capacity",
                value: capacity.to_string(),
                reason: "must be >= 1",
            });
        }
        Ok(Self {
            zones: Vec::with_capacity(capacity),
            capacity,
            index: 0,
        })
    }

    /// Add a normalized zone and return its current index.
    pub fn add(&mut self, top: f64, bottom: f64, flags: u32) -> usize {
        if self.zones.len() == self.capacity {
            self.zones.remove(0);
        }
        let (top, bottom) = if top >= bottom {
            (top, bottom)
        } else {
            (bottom, top)
        };
        self.zones.push(Zone {
            top,
            bottom,
            birth: self.index,
            flags,
        });
        self.zones.len() - 1
    }

    /// Advance one bar, returning flags for zones mitigated on this bar.
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

    /// Return the number of currently active zones.
    pub fn zone_count(&self) -> usize {
        self.zones.len()
    }

    /// Reset the list while retaining its allocation.
    pub fn reset(&mut self) {
        self.zones.clear();
        self.index = 0;
    }
}
