# TrueStrengthIndex benchmark (`TSI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 49.39M | 0.017 | 59.33M | 0.208 | 10.27× | 12.33× |
| 10,000 | 0.144 | 69.63M | 0.148 | 67.58M | 0.614 | 4.27× | 4.15× |
| 100,000 | 1.355 | 73.81M | 1.335 | 74.93M | 5.762 | 4.25× | 4.32× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.089 | 0.290 | 3.26× |
| 1 | 5 | 0.303 | 1.379 | 4.55× |
| 1 | 10 | 0.487 | 2.787 | 5.72× |
| 10 | 1 | 0.054 | 0.246 | 4.53× |
| 10 | 5 | 0.260 | 1.416 | 5.44× |
| 10 | 10 | 0.470 | 2.548 | 5.42× |
| 100 | 1 | 0.054 | 0.249 | 4.65× |
| 100 | 5 | 0.243 | 1.424 | 5.87× |
| 100 | 10 | 0.529 | 2.764 | 5.22× |
| 1,000 | 1 | 0.068 | 0.289 | 4.24× |
| 1,000 | 5 | 0.245 | 1.692 | 6.90× |
| 1,000 | 10 | 0.543 | 3.019 | 5.56× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
