# RollingLinearRegressionSlope benchmark (`LINEARREG_SLOPE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.016 | 64.03M | 0.016 | 63.78M | 0.044 | 2.79× | 2.78× |
| 10,000 | 0.117 | 85.35M | 0.114 | 87.95M | 0.157 | 1.34× | 1.38× |
| 100,000 | 1.355 | 73.82M | 1.319 | 75.82M | 1.334 | 0.98× | 1.01× |
| 1,000,000 | 13.668 | 73.16M | 12.940 | 77.28M | 12.415 | 0.91× | 0.96× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.089 | 0.131 | 1.46× |
| 1 | 5 | 0.350 | 0.492 | 1.41× |
| 1 | 10 | 0.581 | 0.980 | 1.69× |
| 10 | 1 | 0.047 | 0.089 | 1.90× |
| 10 | 5 | 0.239 | 0.494 | 2.07× |
| 10 | 10 | 0.548 | 0.990 | 1.81× |
| 100 | 1 | 0.054 | 0.101 | 1.85× |
| 100 | 5 | 0.224 | 0.470 | 2.10× |
| 100 | 10 | 0.548 | 1.095 | 2.00× |
| 1,000 | 1 | 0.070 | 0.110 | 1.56× |
| 1,000 | 5 | 0.264 | 0.508 | 1.92× |
| 1,000 | 10 | 0.519 | 1.146 | 2.21× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
