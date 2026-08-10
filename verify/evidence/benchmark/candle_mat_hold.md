# CandleMatHold benchmark (`CDLMATHOLD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.024 | 41.70M | 0.020 | 49.58M | 0.041 | 1.71× | 2.03× |
| 10,000 | 0.196 | 50.95M | 0.191 | 52.40M | 0.127 | 0.65× | 0.67× |
| 100,000 | 1.982 | 50.44M | 1.930 | 51.83M | 0.931 | 0.47× | 0.48× |
| 1,000,000 | 18.761 | 53.30M | 19.821 | 50.45M | 9.026 | 0.48× | 0.46× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.132 | 0.149 | 1.13× |
| 1 | 5 | 0.290 | 0.473 | 1.63× |
| 1 | 10 | 0.547 | 1.044 | 1.91× |
| 10 | 1 | 0.065 | 0.093 | 1.42× |
| 10 | 5 | 0.281 | 0.504 | 1.79× |
| 10 | 10 | 0.571 | 0.974 | 1.71× |
| 100 | 1 | 0.054 | 0.102 | 1.90× |
| 100 | 5 | 0.284 | 0.506 | 1.78× |
| 100 | 10 | 0.561 | 1.011 | 1.80× |
| 1,000 | 1 | 0.072 | 0.116 | 1.63× |
| 1,000 | 5 | 0.289 | 0.544 | 1.88× |
| 1,000 | 10 | 0.631 | 1.187 | 1.88× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
