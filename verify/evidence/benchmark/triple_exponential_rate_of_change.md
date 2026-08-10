# TripleExponentialRateOfChange benchmark (`TRIX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 168.49M | 0.005 | 195.30M | 0.038 | 6.35× | 7.37× |
| 10,000 | 0.028 | 356.78M | 0.024 | 408.47M | 0.129 | 4.61× | 5.28× |
| 100,000 | 0.238 | 420.37M | 0.216 | 462.22M | 0.934 | 3.93× | 4.32× |
| 1,000,000 | 2.602 | 384.29M | 2.215 | 451.43M | 9.108 | 3.50× | 4.11× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.090 | 0.150 | 1.67× |
| 1 | 5 | 0.275 | 0.476 | 1.73× |
| 1 | 10 | 0.483 | 0.919 | 1.90× |
| 10 | 1 | 0.047 | 0.088 | 1.89× |
| 10 | 5 | 0.207 | 0.438 | 2.11× |
| 10 | 10 | 0.481 | 0.928 | 1.93× |
| 100 | 1 | 0.048 | 0.098 | 2.03× |
| 100 | 5 | 0.215 | 0.444 | 2.07× |
| 100 | 10 | 0.468 | 0.923 | 1.97× |
| 1,000 | 1 | 0.052 | 0.094 | 1.80× |
| 1,000 | 5 | 0.236 | 0.464 | 1.97× |
| 1,000 | 10 | 0.494 | 1.032 | 2.09× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
