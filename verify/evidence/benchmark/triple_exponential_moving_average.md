# TripleExponentialMovingAverage benchmark (`TEMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.067 | 14.87M | 0.053 | 18.85M | 0.041 | 0.61× | 0.77× |
| 10,000 | 0.477 | 20.95M | 0.489 | 20.44M | 0.114 | 0.24× | 0.23× |
| 100,000 | 4.532 | 22.06M | 4.960 | 20.16M | 0.944 | 0.21× | 0.19× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.117 | 0.128 | 1.10× |
| 1 | 5 | 0.384 | 0.517 | 1.35× |
| 1 | 10 | 0.646 | 0.929 | 1.44× |
| 10 | 1 | 0.063 | 0.092 | 1.47× |
| 10 | 5 | 0.306 | 0.472 | 1.54× |
| 10 | 10 | 0.663 | 0.951 | 1.43× |
| 100 | 1 | 0.074 | 0.096 | 1.29× |
| 100 | 5 | 0.320 | 0.453 | 1.41× |
| 100 | 10 | 0.622 | 0.947 | 1.52× |
| 1,000 | 1 | 0.115 | 0.102 | 0.88× |
| 1,000 | 5 | 0.320 | 0.489 | 1.53× |
| 1,000 | 10 | 0.677 | 1.036 | 1.53× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
