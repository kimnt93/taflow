# RollingAutocorr benchmark (`Autocorrelation` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.062 | 16.25M | 0.059 | 16.85M | 0.308 | 5.01× | 5.19× |
| 10,000 | 0.636 | 15.73M | 0.596 | 16.77M | 1.203 | 1.89× | 2.02× |
| 100,000 | 6.009 | 16.64M | 5.970 | 16.75M | 9.714 | 1.62× | 1.63× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.071 | 0.294 | 4.12× |
| 1 | 5 | 0.313 | 1.417 | 4.53× |
| 1 | 10 | 0.475 | 2.852 | 6.01× |
| 10 | 1 | 0.054 | 0.263 | 4.88× |
| 10 | 5 | 0.197 | 1.611 | 8.18× |
| 10 | 10 | 0.439 | 2.479 | 5.65× |
| 100 | 1 | 0.054 | 0.275 | 5.09× |
| 100 | 5 | 0.200 | 1.458 | 7.28× |
| 100 | 10 | 0.441 | 2.892 | 6.56× |
| 1,000 | 1 | 0.112 | 0.333 | 2.97× |
| 1,000 | 5 | 0.236 | 2.009 | 8.52× |
| 1,000 | 10 | 0.517 | 3.509 | 6.79× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
