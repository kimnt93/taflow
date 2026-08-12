# CandleThreeStarsInSouth benchmark (`CDL3STARSINSOUTH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 88.99M | 0.009 | 117.02M | 0.032 | 2.84× | 3.73× |
| 10,000 | 0.068 | 145.99M | 0.063 | 159.19M | 0.105 | 1.53× | 1.67× |
| 100,000 | 0.640 | 156.18M | 0.640 | 156.14M | 0.829 | 1.29× | 1.29× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.118 | 0.097 | 0.82× |
| 1 | 5 | 0.336 | 0.444 | 1.32× |
| 1 | 10 | 0.514 | 0.897 | 1.75× |
| 10 | 1 | 0.054 | 0.089 | 1.66× |
| 10 | 5 | 0.243 | 0.471 | 1.94× |
| 10 | 10 | 0.547 | 0.895 | 1.64× |
| 100 | 1 | 0.056 | 0.087 | 1.54× |
| 100 | 5 | 0.253 | 0.421 | 1.67× |
| 100 | 10 | 0.533 | 0.909 | 1.71× |
| 1,000 | 1 | 0.061 | 0.101 | 1.67× |
| 1,000 | 5 | 0.261 | 0.483 | 1.85× |
| 1,000 | 10 | 0.573 | 1.001 | 1.75× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
