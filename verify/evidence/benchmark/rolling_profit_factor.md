# RollingProfitFactor benchmark (`ProfitFactor` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.022 | 45.17M | 0.021 | 47.20M | 0.172 | 7.75× | 8.10× |
| 10,000 | 0.206 | 48.45M | 0.208 | 48.10M | 0.691 | 3.35× | 3.32× |
| 100,000 | 2.204 | 45.38M | 2.117 | 47.24M | 5.666 | 2.57× | 2.68× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.067 | 0.305 | 4.52× |
| 1 | 5 | 0.307 | 0.991 | 3.23× |
| 1 | 10 | 0.499 | 2.412 | 4.83× |
| 10 | 1 | 0.046 | 0.194 | 4.21× |
| 10 | 5 | 0.229 | 0.989 | 4.33× |
| 10 | 10 | 0.430 | 2.487 | 5.79× |
| 100 | 1 | 0.051 | 0.197 | 3.85× |
| 100 | 5 | 0.200 | 0.951 | 4.75× |
| 100 | 10 | 0.480 | 2.195 | 4.58× |
| 1,000 | 1 | 0.068 | 0.244 | 3.60× |
| 1,000 | 5 | 0.220 | 1.351 | 6.14× |
| 1,000 | 10 | 0.462 | 2.664 | 5.77× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
