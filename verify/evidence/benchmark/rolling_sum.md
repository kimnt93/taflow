# RollingSum benchmark (`SUM` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 145.38M | 0.006 | 154.64M | 0.040 | 5.88× | 6.26× |
| 10,000 | 0.042 | 239.44M | 0.038 | 265.00M | 0.062 | 1.48× | 1.63× |
| 100,000 | 0.419 | 238.40M | 0.354 | 282.88M | 0.268 | 0.64× | 0.76× |
| 1,000,000 | 4.226 | 236.62M | 3.587 | 278.80M | 2.269 | 0.54× | 0.63× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.082 | 0.136 | 1.67× |
| 1 | 5 | 0.305 | 0.485 | 1.59× |
| 1 | 10 | 0.504 | 1.084 | 2.15× |
| 10 | 1 | 0.067 | 0.110 | 1.65× |
| 10 | 5 | 0.319 | 0.548 | 1.72× |
| 10 | 10 | 0.560 | 1.161 | 2.07× |
| 100 | 1 | 0.069 | 0.138 | 2.00× |
| 100 | 5 | 0.333 | 0.584 | 1.75× |
| 100 | 10 | 0.626 | 1.144 | 1.83× |
| 1,000 | 1 | 0.094 | 0.114 | 1.21× |
| 1,000 | 5 | 0.310 | 0.538 | 1.73× |
| 1,000 | 10 | 0.563 | 1.003 | 1.78× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
