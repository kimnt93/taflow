# WeightedClose benchmark (`WCLPRICE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.003 | 337.07M | 0.001 | 784.86M | 0.030 | 10.09× | 23.50× |
| 10,000 | 0.009 | 1.09G | 0.006 | 1.80G | 0.035 | 3.78× | 6.25× |
| 100,000 | 0.074 | 1.34G | 0.051 | 1.96G | 0.081 | 1.09× | 1.59× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.058 | 0.137 | 2.37× |
| 1 | 5 | 0.217 | 0.454 | 2.09× |
| 1 | 10 | 0.484 | 0.900 | 1.86× |
| 10 | 1 | 0.050 | 0.090 | 1.81× |
| 10 | 5 | 0.213 | 0.427 | 2.01× |
| 10 | 10 | 0.377 | 0.977 | 2.59× |
| 100 | 1 | 0.046 | 0.085 | 1.86× |
| 100 | 5 | 0.189 | 0.461 | 2.44× |
| 100 | 10 | 0.380 | 0.948 | 2.49× |
| 1,000 | 1 | 0.048 | 0.102 | 2.13× |
| 1,000 | 5 | 0.203 | 0.465 | 2.29× |
| 1,000 | 10 | 0.403 | 0.935 | 2.32× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
