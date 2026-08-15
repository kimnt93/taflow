# RollingOmegaRatio benchmark (`OmegaRatio` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.027 | 36.44M | 0.045 | 22.06M | 0.220 | 8.02× | 4.85× |
| 10,000 | 0.266 | 37.63M | 0.264 | 37.92M | 0.690 | 2.60× | 2.62× |
| 100,000 | 2.649 | 37.75M | 2.540 | 39.37M | 5.501 | 2.08× | 2.17× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.145 | 0.308 | 2.13× |
| 1 | 5 | 0.212 | 1.163 | 5.48× |
| 1 | 10 | 0.404 | 2.522 | 6.24× |
| 10 | 1 | 0.046 | 0.239 | 5.25× |
| 10 | 5 | 0.205 | 1.339 | 6.52× |
| 10 | 10 | 0.435 | 2.517 | 5.78× |
| 100 | 1 | 0.049 | 0.230 | 4.66× |
| 100 | 5 | 0.190 | 1.383 | 7.29× |
| 100 | 10 | 0.466 | 2.636 | 5.66× |
| 1,000 | 1 | 0.077 | 0.294 | 3.83× |
| 1,000 | 5 | 0.251 | 1.728 | 6.89× |
| 1,000 | 10 | 0.446 | 3.275 | 7.34× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
