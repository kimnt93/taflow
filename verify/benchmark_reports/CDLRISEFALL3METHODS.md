# CandleRiseFallThreeMethods benchmark (`CDLRISEFALL3METHODS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 85.70M | 0.010 | 101.93M | 0.034 | 2.92× | 3.48× |
| 10,000 | 0.094 | 105.83M | 0.090 | 110.87M | 0.119 | 1.26× | 1.32× |
| 100,000 | 0.990 | 100.99M | 0.975 | 102.59M | 0.903 | 0.91× | 0.93× |
| 1,000,000 | 10.002 | 99.98M | 10.494 | 95.30M | 9.665 | 0.97× | 0.92× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.130 | 0.214 | 1.64× |
| 1 | 5 | 0.428 | 0.857 | 2.00× |
| 1 | 10 | 0.549 | 1.002 | 1.82× |
| 10 | 1 | 0.059 | 0.091 | 1.55× |
| 10 | 5 | 0.246 | 0.437 | 1.77× |
| 10 | 10 | 0.500 | 0.937 | 1.87× |
| 100 | 1 | 0.055 | 0.101 | 1.83× |
| 100 | 5 | 0.251 | 0.432 | 1.73× |
| 100 | 10 | 0.566 | 0.982 | 1.74× |
| 1,000 | 1 | 0.069 | 0.102 | 1.47× |
| 1,000 | 5 | 0.251 | 0.495 | 1.97× |
| 1,000 | 10 | 0.533 | 1.021 | 1.92× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
