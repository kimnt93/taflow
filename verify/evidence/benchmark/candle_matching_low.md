# CandleMatchingLow benchmark (`CDLMATCHINGLOW` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.016 | 62.17M | 0.013 | 75.16M | 0.033 | 2.03× | 2.45× |
| 10,000 | 0.105 | 95.10M | 0.099 | 100.72M | 0.090 | 0.86× | 0.91× |
| 100,000 | 0.965 | 103.68M | 0.945 | 105.83M | 0.630 | 0.65× | 0.67× |
| 1,000,000 | 10.057 | 99.43M | 9.685 | 103.26M | 6.867 | 0.68× | 0.71× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.081 | 0.144 | 1.78× |
| 1 | 5 | 0.356 | 0.471 | 1.32× |
| 1 | 10 | 0.563 | 0.912 | 1.62× |
| 10 | 1 | 0.054 | 0.086 | 1.61× |
| 10 | 5 | 0.251 | 0.441 | 1.76× |
| 10 | 10 | 0.546 | 0.911 | 1.67× |
| 100 | 1 | 0.057 | 0.086 | 1.52× |
| 100 | 5 | 0.251 | 0.428 | 1.71× |
| 100 | 10 | 0.535 | 0.920 | 1.72× |
| 1,000 | 1 | 0.064 | 0.099 | 1.55× |
| 1,000 | 5 | 0.326 | 0.548 | 1.68× |
| 1,000 | 10 | 0.596 | 1.021 | 1.71× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
