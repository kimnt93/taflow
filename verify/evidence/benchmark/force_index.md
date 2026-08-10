# ForceIndex benchmark (`ForceIndex` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 101.71M | 0.008 | 131.56M | 0.197 | 20.04× | 25.92× |
| 10,000 | 0.049 | 202.63M | 0.048 | 209.92M | 0.765 | 15.50× | 16.06× |
| 100,000 | 0.461 | 217.11M | 0.431 | 232.25M | 6.511 | 14.14× | 15.12× |
| 1,000,000 | 4.912 | 203.58M | 4.715 | 212.07M | 64.693 | 13.17× | 13.72× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.116 | 0.278 | 2.39× |
| 1 | 5 | 0.258 | 1.611 | 6.24× |
| 1 | 10 | 0.502 | 2.432 | 4.85× |
| 10 | 1 | 0.053 | 0.219 | 4.14× |
| 10 | 5 | 0.265 | 1.544 | 5.83× |
| 10 | 10 | 0.470 | 2.347 | 4.99× |
| 100 | 1 | 0.053 | 0.223 | 4.25× |
| 100 | 5 | 0.229 | 1.417 | 6.18× |
| 100 | 10 | 0.498 | 2.473 | 4.97× |
| 1,000 | 1 | 0.068 | 0.284 | 4.17× |
| 1,000 | 5 | 0.249 | 1.700 | 6.82× |
| 1,000 | 10 | 0.516 | 3.023 | 5.86× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
