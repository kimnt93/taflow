pub use crate::stream::{accumulation_distribution, accumulation_distribution_oscillator};

/// Chaikin A/D is implemented by the stream module; this module re-exports it.

/// Compatibility export for the stream-owned On-Balance Volume kernel.
pub use crate::stream::on_balance_volume;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_obv_basic() {
        let close = vec![1.0, 2.0, 1.5, 3.0, 2.5];
        let volume = vec![100.0, 200.0, 150.0, 300.0, 250.0];
        let result = on_balance_volume(&close, &volume).unwrap();
        assert!((result[0] - 100.0).abs() < 1e-10);
        assert!((result[1] - 300.0).abs() < 1e-10);
        assert!((result[2] - 150.0).abs() < 1e-10);
    }
}
