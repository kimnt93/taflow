# CandleHangingMan benchmark (`CDLHANGINGMAN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.026 | 38.26M | 0.019 | 51.98M | 0.051 | 1.95× | 2.65× |
| 10,000 | 0.199 | 50.26M | 0.214 | 46.83M | 0.198 | 0.99× | 0.93× |
| 100,000 | 2.086 | 47.94M | 2.095 | 47.74M | 1.637 | 0.78× | 0.78× |
| 1,000,000 | 20.257 | 49.36M | 19.567 | 51.11M | 16.108 | 0.80× | 0.82× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.076 | 0.111 | 1.46× |
| 1 | 5 | 0.363 | 0.530 | 1.46× |
| 1 | 10 | 0.618 | 0.995 | 1.61× |
| 10 | 1 | 0.057 | 0.095 | 1.67× |
| 10 | 5 | 0.267 | 0.517 | 1.94× |
| 10 | 10 | 0.634 | 1.011 | 1.59× |
| 100 | 1 | 0.078 | 0.099 | 1.27× |
| 100 | 5 | 0.292 | 0.485 | 1.66× |
| 100 | 10 | 0.642 | 1.042 | 1.62× |
| 1,000 | 1 | 0.086 | 0.107 | 1.25× |
| 1,000 | 5 | 0.334 | 0.550 | 1.65× |
| 1,000 | 10 | 0.659 | 1.291 | 1.96× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
