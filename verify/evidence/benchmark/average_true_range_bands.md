# AverageTrueRangeBands benchmark (`AtrBands` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.017 | 57.70M | 0.014 | 70.21M | 0.607 | 35.02× | 42.61× |
| 10,000 | 0.106 | 94.31M | 0.098 | 102.24M | 6.612 | 62.36× | 67.60× |
| 100,000 | 1.463 | 68.33M | 0.848 | 117.91M | 44.952 | 30.72× | 53.00× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.115 | 0.327 | 2.84× |
| 1 | 5 | 0.316 | 1.461 | 4.63× |
| 1 | 10 | 0.536 | 2.731 | 5.09× |
| 10 | 1 | 0.071 | 0.302 | 4.25× |
| 10 | 5 | 0.250 | 1.502 | 6.01× |
| 10 | 10 | 0.526 | 2.959 | 5.62× |
| 100 | 1 | 0.056 | 0.314 | 5.61× |
| 100 | 5 | 0.254 | 1.676 | 6.59× |
| 100 | 10 | 0.579 | 3.186 | 5.50× |
| 1,000 | 1 | 0.066 | 0.896 | 13.64× |
| 1,000 | 5 | 0.283 | 3.840 | 13.55× |
| 1,000 | 10 | 0.570 | 7.670 | 13.46× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
