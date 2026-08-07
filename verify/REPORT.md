# taflow correctness verification

Date: 2026-08-07 | bars: 10,000 | warm-up split: 9,000 + 1,000 continue | tolerance rtol=1e-08, atol=1e-10
Environment: python 3.12.3, numpy 2.5.1, TA-Lib 0.7.1, taflow 0.1.2

Summary: MATCH: 2, MISMATCH: 2

Columns — *batch vs oracle*: full-series batch against the
reference; *continue vs batch*: 9k `extend` + 1k `append` stitched
output bitwise-identical to the one-shot batch (chunk-invariance
contract); *continue vs oracle*: the stitched output against the
reference.

| Function | Oracle | Verdict | Batch vs oracle | Continue vs batch (bitwise) | Continue vs oracle |
|---|---|---|---|---|---|
| CDLLADDERBOTTOM | TA-Lib | MISMATCH | **FAIL** (err 1.0e+02, nan 0) | — | — |
| CDLTRISTAR | TA-Lib | MISMATCH | **FAIL** (err 1.0e+02, nan 0) | — | — |
| CDLHIKKAKEMOD | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | — | — |
| CDLUNIQUE3RIVER | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | — | — |

## Follow-ups

- Mismatches: CDLLADDERBOTTOM, CDLTRISTAR
- Errors: none
