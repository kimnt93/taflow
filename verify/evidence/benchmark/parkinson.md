# Parkinson benchmark (`ParkinsonVolatility` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 49.56M | 0.018 | 56.26M | 0.223 | 11.05× | 12.55× |
| 10,000 | 0.139 | 72.03M | 0.139 | 72.00M | 0.901 | 6.49× | 6.49× |
| 100,000 | 1.409 | 70.99M | 1.312 | 76.23M | 7.799 | 5.54× | 5.95× |
| 1,000,000 | 13.761 | 72.67M | 13.350 | 74.91M | 73.001 | 5.30× | 5.47× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.095 | 0.287 | 3.03× |
| 1 | 5 | 0.285 | 1.355 | 4.75× |
| 1 | 10 | 0.491 | 2.726 | 5.55× |
| 10 | 1 | 0.055 | 0.231 | 4.20× |
| 10 | 5 | 0.232 | 1.458 | 6.28× |
| 10 | 10 | 0.485 | 2.494 | 5.14× |
| 100 | 1 | 0.057 | 0.246 | 4.31× |
| 100 | 5 | 0.238 | 1.486 | 6.25× |
| 100 | 10 | 0.543 | 2.770 | 5.10× |
| 1,000 | 1 | 0.071 | 0.309 | 4.37× |
| 1,000 | 5 | 0.265 | 1.919 | 7.25× |
| 1,000 | 10 | 0.542 | 3.270 | 6.04× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
