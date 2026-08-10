# CandleUpsideGapTwoCrows benchmark (`CDLUPSIDEGAP2CROWS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 53.05M | 0.015 | 66.40M | 0.032 | 1.72× | 2.15× |
| 10,000 | 0.133 | 75.11M | 0.133 | 75.42M | 0.124 | 0.93× | 0.93× |
| 100,000 | 1.237 | 80.86M | 1.242 | 80.50M | 1.011 | 0.82× | 0.81× |
| 1,000,000 | 13.129 | 76.17M | 12.929 | 77.34M | 9.392 | 0.72× | 0.73× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.115 | 0.113 | 0.98× |
| 1 | 5 | 0.355 | 0.458 | 1.29× |
| 1 | 10 | 0.538 | 0.884 | 1.64× |
| 10 | 1 | 0.054 | 0.086 | 1.61× |
| 10 | 5 | 0.239 | 0.423 | 1.77× |
| 10 | 10 | 0.507 | 0.896 | 1.77× |
| 100 | 1 | 0.053 | 0.094 | 1.75× |
| 100 | 5 | 0.256 | 0.421 | 1.65× |
| 100 | 10 | 0.528 | 0.912 | 1.73× |
| 1,000 | 1 | 0.072 | 0.097 | 1.35× |
| 1,000 | 5 | 0.280 | 0.478 | 1.71× |
| 1,000 | 10 | 0.567 | 1.006 | 1.77× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
