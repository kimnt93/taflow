//! Mathematical transform tests; each public transform has its own stream file.

#[cfg(test)]
mod tests {
    use super::super::{acos, cos, sin, sqrt};

    #[test]
    fn test_sqrt() {
        let input = vec![1.0, 4.0, 9.0, 16.0];
        let result = sqrt(&input);
        assert!((result[0] - 1.0).abs() < 1e-10);
        assert!((result[1] - 2.0).abs() < 1e-10);
        assert!((result[2] - 3.0).abs() < 1e-10);
        assert!((result[3] - 4.0).abs() < 1e-10);
    }

    #[test]
    fn test_sin_cos_identity() {
        let input: Vec<f64> = (0..100).map(|i| i as f64 * 0.1).collect();
        let s = sin(&input);
        let c = cos(&input);
        for i in 0..100 {
            let sum = s[i] * s[i] + c[i] * c[i];
            assert!((sum - 1.0).abs() < 1e-10, "sin²+cos²≠1 at {}", i);
        }
    }
}
