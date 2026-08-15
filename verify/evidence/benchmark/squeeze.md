# Squeeze benchmark (`squeeze` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.030 | 33.06M | 0.030 | 33.57M | 4.726 | 156.26× | 158.68× |
| 10,000 | 0.258 | 38.83M | 0.263 | 38.09M | 6.636 | 25.77× | 25.28× |
| 100,000 | 2.708 | 36.93M | 2.689 | 37.19M | 27.937 | 10.32× | 10.39× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.162 | 0.310 | 1.91× |
| 1 | 5 | 0.462 | 1.542 | 3.34× |
| 1 | 10 | 0.453 | 3.143 | 6.94× |
| 10 | 1 | 0.049 | 0.306 | 6.27× |
| 10 | 5 | 0.206 | 1.575 | 7.64× |
| 10 | 10 | 0.419 | 3.011 | 7.18× |
| 100 | 1 | 0.055 | 4.868 | 87.82× |
| 100 | 5 | 0.231 | 24.540 | 106.04× |
| 100 | 10 | 0.452 | 48.649 | 107.71× |
| 1,000 | 1 | 0.079 | 4.899 | 62.05× |
| 1,000 | 5 | 0.289 | 26.304 | 91.05× |
| 1,000 | 10 | 0.450 | 53.424 | 118.68× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
