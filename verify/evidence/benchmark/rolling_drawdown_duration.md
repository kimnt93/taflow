# RollingDrawdownDuration benchmark (`DrawdownDuration` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 190.65M | 0.005 | 220.42M | 0.116 | 22.02× | 25.46× |
| 10,000 | 0.026 | 388.54M | 0.024 | 418.57M | 0.421 | 16.34× | 17.60× |
| 100,000 | 0.236 | 424.20M | 0.212 | 471.52M | 3.396 | 14.41× | 16.01× |
| 1,000,000 | 2.564 | 389.98M | 2.161 | 462.67M | 40.237 | 15.69× | 18.62× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.080 | 0.209 | 2.62× |
| 1 | 5 | 0.298 | 0.789 | 2.65× |
| 1 | 10 | 0.458 | 1.569 | 3.43× |
| 10 | 1 | 0.048 | 0.161 | 3.37× |
| 10 | 5 | 0.239 | 1.012 | 4.24× |
| 10 | 10 | 0.455 | 1.572 | 3.45× |
| 100 | 1 | 0.050 | 0.163 | 3.24× |
| 100 | 5 | 0.237 | 1.046 | 4.41× |
| 100 | 10 | 0.467 | 1.631 | 3.49× |
| 1,000 | 1 | 0.050 | 0.193 | 3.89× |
| 1,000 | 5 | 0.265 | 1.201 | 4.53× |
| 1,000 | 10 | 0.478 | 1.943 | 4.06× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
