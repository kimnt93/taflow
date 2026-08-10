# RollingSkew benchmark (`Skewness` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.036 | 27.64M | 0.036 | 28.05M | 0.185 | 5.12× | 5.19× |
| 10,000 | 0.334 | 29.98M | 0.329 | 30.39M | 0.696 | 2.08× | 2.11× |
| 100,000 | 3.198 | 31.27M | 3.116 | 32.10M | 5.579 | 1.74× | 1.79× |
| 1,000,000 | 31.624 | 31.62M | 31.783 | 31.46M | 55.512 | 1.76× | 1.75× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.087 | 0.319 | 3.67× |
| 1 | 5 | 0.350 | 1.289 | 3.68× |
| 1 | 10 | 0.451 | 8.177 | 18.12× |
| 10 | 1 | 0.051 | 0.211 | 4.13× |
| 10 | 5 | 0.240 | 1.164 | 4.85× |
| 10 | 10 | 0.471 | 2.217 | 4.71× |
| 100 | 1 | 0.054 | 0.220 | 4.12× |
| 100 | 5 | 0.244 | 1.221 | 5.00× |
| 100 | 10 | 0.479 | 2.298 | 4.79× |
| 1,000 | 1 | 0.093 | 0.281 | 3.03× |
| 1,000 | 5 | 0.272 | 1.578 | 5.81× |
| 1,000 | 10 | 0.571 | 3.106 | 5.44× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
