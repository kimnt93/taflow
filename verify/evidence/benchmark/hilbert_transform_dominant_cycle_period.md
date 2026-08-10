# HilbertTransformDominantCyclePeriod benchmark (`HT_DCPERIOD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.049 | 20.30M | 0.049 | 20.47M | 0.080 | 1.62× | 1.64× |
| 10,000 | 0.479 | 20.90M | 0.468 | 21.37M | 0.489 | 1.02× | 1.05× |
| 100,000 | 4.601 | 21.74M | 4.782 | 20.91M | 4.619 | 1.00× | 0.97× |
| 1,000,000 | 46.552 | 21.48M | 48.408 | 20.66M | 46.511 | 1.00× | 0.96× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.104 | 0.177 | 1.71× |
| 1 | 5 | 0.292 | 0.462 | 1.58× |
| 1 | 10 | 0.499 | 0.898 | 1.80× |
| 10 | 1 | 0.048 | 0.095 | 1.99× |
| 10 | 5 | 0.266 | 0.531 | 1.99× |
| 10 | 10 | 0.465 | 0.915 | 1.97× |
| 100 | 1 | 0.058 | 0.094 | 1.61× |
| 100 | 5 | 0.264 | 0.521 | 1.97× |
| 100 | 10 | 0.553 | 0.997 | 1.80× |
| 1,000 | 1 | 0.102 | 0.138 | 1.35× |
| 1,000 | 5 | 0.231 | 0.672 | 2.91× |
| 1,000 | 10 | 0.561 | 1.425 | 2.54× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
