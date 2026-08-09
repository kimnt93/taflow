# CandleIdenticalThreeCrows benchmark (`CDLIDENTICAL3CROWS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 93.86M | 0.008 | 129.31M | 0.035 | 3.24× | 4.47× |
| 10,000 | 0.065 | 154.76M | 0.060 | 166.12M | 0.116 | 1.79× | 1.92× |
| 100,000 | 0.805 | 124.26M | 0.762 | 131.27M | 0.860 | 1.07× | 1.13× |
| 1,000,000 | 8.263 | 121.02M | 7.889 | 126.77M | 8.731 | 1.06× | 1.11× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.109 | 0.162 | 1.48× |
| 1 | 5 | 0.303 | 0.512 | 1.69× |
| 1 | 10 | 0.530 | 0.996 | 1.88× |
| 10 | 1 | 0.053 | 0.098 | 1.84× |
| 10 | 5 | 0.237 | 0.431 | 1.81× |
| 10 | 10 | 0.508 | 0.929 | 1.83× |
| 100 | 1 | 0.056 | 0.099 | 1.78× |
| 100 | 5 | 0.244 | 0.439 | 1.80× |
| 100 | 10 | 0.548 | 0.978 | 1.79× |
| 1,000 | 1 | 0.062 | 0.101 | 1.63× |
| 1,000 | 5 | 0.249 | 0.494 | 1.98× |
| 1,000 | 10 | 0.541 | 1.030 | 1.90× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
