# CandleMarubozu benchmark (`CDLMARUBOZU` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 99.98M | 0.010 | 95.82M | 0.037 | 3.73× | 3.57× |
| 10,000 | 0.099 | 100.63M | 0.097 | 102.77M | 0.136 | 1.37× | 1.40× |
| 100,000 | 1.085 | 92.15M | 0.956 | 104.59M | 1.029 | 0.95× | 1.08× |
| 1,000,000 | 10.446 | 95.73M | 10.153 | 98.50M | 10.130 | 0.97× | 1.00× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.105 | 0.116 | 1.10× |
| 1 | 5 | 0.349 | 0.480 | 1.38× |
| 1 | 10 | 0.526 | 0.925 | 1.76× |
| 10 | 1 | 0.050 | 0.090 | 1.79× |
| 10 | 5 | 0.245 | 0.430 | 1.75× |
| 10 | 10 | 0.511 | 0.921 | 1.80× |
| 100 | 1 | 0.055 | 0.090 | 1.66× |
| 100 | 5 | 0.247 | 0.446 | 1.80× |
| 100 | 10 | 0.537 | 0.920 | 1.71× |
| 1,000 | 1 | 0.069 | 0.102 | 1.48× |
| 1,000 | 5 | 0.252 | 0.501 | 1.99× |
| 1,000 | 10 | 0.558 | 1.057 | 1.90× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
