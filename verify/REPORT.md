# taflow correctness verification

Date: 2026-08-07 | bars: 2,000 | warm-up split: 1,500 + 500 continue | tolerance rtol=1e-08, atol=1e-10
Environment: python 3.12.3, numpy 2.5.1; TA-Lib batch migration disabled; taflow 0.1.2

Summary: MATCH: 8

Columns — *continue vs oracle*: the stitched persistent output
against the reference. Batch TA-Lib compatibility is not part of
the continuous-only package contract.

| Function | Oracle | Verdict | Batch vs oracle | Continue vs batch (bitwise) | Continue vs oracle |
|---|---|---|---|---|---|
| ewm_std | pandas | MATCH | pass (err 3.9e-14, nan 0) | yes | pass (err 3.9e-14, nan 0) |
| ewm_var | pandas | MATCH | pass (err 4.1e-13, nan 0) | yes | pass (err 4.1e-13, nan 0) |
| rolling_cov | pandas | MATCH | pass (err 5.8e-12, nan 0) | yes | pass (err 5.8e-12, nan 0) |
| rolling_kurtosis | pandas | MATCH | pass (err 1.3e-15, nan 0) | yes | pass (err 1.3e-15, nan 0) |
| rolling_median | pandas | MATCH | pass (err 0.0e+00, nan 0) | yes | pass (err 0.0e+00, nan 0) |
| rolling_quantile | pandas | MATCH | pass (err 0.0e+00, nan 0) | yes | pass (err 0.0e+00, nan 0) |
| rolling_skew | pandas | MATCH | pass (err 4.4e-16, nan 0) | yes | pass (err 4.4e-16, nan 0) |
| rolling_zscore | pandas | MATCH | pass (err 3.3e-10, nan 0) | yes | pass (err 3.3e-10, nan 0) |

## Follow-ups

- Mismatches: none
- Errors: none
