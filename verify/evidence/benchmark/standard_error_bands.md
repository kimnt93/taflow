# StandardErrorBands benchmark (`StandardErrorBands` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.070 | 14.22M | 0.069 | 14.52M | 0.595 | 8.46× | 8.64× |
| 10,000 | 0.696 | 14.37M | 0.687 | 14.56M | 4.441 | 6.38× | 6.47× |
| 100,000 | 7.438 | 13.44M | 6.923 | 14.45M | 44.204 | 5.94× | 6.39× |
| 1,000,000 | 69.736 | 14.34M | 68.410 | 14.62M | 502.791 | 7.21× | 7.35× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.106 | 0.331 | 3.11× |
| 1 | 5 | 0.272 | 1.334 | 4.90× |
| 1 | 10 | 0.483 | 2.678 | 5.54× |
| 10 | 1 | 0.049 | 0.249 | 5.04× |
| 10 | 5 | 0.234 | 1.447 | 6.18× |
| 10 | 10 | 0.514 | 2.877 | 5.59× |
| 100 | 1 | 0.060 | 0.304 | 5.08× |
| 100 | 5 | 0.961 | 1.834 | 1.91× |
| 100 | 10 | 0.552 | 3.349 | 6.07× |
| 1,000 | 1 | 0.127 | 0.873 | 6.88× |
| 1,000 | 5 | 0.315 | 3.862 | 12.24× |
| 1,000 | 10 | 0.588 | 15.094 | 25.65× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
