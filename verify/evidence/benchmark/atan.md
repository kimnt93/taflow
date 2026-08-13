# MathAtan benchmark (`ATAN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.029 | 34.82M | 0.024 | 42.15M | 0.032 | 1.11× | 1.35× |
| 10,000 | 0.189 | 53.02M | 0.181 | 55.37M | 0.090 | 0.48× | 0.50× |
| 100,000 | 1.780 | 56.18M | 1.760 | 56.80M | 0.603 | 0.34× | 0.34× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.114 | 0.119 | 1.05× |
| 1 | 5 | 0.426 | 0.469 | 1.10× |
| 1 | 10 | 0.597 | 0.899 | 1.51× |
| 10 | 1 | 0.066 | 0.094 | 1.42× |
| 10 | 5 | 0.282 | 0.419 | 1.49× |
| 10 | 10 | 0.578 | 0.857 | 1.48× |
| 100 | 1 | 0.064 | 0.086 | 1.34× |
| 100 | 5 | 0.294 | 0.413 | 1.41× |
| 100 | 10 | 0.606 | 0.892 | 1.47× |
| 1,000 | 1 | 0.079 | 0.093 | 1.19× |
| 1,000 | 5 | 0.306 | 0.440 | 1.44× |
| 1,000 | 10 | 0.602 | 0.933 | 1.55× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
