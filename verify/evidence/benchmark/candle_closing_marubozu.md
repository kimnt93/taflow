# CandleClosingMarubozu benchmark (`CDLCLOSINGMARUBOZU` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.024 | 41.14M | 0.021 | 48.27M | 0.039 | 1.60× | 1.88× |
| 10,000 | 0.164 | 61.15M | 0.156 | 64.22M | 0.134 | 0.82× | 0.86× |
| 100,000 | 1.564 | 63.93M | 1.602 | 62.40M | 1.079 | 0.69× | 0.67× |
| 1,000,000 | 16.460 | 60.75M | 15.674 | 63.80M | 11.251 | 0.68× | 0.72× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.105 | 0.154 | 1.47× |
| 1 | 5 | 0.296 | 0.518 | 1.75× |
| 1 | 10 | 0.642 | 0.972 | 1.52× |
| 10 | 1 | 0.057 | 0.090 | 1.56× |
| 10 | 5 | 0.271 | 0.494 | 1.82× |
| 10 | 10 | 0.687 | 0.995 | 1.45× |
| 100 | 1 | 0.056 | 0.091 | 1.61× |
| 100 | 5 | 0.283 | 0.460 | 1.62× |
| 100 | 10 | 0.650 | 1.047 | 1.61× |
| 1,000 | 1 | 0.075 | 0.098 | 1.31× |
| 1,000 | 5 | 0.287 | 0.512 | 1.78× |
| 1,000 | 10 | 0.654 | 1.183 | 1.81× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
