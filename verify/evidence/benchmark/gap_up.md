# GapUp benchmark (`gap up relation` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 125.05M | 0.007 | 152.67M | 0.035 | 4.42× | 5.40× |
| 10,000 | 0.034 | 291.56M | 0.031 | 324.54M | 0.045 | 1.31× | 1.46× |
| 100,000 | 0.369 | 271.29M | 0.278 | 359.20M | 0.258 | 0.70× | 0.93× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.083 | 0.095 | 1.15× |
| 1 | 5 | 0.269 | 0.377 | 1.40× |
| 1 | 10 | 0.511 | 0.747 | 1.46× |
| 10 | 1 | 0.052 | 0.070 | 1.35× |
| 10 | 5 | 0.234 | 0.369 | 1.57× |
| 10 | 10 | 0.524 | 0.803 | 1.53× |
| 100 | 1 | 0.055 | 0.071 | 1.28× |
| 100 | 5 | 0.246 | 0.382 | 1.55× |
| 100 | 10 | 0.475 | 0.831 | 1.75× |
| 1,000 | 1 | 0.055 | 0.088 | 1.59× |
| 1,000 | 5 | 0.228 | 0.570 | 2.50× |
| 1,000 | 10 | 0.571 | 1.258 | 2.20× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
