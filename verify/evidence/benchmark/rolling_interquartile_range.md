# RollingInterquartileRange benchmark (`RollingIqr` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.070 | 14.27M | 0.066 | 15.20M | 0.301 | 4.30× | 4.57× |
| 10,000 | 0.688 | 14.53M | 0.661 | 15.12M | 2.072 | 3.01× | 3.13× |
| 100,000 | 7.065 | 14.15M | 7.092 | 14.10M | 15.821 | 2.24× | 2.23× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.065 | 0.254 | 3.89× |
| 1 | 5 | 0.316 | 1.115 | 3.52× |
| 1 | 10 | 0.455 | 2.659 | 5.85× |
| 10 | 1 | 0.055 | 0.222 | 4.03× |
| 10 | 5 | 0.234 | 1.073 | 4.60× |
| 10 | 10 | 0.513 | 2.325 | 4.53× |
| 100 | 1 | 0.057 | 0.233 | 4.10× |
| 100 | 5 | 0.255 | 1.488 | 5.83× |
| 100 | 10 | 0.470 | 2.596 | 5.52× |
| 1,000 | 1 | 0.133 | 0.398 | 3.01× |
| 1,000 | 5 | 0.304 | 2.262 | 7.44× |
| 1,000 | 10 | 0.539 | 4.130 | 7.67× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
