# MovingAverageConvergenceDivergenceFixed benchmark (`MACDFIX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.048 | 21.00M | 0.039 | 25.97M | 0.045 | 0.95× | 1.18× |
| 10,000 | 0.324 | 30.90M | 0.322 | 31.01M | 0.129 | 0.40× | 0.40× |
| 100,000 | 4.079 | 24.52M | 2.927 | 34.16M | 1.568 | 0.38× | 0.54× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.100 | 0.133 | 1.34× |
| 1 | 5 | 0.490 | 0.504 | 1.03× |
| 1 | 10 | 0.584 | 0.998 | 1.71× |
| 10 | 1 | 0.068 | 0.094 | 1.39× |
| 10 | 5 | 0.273 | 0.470 | 1.72× |
| 10 | 10 | 0.584 | 0.989 | 1.69× |
| 100 | 1 | 0.069 | 0.098 | 1.43× |
| 100 | 5 | 0.286 | 0.466 | 1.63× |
| 100 | 10 | 0.563 | 1.018 | 1.81× |
| 1,000 | 1 | 0.092 | 0.114 | 1.24× |
| 1,000 | 5 | 0.307 | 0.522 | 1.70× |
| 1,000 | 10 | 0.620 | 1.120 | 1.81× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
