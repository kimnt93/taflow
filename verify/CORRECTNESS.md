# TAFlow correctness verification

Date: 2026-08-15 | bars: 10,000 | warm-up split: 9,000 extend + 1,000 append | tolerance rtol=1e-08, atol=1e-10
Environment: python 3.12.3, numpy 2.4.6, TA-Lib 0.7.1, Wickra 0.9.9, pandas-ta-classic 0.6.52, SMC 0.0.27, TAFlow 0.1.2

Summary: MATCH: 10

TAFlow is driven only through canonical Python classes. The registry
selects TA-Lib, Wickra, pandas-ta-classic, explicit NumPy formula oracles, then SMC.
*Batch vs oracle*:
cold `extend` over the full series against the reference;
*continue vs batch*: 9k `extend` + 1k `append` stitched output
bitwise-identical to one-shot batch (chunk invariance); *continue
vs oracle*: the stitched output against the reference. Repeated
native `extend` chunks [1, 10, 1000] are also checked bitwise.

| **Class** | **Target** | **Verdict** | **Batch vs oracle** | **Continue vs oracle** |
|---|---|---|---|---|
| AccumulationDistribution | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | pass (err 0.0e+00, nan 0) |
| CandleAbandonedBaby | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | pass (err 0.0e+00, nan 0) |
| CandleConcealBabySwall | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | pass (err 0.0e+00, nan 0) |
| CandleEveningDojiStar | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | pass (err 0.0e+00, nan 0) |
| CandleLadderBottom | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | pass (err 0.0e+00, nan 0) |
| CandleMatHold | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | pass (err 0.0e+00, nan 0) |
| CandleMorningDojiStar | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | pass (err 0.0e+00, nan 0) |
| CandleRiseFallThreeMethods | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | pass (err 0.0e+00, nan 0) |
| CandleTriStar | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | pass (err 0.0e+00, nan 0) |
| CandleUniqueThreeRiver | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | pass (err 0.0e+00, nan 0) |

## Follow-ups

- Mismatches: none
- Errors (class/mapping/runtime): none
- Compared at TA-Lib defaults only (unmapped params): CDLMATHOLD, CDLMORNINGDOJISTAR, CDLEVENINGDOJISTAR, CDLABANDONEDBABY
