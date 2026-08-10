# ZigZag benchmark (`ZigZag` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 97.81M | 0.011 | 91.12M | 0.495 | 48.44× | 45.13× |
| 10,000 | 0.080 | 124.33M | 0.076 | 131.95M | 3.370 | 41.90× | 44.47× |
| 100,000 | 0.745 | 134.25M | 0.701 | 142.62M | 36.780 | 49.38× | 52.45× |
| 1,000,000 | 7.946 | 125.85M | 6.937 | 144.16M | 410.691 | 51.68× | 59.20× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.111 | 0.274 | 2.47× |
| 1 | 5 | 0.347 | 1.195 | 3.45× |
| 1 | 10 | 0.495 | 2.341 | 4.73× |
| 10 | 1 | 0.068 | 0.235 | 3.47× |
| 10 | 5 | 0.254 | 1.276 | 5.04× |
| 10 | 10 | 0.478 | 2.495 | 5.22× |
| 100 | 1 | 0.053 | 0.262 | 4.91× |
| 100 | 5 | 0.288 | 1.467 | 5.09× |
| 100 | 10 | 0.533 | 2.894 | 5.43× |
| 1,000 | 1 | 0.062 | 0.667 | 10.70× |
| 1,000 | 5 | 0.249 | 3.134 | 12.61× |
| 1,000 | 10 | 0.545 | 6.262 | 11.48× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
