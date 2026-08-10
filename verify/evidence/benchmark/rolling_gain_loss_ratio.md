# RollingGainLossRatio benchmark (`GainLossRatio` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.027 | 37.12M | 0.027 | 36.78M | 0.151 | 5.62× | 5.57× |
| 10,000 | 0.241 | 41.53M | 0.232 | 43.17M | 0.549 | 2.28× | 2.37× |
| 100,000 | 2.300 | 43.48M | 2.391 | 41.83M | 4.538 | 1.97× | 1.90× |
| 1,000,000 | 24.550 | 40.73M | 23.306 | 42.91M | 43.827 | 1.79× | 1.88× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.141 | 0.277 | 1.97× |
| 1 | 5 | 0.256 | 1.027 | 4.01× |
| 1 | 10 | 0.462 | 2.070 | 4.48× |
| 10 | 1 | 0.053 | 0.198 | 3.71× |
| 10 | 5 | 0.236 | 0.944 | 4.00× |
| 10 | 10 | 0.477 | 2.116 | 4.43× |
| 100 | 1 | 0.055 | 0.183 | 3.32× |
| 100 | 5 | 0.230 | 0.971 | 4.23× |
| 100 | 10 | 0.483 | 2.143 | 4.43× |
| 1,000 | 1 | 0.077 | 0.230 | 2.96× |
| 1,000 | 5 | 0.273 | 1.252 | 4.58× |
| 1,000 | 10 | 0.513 | 2.559 | 4.98× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
