# ExponentiallyWeightedCorrelation benchmark (`ewm correlation` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.044 | 22.66M | 0.037 | 27.26M | 1.261 | 28.58× | 34.37× |
| 10,000 | 0.298 | 33.56M | 0.293 | 34.15M | 12.364 | 41.50× | 42.22× |
| 100,000 | 2.684 | 37.25M | 2.696 | 37.09M | 124.275 | 46.29× | 46.09× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.121 | 0.147 | 1.22× |
| 1 | 5 | 0.436 | 0.490 | 1.12× |
| 1 | 10 | 0.612 | 1.004 | 1.64× |
| 10 | 1 | 0.079 | 0.128 | 1.62× |
| 10 | 5 | 0.318 | 0.572 | 1.80× |
| 10 | 10 | 0.687 | 1.170 | 1.70× |
| 100 | 1 | 0.077 | 0.230 | 3.00× |
| 100 | 5 | 0.300 | 1.154 | 3.85× |
| 100 | 10 | 0.654 | 2.241 | 3.42× |
| 1,000 | 1 | 0.100 | 1.362 | 13.63× |
| 1,000 | 5 | 0.305 | 6.847 | 22.48× |
| 1,000 | 10 | 0.627 | 13.977 | 22.27× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
