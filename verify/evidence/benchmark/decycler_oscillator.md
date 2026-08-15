# DecyclerOscillator benchmark (`DecyclerOscillator` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 119.56M | 0.007 | 140.38M | 0.176 | 21.06× | 24.72× |
| 10,000 | 0.065 | 153.54M | 0.063 | 158.75M | 0.519 | 7.97× | 8.24× |
| 100,000 | 0.625 | 160.05M | 0.596 | 167.86M | 3.870 | 6.19× | 6.50× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.063 | 0.276 | 4.38× |
| 1 | 5 | 0.285 | 1.091 | 3.83× |
| 1 | 10 | 0.407 | 2.274 | 5.58× |
| 10 | 1 | 0.045 | 0.212 | 4.70× |
| 10 | 5 | 0.204 | 1.044 | 5.11× |
| 10 | 10 | 0.397 | 2.345 | 5.91× |
| 100 | 1 | 0.046 | 0.217 | 4.76× |
| 100 | 5 | 0.193 | 1.048 | 5.42× |
| 100 | 10 | 0.436 | 2.368 | 5.44× |
| 1,000 | 1 | 0.052 | 0.244 | 4.73× |
| 1,000 | 5 | 0.204 | 1.248 | 6.13× |
| 1,000 | 10 | 0.438 | 2.768 | 6.32× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
