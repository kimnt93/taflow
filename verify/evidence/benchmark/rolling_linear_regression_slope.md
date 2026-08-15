# RollingLinearRegressionSlope benchmark (`LINEARREG_SLOPE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 81.27M | 0.011 | 88.59M | 0.040 | 3.25× | 3.55× |
| 10,000 | 0.113 | 88.20M | 0.106 | 94.40M | 0.136 | 1.20× | 1.29× |
| 100,000 | 1.051 | 95.19M | 1.052 | 95.05M | 1.033 | 0.98× | 0.98× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.058 | 0.102 | 1.77× |
| 1 | 5 | 0.372 | 0.464 | 1.25× |
| 1 | 10 | 0.423 | 0.922 | 2.18× |
| 10 | 1 | 0.042 | 0.086 | 2.08× |
| 10 | 5 | 0.182 | 0.415 | 2.28× |
| 10 | 10 | 0.383 | 0.943 | 2.46× |
| 100 | 1 | 0.046 | 0.086 | 1.89× |
| 100 | 5 | 0.198 | 0.433 | 2.19× |
| 100 | 10 | 0.407 | 0.938 | 2.30× |
| 1,000 | 1 | 0.061 | 0.098 | 1.61× |
| 1,000 | 5 | 0.217 | 0.525 | 2.42× |
| 1,000 | 10 | 0.415 | 1.013 | 2.44× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
