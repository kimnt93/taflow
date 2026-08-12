# CandleInvertedHammer benchmark (`CDLINVERTEDHAMMER` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 52.55M | 0.016 | 63.51M | 0.044 | 2.31× | 2.80× |
| 10,000 | 0.160 | 62.42M | 0.174 | 57.59M | 0.256 | 1.60× | 1.47× |
| 100,000 | 1.651 | 60.56M | 1.602 | 62.42M | 1.557 | 0.94× | 0.97× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.076 | 0.100 | 1.32× |
| 1 | 5 | 0.343 | 0.485 | 1.42× |
| 1 | 10 | 0.589 | 0.951 | 1.61× |
| 10 | 1 | 0.060 | 0.103 | 1.72× |
| 10 | 5 | 0.269 | 0.534 | 1.98× |
| 10 | 10 | 0.677 | 0.992 | 1.47× |
| 100 | 1 | 0.057 | 0.093 | 1.63× |
| 100 | 5 | 0.267 | 0.508 | 1.90× |
| 100 | 10 | 0.649 | 1.008 | 1.55× |
| 1,000 | 1 | 0.079 | 0.110 | 1.40× |
| 1,000 | 5 | 0.288 | 0.527 | 1.83× |
| 1,000 | 10 | 0.593 | 1.167 | 1.97× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
