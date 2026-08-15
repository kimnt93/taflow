# ZigZag benchmark (`ZigZag` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 115.77M | 0.007 | 140.95M | 0.486 | 56.24× | 68.47× |
| 10,000 | 0.082 | 121.39M | 0.074 | 134.77M | 3.473 | 42.15× | 46.80× |
| 100,000 | 0.776 | 128.81M | 0.760 | 131.64M | 37.693 | 48.55× | 49.62× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.093 | 0.310 | 3.32× |
| 1 | 5 | 0.214 | 1.071 | 5.01× |
| 1 | 10 | 0.384 | 2.310 | 6.02× |
| 10 | 1 | 0.048 | 0.221 | 4.59× |
| 10 | 5 | 0.197 | 1.244 | 6.33× |
| 10 | 10 | 0.404 | 2.426 | 6.01× |
| 100 | 1 | 0.048 | 0.267 | 5.59× |
| 100 | 5 | 0.202 | 1.494 | 7.40× |
| 100 | 10 | 0.437 | 2.849 | 6.51× |
| 1,000 | 1 | 0.053 | 0.673 | 12.64× |
| 1,000 | 5 | 0.205 | 3.216 | 15.69× |
| 1,000 | 10 | 0.449 | 6.534 | 14.54× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
