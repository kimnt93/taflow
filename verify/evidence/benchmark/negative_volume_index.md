# NegativeVolumeIndex benchmark (`NVI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 175.31M | 0.004 | 248.11M | 0.190 | 33.23× | 47.03× |
| 10,000 | 0.055 | 182.01M | 0.051 | 197.24M | 0.862 | 15.70× | 17.01× |
| 100,000 | 0.569 | 175.68M | 0.520 | 192.49M | 6.856 | 12.04× | 13.20× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.081 | 0.220 | 2.71× |
| 1 | 5 | 0.287 | 0.999 | 3.48× |
| 1 | 10 | 0.391 | 2.166 | 5.54× |
| 10 | 1 | 0.048 | 0.205 | 4.31× |
| 10 | 5 | 0.201 | 1.294 | 6.44× |
| 10 | 10 | 0.406 | 2.146 | 5.29× |
| 100 | 1 | 0.045 | 0.216 | 4.78× |
| 100 | 5 | 0.188 | 1.308 | 6.97× |
| 100 | 10 | 0.416 | 2.288 | 5.49× |
| 1,000 | 1 | 0.052 | 0.266 | 5.15× |
| 1,000 | 5 | 0.200 | 1.622 | 8.09× |
| 1,000 | 10 | 0.430 | 2.816 | 6.55× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
