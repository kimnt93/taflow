# RollingVarianceRatio benchmark (`VarianceRatio` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.171 | 5.86M | 0.172 | 5.82M | 0.358 | 2.10× | 2.08× |
| 10,000 | 1.723 | 5.80M | 1.960 | 5.10M | 2.430 | 1.41× | 1.24× |
| 100,000 | 17.557 | 5.70M | 17.993 | 5.56M | 21.648 | 1.23× | 1.20× |
| 1,000,000 | 179.596 | 5.57M | 190.598 | 5.25M | 231.309 | 1.29× | 1.21× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.124 | 0.541 | 4.35× |
| 1 | 5 | 0.309 | 1.546 | 5.01× |
| 1 | 10 | 0.721 | 3.659 | 5.08× |
| 10 | 1 | 0.057 | 0.263 | 4.65× |
| 10 | 5 | 0.282 | 1.607 | 5.70× |
| 10 | 10 | 0.597 | 3.337 | 5.59× |
| 100 | 1 | 0.096 | 0.357 | 3.73× |
| 100 | 5 | 0.296 | 1.940 | 6.56× |
| 100 | 10 | 0.631 | 3.553 | 5.63× |
| 1,000 | 1 | 0.261 | 0.520 | 1.99× |
| 1,000 | 5 | 0.483 | 3.803 | 7.87× |
| 1,000 | 10 | 1.245 | 6.154 | 4.95× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
