# RollingProfitFactor benchmark (`ProfitFactor` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.169 | 5.91M | 0.159 | 6.27M | 0.159 | 0.94× | 1.00× |
| 10,000 | 1.733 | 5.77M | 1.599 | 6.25M | 0.601 | 0.35× | 0.38× |
| 100,000 | 15.166 | 6.59M | 15.322 | 6.53M | 4.878 | 0.32× | 0.32× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.121 | 0.231 | 1.92× |
| 1 | 5 | 0.463 | 0.985 | 2.13× |
| 1 | 10 | 0.595 | 2.128 | 3.58× |
| 10 | 1 | 0.071 | 0.202 | 2.83× |
| 10 | 5 | 0.301 | 0.950 | 3.15× |
| 10 | 10 | 0.608 | 2.109 | 3.47× |
| 100 | 1 | 0.095 | 0.203 | 2.13× |
| 100 | 5 | 0.282 | 0.938 | 3.32× |
| 100 | 10 | 0.633 | 2.114 | 3.34× |
| 1,000 | 1 | 0.237 | 0.245 | 1.03× |
| 1,000 | 5 | 0.417 | 1.182 | 2.83× |
| 1,000 | 10 | 0.738 | 2.591 | 3.51× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
