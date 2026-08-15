# CandleUpsideGapTwoCrows benchmark (`CDLUPSIDEGAP2CROWS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 72.58M | 0.010 | 98.12M | 0.034 | 2.44× | 3.30× |
| 10,000 | 0.129 | 77.71M | 0.126 | 79.54M | 0.122 | 0.95× | 0.97× |
| 100,000 | 1.239 | 80.69M | 1.227 | 81.52M | 1.010 | 0.82× | 0.82× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.143 | 0.117 | 0.82× |
| 1 | 5 | 0.306 | 0.457 | 1.49× |
| 1 | 10 | 0.381 | 0.949 | 2.49× |
| 10 | 1 | 0.109 | 0.094 | 0.86× |
| 10 | 5 | 0.188 | 0.422 | 2.24× |
| 10 | 10 | 0.379 | 0.890 | 2.35× |
| 100 | 1 | 0.041 | 0.092 | 2.22× |
| 100 | 5 | 0.192 | 0.459 | 2.39× |
| 100 | 10 | 0.438 | 0.907 | 2.07× |
| 1,000 | 1 | 0.056 | 0.100 | 1.79× |
| 1,000 | 5 | 0.205 | 0.491 | 2.39× |
| 1,000 | 10 | 0.462 | 1.024 | 2.21× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
