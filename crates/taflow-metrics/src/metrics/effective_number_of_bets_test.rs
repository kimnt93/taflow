use super::effective_number_of_bets::EffectiveNumberOfBets;
use crate::NanPolicy;

#[test]
fn distribution_entropy_has_expected_boundaries() {
    let mut equal = EffectiveNumberOfBets::new(NanPolicy::Omit).unwrap();
    equal.from_risk_contributions(&[]).unwrap();
    equal.extend(&[1.0, 1.0, 1.0, 1.0]).unwrap();
    assert!((equal.compute().unwrap() - 4.0).abs() < 1e-14);
    let mut concentrated = EffectiveNumberOfBets::new(NanPolicy::Omit).unwrap();
    concentrated.from_risk_contributions(&[]).unwrap();
    concentrated.extend(&[1.0, 0.0, 0.0]).unwrap();
    assert_eq!(concentrated.compute(), Some(1.0));
}

#[test]
fn identity_covariance_matches_weight_squared_distribution() {
    let weights = [0.5, 0.5];
    let covariance = [1.0, 0.0, 0.0, 1.0];
    let mut metric = EffectiveNumberOfBets::new(NanPolicy::Omit).unwrap();
    metric
        .from_weights_and_covariance(&weights, &covariance)
        .unwrap();
    assert!((metric.compute().unwrap() - 2.0).abs() < 1e-14);
}

#[test]
fn correlated_covariance_matches_independent_pca_oracle() {
    let weights = [0.6, 0.4];
    let covariance = [0.04, 0.012, 0.012, 0.09];
    let mut metric = EffectiveNumberOfBets::new(NanPolicy::Omit).unwrap();
    metric
        .from_weights_and_covariance(&weights, &covariance)
        .unwrap();
    assert!((metric.compute().unwrap() - 1.783_962_996_790_890_5).abs() < 1e-12);
}

#[test]
fn lifecycle_and_validation_are_stable() {
    let mut metric = EffectiveNumberOfBets::new(NanPolicy::Omit).unwrap();
    metric.from_risk_contributions(&[]).unwrap();
    metric.extend(&[1.0, f64::NAN, 2.0]).unwrap();
    let expected = metric.compute();
    assert_eq!(metric.len(), 2);
    assert_eq!(metric.compute(), expected);
    metric.reset();
    assert!(metric.is_empty());
    assert_eq!(metric.extend(&[1.0, 2.0]).unwrap(), expected);
    assert!(metric.append(-1.0).is_err());
    let mut invalid = EffectiveNumberOfBets::new(NanPolicy::Omit).unwrap();
    assert!(invalid
        .from_weights_and_covariance(&[0.5, 0.5], &[1.0, 2.0, 0.0, 1.0])
        .is_err());
}
