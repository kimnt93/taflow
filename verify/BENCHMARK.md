# TAFlow benchmark

Generated 2026-08-10 with Python 3.12.3, NumPy 2.4.6, TA-Lib 0.7.1, Wickra 0.9.9, SMC 0.0.27, and TAFlow 0.1.2.

Only `MATCH` indicators are timed. Speedup is reference time divided by TAFlow time; values above 1× favor TAFlow. Each cell is API/kernel.

| Class | Target | 1k | 10k | 100k | 1m |
|---|---|---:|---:|---:|---:|
| CandleHikkake | TA-Lib `CDLHIKKAKE` | 2.89×/4.03× | 1.32×/1.43× | — | — |
| FairValueGap | SMC `smartmoneyconcepts.smc.fvg` | 273.94×/342.72× | 45.59×/71.83× | — | — |
| MathAbs | NumPy `numpy.abs` | 0.35×/0.83× | 0.56×/0.53× | — | — |
| RollingOmegaRatio | Wickra `OmegaRatio` | 2.46×/2.53× | 1.93×/1.94× | — | — |
| Sessions | SMC `smartmoneyconcepts.smc.sessions` | 6561.64×/8252.56× | 10310.47×/12252.42× | — | — |

Complete vector and warm-up/thread tables plus raw samples are stored under `verify/evidence/benchmark/`.
