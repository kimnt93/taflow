# RollingInformationRatio benchmark (`InformationRatio` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.355 | 2.82M | 0.357 | 2.80M | 0.190 | 0.54× | 0.53× |
| 10,000 | 3.577 | 2.80M | 3.461 | 2.89M | 0.799 | 0.22× | 0.23× |
| 100,000 | 33.987 | 2.94M | 33.716 | 2.97M | 6.869 | 0.20× | 0.20× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.227 | 0.231 | 1.02× |
| 1 | 5 | 0.471 | 1.052 | 2.23× |
| 1 | 10 | 0.644 | 2.306 | 3.58× |
| 10 | 1 | 0.073 | 0.195 | 2.66× |
| 10 | 5 | 0.307 | 0.977 | 3.19× |
| 10 | 10 | 0.636 | 2.366 | 3.72× |
| 100 | 1 | 0.107 | 0.202 | 1.88× |
| 100 | 5 | 0.310 | 1.020 | 3.30× |
| 100 | 10 | 0.617 | 2.274 | 3.69× |
| 1,000 | 1 | 0.424 | 0.278 | 0.66× |
| 1,000 | 5 | 0.778 | 1.353 | 1.74× |
| 1,000 | 10 | 1.090 | 3.032 | 2.78× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
