# WilliamsAccumulationDistribution benchmark (`Wad` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 68.56M | 0.009 | 116.42M | 0.194 | 13.32× | 22.61× |
| 10,000 | 0.074 | 134.40M | 0.070 | 143.48M | 1.097 | 14.74× | 15.74× |
| 100,000 | 0.678 | 147.50M | 0.649 | 154.05M | 10.009 | 14.76× | 15.42× |
| 1,000,000 | 7.232 | 138.28M | 6.607 | 151.35M | 118.329 | 16.36× | 17.91× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.099 | 0.218 | 2.21× |
| 1 | 5 | 0.359 | 0.838 | 2.33× |
| 1 | 10 | 0.507 | 1.942 | 3.83× |
| 10 | 1 | 0.062 | 0.184 | 2.97× |
| 10 | 5 | 0.247 | 0.824 | 3.33× |
| 10 | 10 | 0.532 | 2.025 | 3.81× |
| 100 | 1 | 0.064 | 0.180 | 2.81× |
| 100 | 5 | 0.262 | 0.900 | 3.44× |
| 100 | 10 | 0.587 | 1.989 | 3.39× |
| 1,000 | 1 | 0.068 | 0.274 | 4.04× |
| 1,000 | 5 | 0.255 | 1.591 | 6.23× |
| 1,000 | 10 | 0.630 | 2.825 | 4.48× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
