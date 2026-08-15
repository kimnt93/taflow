# CandleIdenticalThreeCrows benchmark (`CDLIDENTICAL3CROWS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 156.89M | 0.003 | 321.46M | 0.037 | 5.75× | 11.79× |
| 10,000 | 0.047 | 213.01M | 0.044 | 225.83M | 0.123 | 2.63× | 2.78× |
| 100,000 | 0.651 | 153.56M | 0.598 | 167.30M | 0.905 | 1.39× | 1.51× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.142 | 0.153 | 1.08× |
| 1 | 5 | 0.261 | 0.444 | 1.70× |
| 1 | 10 | 0.428 | 0.979 | 2.29× |
| 10 | 1 | 0.046 | 0.090 | 1.94× |
| 10 | 5 | 0.183 | 0.424 | 2.32× |
| 10 | 10 | 0.396 | 0.889 | 2.24× |
| 100 | 1 | 0.039 | 0.087 | 2.21× |
| 100 | 5 | 0.298 | 0.595 | 2.00× |
| 100 | 10 | 0.400 | 0.906 | 2.27× |
| 1,000 | 1 | 0.050 | 0.101 | 2.01× |
| 1,000 | 5 | 0.191 | 0.480 | 2.52× |
| 1,000 | 10 | 0.451 | 1.008 | 2.23× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
