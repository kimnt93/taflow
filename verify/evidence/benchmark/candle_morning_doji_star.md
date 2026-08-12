# CandleMorningDojiStar benchmark (`CDLMORNINGDOJISTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 48.83M | 0.016 | 60.93M | 0.040 | 1.95× | 2.44× |
| 10,000 | 0.160 | 62.56M | 0.181 | 55.16M | 0.115 | 0.72× | 0.64× |
| 100,000 | 1.661 | 60.20M | 1.606 | 62.27M | 0.999 | 0.60× | 0.62× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.119 | 0.158 | 1.33× |
| 1 | 5 | 0.337 | 0.498 | 1.48× |
| 1 | 10 | 0.606 | 0.971 | 1.60× |
| 10 | 1 | 0.060 | 0.095 | 1.59× |
| 10 | 5 | 0.247 | 0.469 | 1.90× |
| 10 | 10 | 0.546 | 1.065 | 1.95× |
| 100 | 1 | 0.058 | 0.099 | 1.70× |
| 100 | 5 | 0.265 | 0.473 | 1.79× |
| 100 | 10 | 0.532 | 1.057 | 1.99× |
| 1,000 | 1 | 0.081 | 0.120 | 1.47× |
| 1,000 | 5 | 0.278 | 0.543 | 1.95× |
| 1,000 | 10 | 0.589 | 1.056 | 1.79× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
